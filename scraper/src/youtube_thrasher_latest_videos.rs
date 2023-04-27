use reqwest::{header::{COOKIE, USER_AGENT, HeaderValue}, cookie};

use scraper::{Html, Selector};

pub async fn scraper_yt() -> String {

    let _cookie_url = "https://www.youtube.com";
    let policy_url = "https://policies.google.com/privacy?hl=fr-FR";
    let client = reqwest::Client::new();

    let response = client.get(policy_url)
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .send()
    .await
    .expect("Error sending policy request");

    let cookies: Vec<cookie::Cookie> = response
    .cookies()
    .collect();

let cookie = cookies.iter()
    .find(|c| c.name() == "CONSENT")
    .expect("Could not find CONSENT cookie");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"));
    headers.insert(COOKIE, HeaderValue::from_str(&format!("CONSENT={};", cookie.value())).unwrap());

    let url = "https://www.youtube.com/c/ThrasherMagazine/videos";
    let response = reqwest::get(url).await.expect("scraper error");
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    let video_selector = Selector::parse("ytd-grid-video-renderer").unwrap();

    let mut formatted_links = Vec::new();
    let formatted_links_string: String;
    for element in document.select(&video_selector) {
        let title_selector = Selector::parse("a#video-title").unwrap();
        let description_selector = Selector::parse("yt-formatted-string#description-text").unwrap();

            println!("{:#?}", title_selector);
        let title = element.select(&title_selector).next().unwrap().text().collect::<String>();
        let description = element.select(&description_selector).next().unwrap().text().collect::<String>();
        let href = element.value().attr("href").unwrap();

        let linked = format!("https://www.youtube.com{}", href);
        let data = format!(
            "{{ \"video_name\": \"{}\", \"description\": \"{}\", \"link\": \"{}\" }},",
            title, description, linked
        );
        formatted_links.push(data.to_string());
    }
    formatted_links_string = formatted_links.join("\n");
    format!("[{}]", formatted_links_string)
}
