use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::widgets::ListState;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
}

impl Events {
    pub fn new() -> Self {
        let tick_rate = Duration::from_millis(250);

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                    if let CEvent::Key(key) = event::read().unwrap() {
                        tx.send(Event::Input(key)).unwrap()
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    tx.send(Event::Tick).unwrap();
                    last_tick = Instant::now();
                }
            }
        });

        Self { rx }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}

pub struct TabsState<'a> {
    pub index: usize,
    pub titles: Vec<&'a str>,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { index: 0, titles }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        self.index = match self.index {
            0 => self.titles.len() - 1,
            n => n - 1,
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        StatefulList { state, items }
    }

    pub fn index(&self) -> usize {
        self.state.selected().unwrap()
    }

    pub fn item(&self) -> &T {
        let i = self.index();
        &self.items[i]
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(n) => (n + 1) % self.items.len(),
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(0) => self.items.len() - 1,
            Some(n) => n - 1,
            None => 0,
        };

        self.state.select(Some(i));
    }
}
