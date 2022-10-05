use std::env; // env variables passed in.
use std::process; // exits program without panicing.

use minigrep::Config; // name of library crate...how?
fn main() {
    let args: Vec<String> = env::args().collect();

    /* 
        Calling the config method new, that creates a new struct
        passing in the args from the user env
        Config method checks we have enough args and returns result type with ok or err
        unwrap gives us the Ok(value) or Err(value), we handle the error with a closure
        process exit stops program without panicing with status code.
    */

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    /*
        Searching for needle
        In file haystack
    */

    /* 
        Because run returns a Result type
        We have to handle the error here when calling
        the run function.
    */
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    };

}
