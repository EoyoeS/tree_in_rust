use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

#[derive(Debug)]
pub struct Config {
    dir_path: String,
    out_file: Option<File>,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        let dir_path: String;
        let out_file: Option<File>;
        // 找-f参数
        if let Some(index) = args.iter().position(|x| x.as_str() == "-f") {
            if index == 1 {
                dir_path = env::current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap();
            } else {
                let dir_ = &args[1];
                if !Path::new(dir_).is_dir() {
                    return Err("The director is invalid");
                }
                dir_path = dir_.clone();
            }
            if let Some(file_) = args.get(index + 1) {
                out_file = Some(File::create(file_).unwrap());
            } else {
                return Err("Input the out file");
            }
        } else {
            out_file = None;
            if args.len() < 2 {
                dir_path = env::current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap();
            } else {
                let dir_ = &args[1];
                if !Path::new(dir_).is_dir() {
                    return Err("The director is invalid");
                }
                dir_path = dir_.clone();
            }
        }
        Ok(Config { dir_path, out_file })
    }
}

struct Symbols<'a>(String, &'a str);

impl Symbols<'_> {
    const PASS: &str = "│   ";
    const NODE: &str = "├───";
    const LAST_PASS: &str = "    ";
    const LAST_NODE: &str = "└───";
}

fn out(res: String, out_file: &mut Option<File>) -> io::Result<()> {
    if let Some(out_file) = out_file {
        out_file.write_all((res + "\n").as_bytes())?;
    } else {
        println!("{res}");
    }
    Ok(())
}
fn do_path(
    path: &Path,
    symbols: Symbols,
    pass: &str,
    out_file: &mut Option<File>,
) -> io::Result<()> {
    let prefix = symbols.0.clone() + &symbols.1;
    let res = format!(
        "{}{}",
        prefix.as_str(),
        path.file_name().unwrap().to_str().unwrap()
    );
    out(res, out_file)?;
    if path.is_dir() {
        tree_dir(path, symbols.0 + pass, out_file)?;
    }
    Ok(())
}

fn tree_dir(dir_path: &Path, pass: String, out_file: &mut Option<File>) -> io::Result<()> {
    let mut path_iter = fs::read_dir(Path::new(dir_path)).unwrap();
    if let Some(last_path) = path_iter.next() {
        let mut last_path = last_path.unwrap().path();
        for path in path_iter {
            let symbols = Symbols(pass.clone(), Symbols::NODE);
            do_path(&last_path, symbols, Symbols::PASS, out_file)?;
            last_path = path.unwrap().path();
        }
        let symbols = Symbols(pass, Symbols::LAST_NODE);
        do_path(&last_path, symbols, Symbols::LAST_PASS, out_file)?;
    }
    Ok(())
}

pub fn run(mut config: Config) -> io::Result<()> {
    out(config.dir_path.clone(), &mut config.out_file)?;
    tree_dir(
        &Path::new(&config.dir_path),
        String::new(),
        &mut config.out_file,
    )?;
    Ok(())
}
