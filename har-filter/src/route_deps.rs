use regex::RegexSet;

lazy_static! {
    static ref ALLOWED_CONTENT_TYPE_REGEXSET: RegexSet = RegexSet::new(&[""]).unwrap();
}

pub fn calc_deps(input: &har::v1_2::Log, output_file: &str) -> () {
    let request_url = &input.entries.first().unwrap().request.url;
    let mut iter = input.entries.clone().into_iter();
    iter.next(); // skip first value
    let deps: Vec<_> = iter.filter(|x|{
        !x.request.headers.clone().into_iter().filter(|y|{
            y.name == "Content-Type" && ALLOWED_CONTENT_TYPE_REGEXSET.is_match(&y.value)
        }).collect::<Vec<_>>().is_empty()
    }).map(|x| {
        x.request.url
    }).collect();
}