extern crate renert;
extern crate clap;
extern crate colored;
extern crate regex;

use renert::*;
use colored::*;
use clap::{App, Arg};
use regex::Regex;

fn get_str_for_print(stdout: String, filename: &str) -> String {
    if stdout == format!("{}*", filename) {
        return filename.green().bold().to_string();
    } else if stdout == format!("{}@", filename) {
        return filename.cyan().bold().to_string();
    } else if stdout != filename {
        return filename.blue().bold().to_string();
    } else {
        return filename.to_string();
    }
}

fn print_files(files_oneline: String) {
    let files: Vec<&str> = files_oneline.split("\n").collect();
    let blue_period = ".".blue().bold().to_string();
    for x in files {
        let x = x.replace("./", "");
        let oput = system(&["ls", "-F", &x]);
        match oput {
            Ok(oput) => {
                let str_for_print = get_str_for_print(oput.stdout, &x);
                if str_for_print == blue_period {
                    continue
                }
                println!("{}", str_for_print);
            },
            Err(oput) => {
                my_eprint(oput.stderr);
            }
        }
    }
}

fn get_cmd(dir: &str, base: &str, allflg: bool) -> String {
    match allflg {
        true => {
            return format!("find {} -maxdepth 1 -name '{}' -printf '%p\n'", dir, base).to_string();
        },
        false => {
            return format!("find {} -maxdepth 1 -name '{}' -not -name '.*' -printf '%p\n'", dir, base).to_string();
        }
    }
}

fn decode_and_print(dir: &str, base: &str, allflg: bool) {
    let cmd = get_cmd(dir, base, allflg);
    let oput = system_on_shell(&cmd);
    match oput {
        Ok(oput) => {
            print_files(oput.stdout);
        },
        Err(oput) => {
            my_eprint(oput.stderr);
        }
    }
}

fn lf(param: &str, dir_regex: &Regex, allflg: bool) {
    let dir = dir_regex.find(param);
    match dir {
        Some(dir) => {
            let dir = dir.as_str();
            let base = dir_regex.replace(param, "");
            decode_and_print(dir, &base, allflg);
        },
        None => {
            decode_and_print("", param, allflg);
        }
    }
}

fn main() {
    let dir_regex = Regex::new(".*/").unwrap();
    let matches = App::new("lf")
        .version("0.0.1")
        .arg(Arg::with_name("PATTERN")
             .help("file path pattern")
             .takes_value(true)
             .multiple(true)
             )
        .arg(Arg::with_name("all")
             .help("Prints all")
             .short("a")
             .long("all")
             )
        .get_matches();
    match matches.values_of("PATTERN") {
        Some(params) => {
            let allflg = matches.is_present("all");
            for x in params {
                lf(x, &dir_regex, allflg);
            }
        },
        _ => {
            let allflg = matches.is_present("all");
            decode_and_print("", "*", allflg);
        }
    }
}
