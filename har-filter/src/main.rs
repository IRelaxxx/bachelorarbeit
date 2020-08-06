use clap::App;
use har;
use std::fs;
use std::path::Path;

mod filter;

// TODO: errorcodes
fn main() {
    let matches = App::new("har-filter")
    .version("1.0.0")
    .author("Alexander Krahl <alexander.krahl@stud.htwk-leipzig.de>")
    .about("")
    .arg("-o, --output=[FILE] 'Output file'")
    .arg("[input] 'input file'")
    .arg("[whitelist] 'whitelist file with valid regexes, one per line. see https://docs.rs/regex/1.3.9/regex/#syntax'")
    .get_matches();

    let input_file_path = match matches.value_of("input") {
        Some(x) => {
            if Path::new(x).exists() {
                x
            } else {
                eprintln!("error: input file not found");
                std::process::exit(1)     
            }
        }
        None => { eprintln!("No input file provided"); std::process::exit(0)}
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
        None => { eprintln!("No whitelist file provided"); std::process::exit(0)}
    };

    let whitelist_file = Path::new(whitelist_file_path);

    let spec = match har::from_path(input_file_path) {
        Ok(spec) => { spec },
        Err(err) => {
            eprintln!("error: {}", err); 
            std::process::exit(1) 
        }
    };

    let (newlog, elapsedtime) = match spec.log {
        har::Spec::V1_2(x) => {
            filter::filter_har_and_calculate_time(&x, whitelist_file)
        }
        har::Spec::V1_3(_) => {
            eprintln!("error: har 1.3 not supported");
            std::process::exit(1) 
        }
    };

    println!("Total time elapsed: {}ms", elapsedtime);

    match matches.value_of("output") {
        Some(x) => {
            let spec = har::Har { log: har::Spec::V1_2(newlog)};
            let output_json = har::to_json(&spec).unwrap();
            fs::write(x, output_json).unwrap();
        }
        None => {}
    };
}