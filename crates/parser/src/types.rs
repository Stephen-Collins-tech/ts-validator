use swc_common::SourceMap;
use swc_ecma_ast::Module;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ParsedModule {
    pub path: PathBuf,
    pub module: Module,
    pub source_map: Arc<SourceMap>,
}
