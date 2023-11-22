// pub fn is_valid_url(url: &str) -> bool {
//     // [(http(s)?):\/\/(www\.)?a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)
//     regex::Regex::new(r"[(http(s)?):\/\/a-zA-Z0-9@:%._\-\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)")
//         .unwrap()
//         .is_match(url)
// }

pub fn is_valid_url(url: &str) -> bool {
    true
}

pub fn extract_domain(url_str: &str) -> Option<String> {
    if let Ok(url) = url::Url::parse(url_str) {
        if let Some(host) = url.host_str() {
            // Split the host into parts
            let parts: Vec<&str> = host.split('.').collect();

            // If there are more than two parts, take the last two
            if parts.len() >= 2 {
                let domain = format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]);
                return Some(domain);
            }
        }
    }
    None
}
