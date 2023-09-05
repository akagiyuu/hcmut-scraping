use crate::diff::diff;
use anyhow::Result;
use mongodb::bson::doc;
use mongodb::sync::Client;
use notify_rust::Notification;
use rand::distributions::Alphanumeric;
use rand::Rng;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::sync::OnceLock;
use std::thread::sleep;
use std::time::Duration;

const NEWS_LINK: &str = "https://fme.hcmut.edu.vn/tin-tuc-thong-bao/thong-bao";
const NEWS_PER_PAGE: usize = 10;
static LINK_SELECTOR: OnceLock<Selector> = OnceLock::new();

#[derive(Serialize, Deserialize, Debug)]
pub struct News {
    announcements: Vec<String>,
}

fn get_announcements() -> Result<Vec<String>> {
    let response = reqwest::blocking::get(NEWS_LINK)?;
    let html = Html::parse_document(&response.text()?);
    let selector = LINK_SELECTOR.get_or_init(|| Selector::parse("article > div:nth-child(1) > div:nth-child(2) > header:nth-child(1) > h2:nth-child(1) > a:nth-child(1)").unwrap());

    let mut announcements = Vec::with_capacity(NEWS_PER_PAGE);
    let test: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(5)
        .collect();
    announcements.push(test);
    for el in html.select(selector) {
        announcements.push(el.value().attr("href").unwrap().to_string());
    }
    announcements.pop();
    Ok(announcements)
}

pub fn sync_start(duration_between_sync: Duration) -> Result<!> {
    let client = Client::with_uri_str(
        "mongodb+srv://yuu:yuu123@cluster0.i0emefv.mongodb.net/?retryWrites=true&w=majority",
    )?;

    let database = client.database("hcmut");

    let collection = database.collection::<News>("news");

    loop {
        let new_announcements = get_announcements()?;
        if let Ok(Some(news)) = collection.find_one(None, None) {
            let old_announcements = news.announcements;

            let changes = diff(new_announcements.as_slice(), old_announcements.as_slice());
            if changes.is_empty() {
                println!("No updates");
                continue;
            }

            let mut buffer = String::new();
            for change in changes {
                writeln!(buffer, "{} {}", change.tag(), change.value())?;
            }
            Notification::new()
                .summary("Announcements change")
                .body(&buffer)
                .timeout(0)
                .show()?;
        }

        collection.replace_one(
            doc! {},
            News {
                announcements: new_announcements,
            },
            None,
        )?;
        sleep(duration_between_sync);
    }
}
