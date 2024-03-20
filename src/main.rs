use minigrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1); // exit with error code 1
    });

    println!("{:?}", args);

    println!("In file {}", config.file_path);
    println!("Searching for {}", config.query);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        std::process::exit(1);
    }

    // run(config).unwrap_or_else(|err| {
    //     println!("Application error: {err}");
    //     std::process::exit(1); // exit with error code 1
    // })
}
