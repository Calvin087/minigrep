use std::error::Error;
use std::fs;
use std::env;
/*
    Everything private by default
    Need to make everything public
*/

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("** Line found: ** \n{}", line)
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
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

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        /* 
            This only really works when doing `export CASE_INSENSITIVE` in terminal.
        */

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
    // vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "poised";

        /*
            Turns out these lines need to be against the edge.
        */
        let contents = "\
In the twenty-first century,
no pop star was as poised,
as polished, or as generally fierce as Beyoncé. She scored";

        assert_eq!(vec!["no pop star was as poised,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "He";
        let contents = "\
In the twenty-first century,
no pop star was as poised,
as polished, or as generally fierce as Beyoncé.
She scored";

        assert_eq!(
            vec!["In the twenty-first century,","as polished, or as generally fierce as Beyoncé.","She scored"],
            search_case_insensitive(query, contents)
        );
    }
}

