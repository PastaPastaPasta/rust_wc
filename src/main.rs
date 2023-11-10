use std::{env, fs, process};
use std::io::{self, BufRead};

struct Options {
    lines: bool,
    words: bool,
    chars: bool
}

fn get_args(args: Vec<String>) -> (Options, String) {
    let mut opt = Options {
        lines: true,
        words: true,
        chars: true
    };
    let mut filename = String::new();
    for arg in &args[1..] {
        if arg.starts_with('-') {
            opt = Options {
                lines: false,
                words: false,
                chars: false
            };
            for c in arg.chars().skip(1) {
                match c {
                    'w' => opt.words = true,
                    'c' => opt.chars = true,
                    'l' => opt.lines = true,
                    _ => {
                        eprintln!("Invalid option: -{}", c);
                        process::exit(1);
                    }
                }
            }
        } else {
            if !filename.is_empty() {
                eprintln!("Multiple files not supported");
                process::exit(1);
            }
            filename = arg.clone();
        }
    }
    (opt, filename)
}

fn process_file(opt: Options, filename: &str) {
    match fs::File::open(filename) {
        Ok(file) => {
            let chars = file.metadata().expect("Unable to access file metadata").len();
            let mut lines = 0;
            let mut words = 0;

            if opt.lines || opt.words {
                let mut file = io::BufReader::new(file);
                let mut line = String::new();
                while let Ok(bytes_read) = file.read_line(&mut line) {
                    if bytes_read == 0 {
                        break;
                    }
                    if line.ends_with('\n') {
                        lines += 1;
                    }
                    if opt.words { words += line.split_whitespace().count(); }
                    line.clear();
                }
            }

            if opt.lines { print!("{} ", lines); }
            if opt.words { print!("{} ", words); }
            if opt.chars { print!("{} ", chars); }
            println!("{}", filename);
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: mywc [-wcl] <file>");
        process::exit(1);
    }

    let (opt, filename) = get_args(args);

    if filename.is_empty() {
        eprintln!("No file specified");
        process::exit(1);
    }
    process_file(opt, &filename);
}
