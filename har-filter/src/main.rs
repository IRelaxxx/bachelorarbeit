use clap::App;
use har;
use std::fs;
use std::fs::OpenOptions;
use std::{io::Write, path::Path};

mod filter;
mod nginx_push;

// TODO: errorcodes
fn main() {
    let matches = App::new("har-filter")
    .version("1.0.0")
    .author("Alexander Krahl <alexander.krahl@stud.htwk-leipzig.de>")
    .about("")
    .arg("-o, --output=[FILE] 'Output file'")
    .arg("-t, --outputtime=[FILE] 'Append elapsed time of the request to a file or created it if it doesnt exist'")
    .arg("[input] 'input file'")
    .arg("[whitelist] 'whitelist file with valid regexes, one per line. see https://docs.rs/regex/1.3.9/regex/#syntax'")
    .subcommand(App::new("pushconfig")
        .about("Generates a NGINX config file for HTTP2 Server push")
        .arg("[input] 'Input file'")
        .arg("[output] 'Output file'"))
    .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("pushconfig") {
        let input_file = match matches.value_of("input") {
            Some(x) => {
                if Path::new(x).exists() {
                    x
                } else {
                    eprintln!("error: input file not found");
                    std::process::exit(1)
                }
            }
            None => {
                eprintln!("No input file provided");
                std::process::exit(0)
            }
        };

        let output_file = match matches.value_of("output") {
            Some(x) => {
                /*if Path::new(x).exists() {
                    x
                } else {
                    eprintln!("error: output file not found");
                    std::process::exit(1)
                }*/
                x
            }
            None => {
                eprintln!("No output file provided");
                std::process::exit(0)
            }
        };

        let spec = match har::from_path(input_file) {
            Ok(spec) => spec,
            Err(err) => {
                eprintln!("error: {}", err);
                std::process::exit(1)
            }
        };
        let log = match spec.log {
            har::Spec::V1_2(x) => x,
            har::Spec::V1_3(_) => {
                eprintln!("error: har 1.3 not supported");
                std::process::exit(1)
            }
        };

        nginx_push::generate_config_file_for_url(&log, output_file);

        return;
    }

    let input_file = match matches.value_of("input") {
        Some(x) => {
            if Path::new(x).exists() {
                x
            } else {
                eprintln!("error: input file not found");
                std::process::exit(1)
            }
        }
        None => {
            eprintln!("No input file provided");
            std::process::exit(0)
        }
    };

    let spec = match har::from_path(input_file) {
        Ok(spec) => spec,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1)
        }
    };

    let log = match spec.log {
        har::Spec::V1_2(x) => x,
        har::Spec::V1_3(_) => {
            eprintln!("error: har 1.3 not supported");
            std::process::exit(1)
        }
    };

    let whitelist_file_path = match matches.value_of("whitelist") {
        Some(x) => {
            if Path::new(x).exists() {
                x
            } else {
                eprintln!("error: whitelist file not found");
                std::process::exit(1)
            }
        }
        None => {
            eprintln!("No whitelist file provided");
            std::process::exit(0)
        }
    };

    let whitelist_file = Path::new(whitelist_file_path);

    let (newlog, elapsedtime) = filter::filter_har_and_calculate_time(&log, whitelist_file);
    match matches.value_of("outputtime") {
        Some(x) => {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(x)
                .unwrap();
            file.write(format!("{}\n", elapsedtime.to_string()).as_bytes());
        }
        None => {}
    }
    println!("Total time elapsed: {}ms", elapsedtime);

    match matches.value_of("output") {
        Some(x) => {
            let spec = har::Har {
                log: har::Spec::V1_2(newlog),
            };
            let output_json = har::to_json(&spec).unwrap();
            fs::write(x, output_json).unwrap();
        }
        None => {}
    };
}
