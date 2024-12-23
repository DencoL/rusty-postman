use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Line,
    widgets::{Block, Borders},
    Frame,
};

use crate::requests_list::{request::Request, requests_list_model::RequestsListModel};

#[derive(PartialEq)]
pub enum AppState {
    Running,
    Done,
}

#[derive(Clone, Debug)]
pub enum Message {
    Quit,
    Up,
    Down,
}

pub struct AppModel {
    state: AppState,
    requests: RequestsListModel,
}

impl AppModel {
    pub fn new(requests: Vec<Request>) -> Self {
        return AppModel {
            state: AppState::Running,
            requests: RequestsListModel::new(requests),
        };
    }

    pub fn state(&self) -> &AppState {
        return &self.state;
    }

    pub fn view(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(35),
                Constraint::Percentage(35),
            ])
            .split(frame.area());

        self.render_requests_list(frame, chunks[0]);
    }

    fn render_requests_list(&mut self, frame: &mut Frame, target: Rect) {
        self.requests
            .view(frame, target, Self::frame_block("Requests"));
    }

    fn frame_block<'a>(title: &'a str) -> Block<'a> {
        return Block::new()
            .title(Line::raw(title).centered())
            .borders(Borders::ALL);
    }

    pub fn update(&mut self, msg: Message) -> Vec<Option<Message>> {
        let messages: Vec<Option<Message>> = vec![self.requests.update(msg.to_owned())];

        match msg {
            Message::Quit => {
                self.state = AppState::Done;
            }
            _ => {}
        };

        return messages;
    }
}
