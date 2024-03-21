use minigrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1); // exit with error code 1
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }

    // run(config).unwrap_or_else(|err| {
    //     println!("Application error: {err}");
    //     std::process::exit(1); // exit with error code 1
    // })
}
