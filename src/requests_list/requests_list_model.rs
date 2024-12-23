use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
    Frame,
};

use crate::app::Message;

use super::request::Request;

pub struct RequestsListModel {
    items: Vec<Request>,
    state: ListState,
}

impl RequestsListModel {
    pub fn new(items: Vec<Request>) -> Self {
        let mut state = ListState::default();
        state.select_first();

        return RequestsListModel { items, state };
    }

    pub fn view(&mut self, frame: &mut Frame, rect: Rect, block: Block) {
        let list = List::new(
            self.items
                .iter()
                .map(|i| ListItem::from(i))
                .collect::<Vec<ListItem>>(),
        )
        .highlight_style(Style::new().reversed())
        .highlight_spacing(HighlightSpacing::Always)
        .block(block);

        StatefulWidget::render(list, rect, frame.buffer_mut(), &mut self.state);
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Down => {
                self.state.select_next();
            }
            Message::Up => {
                self.state.select_previous();
            }
            _ => {}
        }

        return None;
    }
}
