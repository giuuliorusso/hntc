use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

mod client;
mod fetch;

pub use client::HnClient;
pub use fetch::*;

pub type ItemId = usize;
pub type UserId = String;

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    id: ItemId,
    #[serde(rename(deserialize = "type"))]
    kind: ItemKind,
    by: UserId,
    #[serde(with = "unix_date_format")]
    time: DateTime<Utc>,

    dead: Option<bool>,
    deleted: Option<bool>,
    kids: Option<Vec<ItemId>>,
    parent: Option<ItemId>,
    parts: Option<Vec<ItemId>>,
    poll: Option<ItemId>,
    pub descendants: Option<usize>,
    pub title: Option<String>,
    pub url: Option<String>,
    score: Option<isize>,
    text: Option<String>,
}

#[derive(Clone, Debug)]
enum ItemKind {
    Comment,
    Job,
    Poll,
    PollOpt,
    Story,
}

impl<'de> Deserialize<'de> for ItemKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let item_type = match s.as_str() {
            "comment" => ItemKind::Comment,
            "job" => ItemKind::Job,
            "poll" => ItemKind::Poll,
            "pollopt" => ItemKind::PollOpt,
            "story" => ItemKind::Story,
            _ => unreachable!(),
        };

        Ok(item_type)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
    id: UserId,

    about: String,
    #[serde(with = "unix_date_format")]
    created: DateTime<Utc>,
    delay: Option<usize>,
    karma: isize,
    submitted: Vec<ItemId>,
}

mod unix_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(deserializer)?;
        Ok(Utc.timestamp(n, 0))
    }
}
