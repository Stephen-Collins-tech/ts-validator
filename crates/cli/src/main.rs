use analysis::do_something as analysis_do_something;
use error::do_something as error_do_something;
use heuristics::do_something as heuristics_do_something;
use parser::do_something as parser_do_something;
use reporting::do_something as reporting_do_something;


fn main() {
    println!("Starting cli!");
    parser_do_something();
    analysis_do_something();
    heuristics_do_something();
    reporting_do_something();
    error_do_something();
}
