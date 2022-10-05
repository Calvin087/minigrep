use std::env; // env variables passed in.
use std::fs;
use std::process; // exits program without panicing.
use std::error::Error;
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
        println!("Problem parsing arguments: {}", err);
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
    if let Err(e) = run(config) {
        println!("Application error {}", e);
        process::exit(1);
    };

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /* 
        () returns nothing on Success
        Box dyn allows us to return any type of error.
        Not exactly but will learn more later.
    */
    let contents = fs::read_to_string(config.filename)?;
    /*
        ? Allows us to automatically return an error type.
        However when calling main, we're not handling
        the Error type yet
    */
    println!("{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        /* 
            by returning a Result instead, we can return 
            a config struct OR an ERR with a string.
        */

        if args.len() < 3 {
            return Err("not enough args were passed in by user");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        /*
            Config expects strings but we were passing refs
            To fix that we remove & from args[]
            We don't want to claim ownership, so we clone as well.
            Not efficient because we're creating copies.
            But it's simple.
        */

        Ok(Config{query, filename})
    }
}
