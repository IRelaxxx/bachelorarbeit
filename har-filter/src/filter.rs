use chrono::DateTime;
use har;
use regex::RegexSet;
use std::fs;
use std::path::Path;

pub fn filter_har_and_calculate_time(data: &har::v1_2::Log, whitelist_file: &Path) -> (har::v1_2::Log, i64) {
    let whitelist = read_whitelist(whitelist_file);
    let filtered_entries = filter_entries(&data, &whitelist);
    let elapsedtime = calculate_elapsedtime(&filtered_entries);
    (filtered_entries, elapsedtime)
}

fn filter_entries(data: &har::v1_2::Log, whitelist: &Vec<String>) -> har::v1_2::Log {
    let regexset = match RegexSet::new(whitelist) {
        Ok(x) => { x }
        Err(_) => {
            eprintln!("error: compiling whitelist regex");
            std::process::exit(1) 
        }
    };
    let filtered_entries: Vec<_> = data.entries.clone()
        .into_iter()
        .filter(|x| {
            regexset.is_match(&x.request.url)
        })
        .collect();
    let new_data = har::v1_2::Log {
        browser: data.browser.clone(),
        creator: data.creator.clone(),
        pages: data.pages.clone(),
        entries: filtered_entries,
        comment: data.comment.clone()
    };
    new_data
}

fn read_whitelist(path: &Path) -> Vec<String> {
    let wholefile = match fs::read_to_string(path){
        Ok(x) => { x }
        Err(_) => {
            eprintln!("error: reading whitelist file");
            std::process::exit(1) 
        }
    };
    wholefile.lines()
        .filter(|x| {!x.is_empty()})
        .map(|x| {x.to_string()})
        .collect()
}

// TODO: remove unwraps
fn calculate_elapsedtime(log: &har::v1_2::Log) -> i64 {
    let (mut start_datetimes, mut end_datetimes): (Vec<_>, Vec<_>) = log.entries.iter().map(|x|{
        let starttime = DateTime::parse_from_rfc3339(&x.started_date_time).unwrap();
        // TODO better time resolution so i dont have to round
        (starttime, starttime.checked_add_signed(chrono::Duration::milliseconds(x.time.round() as i64)).unwrap())
    }).unzip();
    start_datetimes.sort();
    end_datetimes.sort();
    end_datetimes.last().unwrap().timestamp_millis() - start_datetimes.first().unwrap().timestamp_millis()
}