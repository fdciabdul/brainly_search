use reqwest::header::HeaderMap;
use scraper::{Html};

pub fn build_headers() -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert("authority", "srv-unified-search.external.search-systems-production.z-dn.net".parse().unwrap());
    headers.insert("accept", "*/*".parse().unwrap());
    headers.insert("accept-language", "en-US,en;q=0.9,es-ES;q=0.8,es;q=0.7,id-ID;q=0.6,id;q=0.5,zh-CN;q=0.4,zh;q=0.3".parse().unwrap());
    headers.insert("content-type", "text/plain;charset=UTF-8".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("origin", "https://brainly.co.id".parse().unwrap());
    headers.insert("referer", "https://brainly.co.id/".parse().unwrap());
    headers.insert("sec-ch-ua", "\"Google Chrome\";v=\"117\", \"Not;A=Brand\";v=\"8\", \"Chromium\";v=\"117\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "cross-site".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36".parse().unwrap());

    headers
}


pub fn html_to_text(html_content: &str) -> String {
    let fragment = Html::parse_fragment(html_content);
    let text = fragment.root_element().text().collect::<String>();
    text.trim().to_string()
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len-3]) // Truncate and add "..."
    } else {
        s.to_string()
    }
}