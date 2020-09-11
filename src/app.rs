use crate::{
    hn::{self, HnClient, ItemId},
    utils::{StatefulList, TabsState},
};

pub struct App<'a> {
    pub client: HnClient,
    pub tabs: TabsState<'a>,
    lists: Vec<Option<StatefulList<ItemId>>>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        Self {
            client: HnClient::new(),
            tabs: TabsState::new(vec![
                "🏅 Top",
                "⏰ New",
                "📈 Best",
                "❓ Ask",
                "🎪 Show",
                "💼 Jobs",
            ]),
            lists: vec![None, None, None, None, None, None],
        }
    }

    pub fn current_list(&self) -> Option<&StatefulList<ItemId>> {
        self.lists[self.tabs.index].as_ref()
    }

    pub fn current_list_mut(&mut self) -> Option<&mut StatefulList<ItemId>> {
        self.lists[self.tabs.index].as_mut()
    }

    pub async fn fetch_list(&mut self) -> Result<(), reqwest::Error> {
        let urls = [
            hn::url::TOP_STORIES,
            hn::url::NEW_STORIES,
            hn::url::BEST_STORIES,
            hn::url::ASK_STORIES,
            hn::url::SHOW_STORIES,
            hn::url::JOB_STORIES,
        ];

        let index = self.tabs.index;
        let url = urls[index];

        let ids = hn::fetch_ids(url)
            .await?
            .into_iter()
            .take(30) // TODO
            .collect::<Vec<ItemId>>();
        self.client.save_items(&ids).await;

        self.lists[index] = Some(StatefulList::with_items(ids));

        Ok(())
    }
}
