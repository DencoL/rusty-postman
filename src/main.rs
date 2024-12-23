use std::time::Duration;

use app::{AppModel, AppState, Message};
use crossterm::event::{self, Event, KeyCode};
use messaging::message_queue::MessageQueue;
use requests_list::request::{Method, Request};

mod app;
mod messaging;
mod requests_list;
mod tui;

fn main() -> color_eyre::Result<()> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;
    let mut app_model = AppModel::new(vec![
        Request::new("Request 1", Some(Method::GET)),
        Request::new("Request 2", Some(Method::POST)),
        Request::new("Request 3", None),
    ]);

    while *app_model.state() != AppState::Done {
        terminal.draw(|f| app_model.view(f))?;

        let current_msg = handle_event()?;
        let mut message_queue: MessageQueue<Message> = MessageQueue::new(&current_msg);
        while let Some(msg) = message_queue.get_message() {
            message_queue.add_messages(app_model.update(msg));
        }
    }

    tui::restore_terminal()?;

    Ok(())
}

fn handle_event() -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }

    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('j') => Some(Message::Down),
        KeyCode::Char('k') => Some(Message::Up),
        _ => None,
    }
}
