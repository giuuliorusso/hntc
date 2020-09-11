use super::*;

pub mod url {
    use crate::hn::{ItemId, UserId};

    pub const ASK_STORIES: &str = "https://hacker-news.firebaseio.com/v0/askstories.json";
    pub const BEST_STORIES: &str = "https://hacker-news.firebaseio.com/v0/beststories.json";
    pub const JOB_STORIES: &str = "https://hacker-news.firebaseio.com/v0/jobstories.json";
    pub const NEW_STORIES: &str = "https://hacker-news.firebaseio.com/v0/newstories.json";
    pub const SHOW_STORIES: &str = "https://hacker-news.firebaseio.com/v0/showstories.json";
    pub const TOP_STORIES: &str = "https://hacker-news.firebaseio.com/v0/topstories.json";

    pub fn item(id: ItemId) -> String {
        format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id)
    }

    #[allow(dead_code)]
    pub fn user(id: UserId) -> String {
        format!("https://hacker-news.firebaseio.com/v0/user/{}.json", id)
    }
}

pub async fn fetch_ids(url: &str) -> Result<Vec<ItemId>, reqwest::Error> {
    reqwest::get(url).await?.json().await
}

pub async fn fetch_item(id: ItemId) -> Result<Item, reqwest::Error> {
    reqwest::get(&url::item(id)).await?.json().await
}

pub async fn fetch_items(ids: &[ItemId]) -> Vec<Result<Item, reqwest::Error>> {
    let handles = ids.iter().map(|&id| fetch_item(id)).map(tokio::spawn);

    futures::future::try_join_all(handles)
        .await
        .unwrap()
        .into_iter()
        .collect()
}
