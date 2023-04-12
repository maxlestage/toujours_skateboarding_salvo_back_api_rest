use scraper::{Html, Selector};

pub async fn scraper() -> String {
    // Certains titre en des caractères spéciaux, il serait bon de les identifier en parsant plusieur titre de string à vec<u8> ou en utilisant as_byte() pour trouver les caractères qui produisent des erreurs et les échapper.

    let url = "https://www.thrashermagazine.com/articles/videos/";
    let response = reqwest::get(url).await.expect("scraper error");
    let body = response.text().await.unwrap();
    let document = Html::parse_document(&body);

    let video_selector = Selector::parse(".post-list-item").unwrap();

    let video_name_selector = Selector::parse(".post-title-link").unwrap();
    let video_description_selector = Selector::parse(".post-description").unwrap();
    let video_link_selector = Selector::parse(".post-title-link").unwrap();

    let mut formatted_links = Vec::new(); // initialise le vecteur vide
    let mut formatted_links_string: String = String::new();
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

        let linked = format!("https://www.thrashermagazine.com{link}");
        let data = format!(
            "{{ \"video_name\": \"{}\", \"description\": \"{}\", \"link\": \"{}\" }},",
            video_name, description, linked
        );
        formatted_links.push(data.to_string());
        formatted_links_string = formatted_links.join("\n");
    }
    // formatted_links_string;
    format!("[{}]", formatted_links_string)
}
