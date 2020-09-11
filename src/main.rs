use crossterm::{
    event::{EnableMouseCapture, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use std::io::{stdout, Write};
use structopt::StructOpt;
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod hn;
mod ui;
mod utils;

use app::App;
use utils::{Event, Events};

#[derive(StructOpt)]
#[structopt(
    version = "0.1.0",
    about = "(Useless) HN Terminal Client",
    author = "Giulio Russo <me@giuuliorusso.me>"
)]
struct Opt {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Opt::from_args();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let events = Events::new();

    let mut app = App::new();
    app.fetch_list().await?;

    loop {
        // Handle key events
        if let Event::Input(key_event) = events.next()? {
            match key_event.code {
                // Quit
                KeyCode::Char('q') => break,

                // Next tab
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => {
                    app.tabs.next();
                    if app.current_list().is_none() {
                        app.fetch_list().await?;
                    }
                }

                // Previous tab
                KeyCode::Left | KeyCode::Char('h') => {
                    app.tabs.previous();
                    if app.current_list().is_none() {
                        app.fetch_list().await?;
                    }
                }

                // Open article/linked page
                KeyCode::Enter => {
                    let &id = app.current_list().unwrap().item();
                    if let Some(item) = app.client.get_item(id) {
                        if let Some(url) = item.url.as_ref() {
                            open::that(url)?;
                        }
                    }
                }

                // Open comments page
                KeyCode::Char('c') => {
                    let &id = app.current_list().unwrap().item();
                    let url = format!("https://news.ycombinator.com/item?id={}", id);
                    open::that(url)?;
                }

                // Move down
                KeyCode::Down | KeyCode::Char('j') => {
                    app.current_list_mut().unwrap().next();
                }

                // Move up
                KeyCode::Up | KeyCode::Char('k') => {
                    app.current_list_mut().unwrap().previous();
                }

                _ => {}
            }
        }

        // Draw
        terminal.draw(|mut f| {
            ui::draw(&mut f, &app);
        })?;
    }

    Ok(())
}
