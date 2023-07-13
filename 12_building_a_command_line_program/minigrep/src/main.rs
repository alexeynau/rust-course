use std::env;
use std::process;
// connetcting to lib
use minigrep::Config;

fn main() {
    // get args from console
    let args: Vec<String> = env::args().collect();

    // set configuration
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // check if there is error in running program
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
