use crate::fs::find_ts_files;
use crate::types::ParsedModule;
// use debug::log_node;
use std::path::Path;
use std::fs;
use std::sync::Arc;
use swc_common::{SourceMap, FileName, sync::Lrc};
use swc_ecma_parser::{Parser, StringInput, Syntax, TsSyntax};

pub fn parse_dir(entry: &Path) -> Vec<ParsedModule> {
    let cm: Arc<SourceMap> = Default::default(); // Shared SourceMap
    let ts_files = find_ts_files(entry);
    let mut parsed_modules: Vec<ParsedModule> = Vec::new();
    for path in ts_files {
        println!("Parsing: {}", path.display());

        let source_code = match fs::read_to_string(&path) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Failed to read {}: {}", path.display(), e);
                continue;
            }
        };

        let fm = cm.new_source_file(Lrc::new(FileName::Real(path.to_path_buf())), source_code);

        let mut parser = Parser::new(
            Syntax::Typescript(TsSyntax {
                tsx: path.extension().and_then(|ext| ext.to_str()) == Some("tsx"),
                ..Default::default()
            }),
            StringInput::from(&*fm),
            None,
        );

        match parser.parse_module() {
            Ok(module) => {
                println!("✅ Parsed {} successfully", fm.name);
                // log_node("RootModule", &cm, &module, 0); // Optional: node-level output
                // You can now pass `module` to analysis
                parsed_modules.push(ParsedModule {
                    path: path.to_path_buf(),
                    module,
                    source_map: cm.clone(),
                });
            }
            Err(err) => {
                eprintln!("❌ Failed to parse: {:?}", err);
            }
        }
    }
    parsed_modules
}
