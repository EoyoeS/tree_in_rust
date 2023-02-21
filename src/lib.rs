use std::{env, error::Error, fs, path::Path};

#[derive(Debug)]
pub struct Config {
    pub dir_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, Box<dyn Error>> {
        if args.len() < 2 {
            return Ok(Config {
                dir_path: env::current_dir()?.into_os_string().into_string().unwrap(),
            });
        }
        let dir_path = args[1].clone();
        if Path::new(&dir_path).exists() {
            return Ok(Config {
                dir_path: args[1].clone(),
            });
        }
        Err("The path should exist".into())
    }
}

struct Symbols<'a>(String, &'a str);

impl Symbols<'_> {
    const PASS: &str = "│   ";
    const NODE: &str = "├───";
    const LAST_PASS: &str = "    ";
    const LAST_NODE: &str = "└───";
}

fn do_path(path: &Path, symbols: Symbols, pass: &str) {
    let prefix = symbols.0.clone() + &symbols.1;
    println!(
        "{}{}",
        prefix.as_str(),
        path.file_name().unwrap().to_str().unwrap()
    );
    if path.is_dir() {
        tree_dir(path, symbols.0 + pass);
    }
}

fn tree_dir(dir_path: &Path, pass: String) {
    let mut path_iter = fs::read_dir(Path::new(dir_path)).unwrap();
    if let Some(last_path) = path_iter.next() {
        let mut last_path = last_path.unwrap().path();
        for path in path_iter {
            let symbols = Symbols(pass.clone(), Symbols::NODE);
            do_path(&last_path, symbols, Symbols::PASS);
            last_path = path.unwrap().path();
        }
        let symbols = Symbols(pass, Symbols::LAST_NODE);
        do_path(&last_path, symbols, Symbols::LAST_PASS);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", &config.dir_path);
    tree_dir(&Path::new(&config.dir_path), String::new());
    Ok(())
}
