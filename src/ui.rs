use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Frame,
};

use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app.tabs.titles.iter().cloned().map(Spans::from).collect();

    draw_tabs(f, chunks[0], titles, app.tabs.index);

    let ids = &app.current_list().unwrap().items;
    let items = app
        .client
        .get_items(ids)
        .into_iter()
        .enumerate()
        .map(|(i, item)| match item {
            Some(item) => format!(
                "{:3} | {} [{}]",
                i + 1,
                item.title.as_ref().unwrap(),
                item.descendants.as_ref().unwrap_or(&0)
            ),
            None => " Error".to_string(),
        })
        .map(Span::raw)
        .map(ListItem::new)
        .collect::<Vec<ListItem>>();

    let state = app.current_list().unwrap().state.clone();

    draw_stateful_list(f, chunks[1], &items, state);
}

fn draw_tabs<B: Backend>(f: &mut Frame<B>, area: Rect, titles: Vec<Spans>, index: usize) {
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL))
        .select(index)
        .highlight_style(Style::default().fg(Color::Magenta));

    f.render_widget(tabs, area);
}

fn draw_stateful_list<'a, B: Backend>(
    f: &mut Frame<B>,
    area: Rect,
    items: &[ListItem<'a>],
    mut state: ListState,
) {
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Magenta));

    f.render_stateful_widget(list, area, &mut state);
}
