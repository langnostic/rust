use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed in");
        }

        Ok(Self {
            query: args[1].as_str(),
            filename: args[2].as_str(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    //println!("With text:\n{}", contents);
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|s| s.contains(query)).collect()
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    mod config_tests {
        use std::vec;

        use super::*;
        #[test]
        fn config_new_panics_with_1_arg() {
            let args = vec!["Hello Rust Tests".to_string()];
            let result = Config::new(&args);
            assert!(result.is_err());
        }

        #[test]
        fn config_new_panics_with_2_arg() {
            let args = vec!["Hello Rust".to_string(), "Tests".to_string()];
            let result = Config::new(&args);
            assert!(result.is_err());
        }

        #[test]
        fn config_new_works_with_3_args() {
            let query = "Rust";
            let filename = "Tests";
            let args = vec!["Hello".to_string(), query.to_string(), filename.to_string()];
            let config = Config::new(&args).unwrap();
            assert_eq!(query, config.query);
            assert_eq!(filename, config.filename);
        }

        #[test]
        fn config_new_works_with_multiple_args() {
            let query = "b";
            let filename = "c";
            let args = vec![
                "a".to_string(),
                query.to_string(),
                filename.to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
            ];
            let config = Config::new(&args).unwrap();
            assert_eq!(query, config.query);
            assert_eq!(filename, config.filename);
        }
    }

    mod run_tests {
        #[test]
        fn run_works() {
            assert!(false);
        }
    }

    mod search_tests {
        use super::*;
        #[test]
        fn search_works() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        }

        #[test]
        fn returns_empty_vec_if_pattern_doesnot_match() {
            let query = ";lkjasdf;kljsadf";
            let contents = "\
            hark.
            some stupid shit i bet.
            derp derp derp.
            ";
            let empty_vec: Vec<&str> = vec![];
            assert_eq!(empty_vec, search(query, contents));
        }
    }
}
