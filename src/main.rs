extern crate renert;
extern crate clap;
extern crate colored;

use renert::*;
use colored::*;
use clap::{App, Arg};

fn make_cmd(matches: &clap::ArgMatches) -> (String, String) {
    let mut fd_d = "fd --type d ".to_string();
    let mut fd_f = "fd --type f --type l ".to_string();
    match matches.value_of("depth") {
        Some(depth) => {
            fd_d = [&fd_d, "-d", depth, " "].join("");
            fd_f = [&fd_f, "-d", depth, " "].join("");
        },
        None => {
            if ! matches.is_present("all") {
                fd_d = [&fd_d, "-d1 "].join("");
                fd_f = [&fd_f, "-d1 "].join("");
            }
        }
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
    let mut regex_ext = "".to_string();
    match matches.values_of("file") {
        Some(extensions) => {
            for (i, extension) in extensions.enumerate() {
                if i == 0 {
                    regex_ext = ["\\.", &extension, "$"].join("");
                } else {
                    regex_ext = [&regex_ext, "|\\.", &extension, "$"].join("");
                }
            }
            cmd_d = [&cmd_d, " | rg '", &regex_ext, "'"].join("");
            cmd_f = [&cmd_d, " | rg '", &regex_ext, "'"].join("");
        },
        None => {}
    }
    match matches.values_of("file-exclude") {
        Some(extensions) => {
            for extension in extensions {
                cmd_d = [&cmd_d, " | rg -v '\\.", &extension, "$'"].join("");
                cmd_f = [&cmd_d, " | rg -v '\\.", &extension, "$'"].join("");
            }
        },
        None => {}
    }
    cmd_d = [fd_d, cmd_d].join("");
    cmd_f = [fd_f, cmd_f].join("");
    match matches.value_of("type") {
        Some(typ) => {
            if typ == "f" {
                cmd_d = "echo '' > /dev/null".to_string();
            } else if typ == "d" {
                cmd_f = "echo '' > /dev/null".to_string();
            }
        },
        None => {}
    }
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
        .arg(Arg::with_name("type")
             .help("file type(f or d)")
             .long("type")
             .short("t")
             .takes_value(true)
             )
        .arg(Arg::with_name("depth")
             .help("max depth")
             .long("depth")
             .short("d")
             .takes_value(true)
             )
        .arg(Arg::with_name("file")
             .help("file-extension")
             .long("file")
             .short("f")
             .takes_value(true)
             .multiple(true)
             )
        .arg(Arg::with_name("file-exclude")
             .help("file-extension exclude")
             .long("file-exclude")
             .short("F")
             .takes_value(true)
             .multiple(true)
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

