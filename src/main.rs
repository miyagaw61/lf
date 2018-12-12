extern crate renert;
extern crate clap;
extern crate colored;

use renert::*;
use colored::*;
use clap::{App, Arg};

fn make_cmd(matches: &clap::ArgMatches) -> (String, String) {
    let mut fd_d = "fd --type d ".to_string();
    let mut fd_f = "fd --type f --type l ".to_string();
    if ! matches.is_present("all") {
        fd_d = [&fd_d, "-d1 "].join("");
        fd_f = [&fd_f, "-d1 "].join("");
    }
    match matches.values_of("exclude") {
        Some(exclude) => {
            for x in exclude {
                let glob = x.replace("@", "*");
                fd_d = [&fd_d, "-E '", &glob, "' "].join("");
                fd_f = [&fd_f, "-E '", &glob, "' "].join("");
            }
        },
        None => {}
    }
    let mut cmd_d = "".to_string();
    let mut cmd_f = "".to_string();
    match matches.values_of("PATTERNS") {
        Some(patterns) => {
            for (i, pattern) in patterns.enumerate() {
                let pattern = pattern.replace("\\", "\\\\");
                if i == 0 {
                    cmd_d = [&cmd_d, pattern.as_str()].join("");
                    cmd_f = [&cmd_f, pattern.as_str()].join("");
                }
                else {
                    let pattern_len = pattern.chars().count();
                    let last_pattern = pattern.chars().nth(pattern_len-1).unwrap();
                    if last_pattern == '$' {
                        cmd_d = [&cmd_d, " | rg '[^/]*", &pattern, "'"].join("");
                        cmd_f = [&cmd_d, " | rg '[^/]*", &pattern, "'"].join("");
                    }
                    else {
                        cmd_d = [&cmd_d, " | rg '[^/]*", &pattern, "[^/]*$'"].join("");
                        cmd_f = [&cmd_d, " | rg '[^/]*", &pattern, "[^/]*$'"].join("");
                    }
                }
            }
        },
        None => {}
    }
    cmd_d = [fd_d, cmd_d].join("");
    cmd_f = [fd_f, cmd_f].join("");
    return (cmd_d.to_string(), cmd_f.to_string());
}

fn main() {
    let matches = App::new("lf")
        .version("0.0.1")
        .arg(Arg::with_name("PATTERNS")
             .help("regex patterns")
             .takes_value(true)
             .multiple(true)
             )
        .arg(Arg::with_name("exclude")
             .help("exclude")
             .short("e")
             .long("exclude")
             .takes_value(true)
             .multiple(true)
             )
        .arg(Arg::with_name("all")
             .help("all depth")
             .short("a")
             .long("all")
             )
        .arg(Arg::with_name("0")
             .help("null separate")
             .short("0")
             )
        .get_matches();
    let (d_cmd, f_cmd) = make_cmd(&matches);
    match system_on_shell(&d_cmd) {
        Ok(oput) => {
            if oput.stdout != "" {
                if matches.is_present("0") {
                    print!("{} ", oput.stdout.replace("\n", "\x00"));
                    print!("\x00");
                }
                else {
                    println!("{}", oput.stdout.blue().bold().to_string());
                }
            }
        },
        Err(oput) => {
            println!("ERROR: {}", oput.stderr);
        }
    }
    match system_on_shell(&f_cmd) {
        Ok(oput) => {
            if oput.stdout != "" {
                if matches.is_present("0") {
                    print!("{} ", oput.stdout.replace("\n", "\x00"));
                }
                else {
                    println!("{}", oput.stdout);
                }
            }
        },
        Err(oput) => {
            println!("ERROR: {}", oput.stderr);
        }
    }
}

