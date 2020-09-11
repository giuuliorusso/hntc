use std::collections::HashMap;

use super::*;

#[derive(Debug)]
pub struct HnClient {
    cached_items: HashMap<ItemId, Item>,
}

impl HnClient {
    pub fn new() -> Self {
        HnClient {
            cached_items: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn save_item(&mut self, id: ItemId) -> Result<(), reqwest::Error> {
        self.cached_items.entry(id).or_insert(fetch_item(id).await?);
        Ok(())
    }

    pub async fn save_items(&mut self, ids: &[ItemId]) {
        // TODO
        let items_not_cached = ids
            .iter()
            .filter(|id| !self.cached_items.contains_key(id))
            .copied()
            .collect::<Vec<ItemId>>();

        let items = fetch_items(&items_not_cached).await;

        items.into_iter().enumerate().for_each(|(i, item)| {
            if let Ok(item) = item {
                let id = items_not_cached[i];
                self.cached_items.insert(id, item);
            }
        });
    }

    pub fn get_item(&self, id: ItemId) -> Option<&Item> {
        self.cached_items.get(&id)
    }

    pub fn get_items(&self, ids: &[ItemId]) -> Vec<Option<&Item>> {
        ids.iter().map(|id| self.cached_items.get(id)).collect()
    }
}
