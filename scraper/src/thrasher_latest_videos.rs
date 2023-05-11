use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct VideoData {
    video_name: String,
    description: String,
    link: String,
}

pub async fn scraper() -> String {
    let url = "https://www.thrashermagazine.com/articles/videos/";
    let response = reqwest::get(url).await.expect("scraper error");
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    let video_selector = Selector::parse(".post-list-item").unwrap();

    let video_name_selector = Selector::parse(".post-title-link").unwrap();
    let video_description_selector = Selector::parse(".post-description").unwrap();
    let video_link_selector = Selector::parse(".post-title-link").unwrap();

    let mut video_data = Vec::new();
    for element in document.select(&video_selector) {
        let video_name_element = element
            .select(&video_name_selector)
            .next()
            .expect("Could not select video name.");

        let video_name = video_name_element
            .value()
            .attr("title")
            .expect("Could not find title attribute.");

        let description_element = element
            .select(&video_description_selector)
            .next()
            .expect("Could not find description");
        let description = description_element.text().collect::<String>();

        let video_link_element = element
            .select(&video_link_selector)
            .next()
            .expect("Could not select video link.");

        let link = video_link_element
            .value()
            .attr("href")
            .expect("Could not find href attribute.");

        let linked = format!("https://www.thrashermagazine.com{}", link);

        let video = VideoData {
            video_name: video_name.to_string(),
            description,
            link: linked,
        };

        video_data.push(video);
    }

    serde_json::to_string(&video_data).unwrap()
}
