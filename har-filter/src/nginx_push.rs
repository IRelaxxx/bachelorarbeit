use har;
use std::fs::OpenOptions;
use std::io::Write;
use url::Url;

fn get_requested_files(data: &har::v1_2::Log, url: &String) -> Vec<String> {
    let url = Url::parse(url).unwrap();
    let domain = url.host_str().unwrap();
    let ret = data
        .entries
        .clone()
        .iter()
        .skip(1)
        .filter_map(|entry| {
            let parsed_url = Url::parse(&entry.request.url).unwrap();
            if parsed_url.host_str().unwrap() == domain {
                Some(parsed_url.path().to_string())
            } else {
                None
            }
        })
        .collect();
    ret
}

pub fn generate_config_file_for_url(data: &har::v1_2::Log, destination: &str) {
    let first_request = data.entries.first().unwrap();
    let url = Url::parse(&first_request.request.url).unwrap();
    let urls = get_requested_files(data, &first_request.request.url);
    let mut push_lines: Vec<String> = Vec::new();
    for x in &urls {
        push_lines.push(format!("    http2_push {}; \n", x));
    }
    let file_content = format!("location = {} {{\n{}}}", url.path(), push_lines.concat());
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(destination)
        .unwrap();
    file.write(file_content.as_bytes());
}
