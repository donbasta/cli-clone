use ratatui::{prelude::*, widgets::*};

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

#[derive(Debug)]
pub enum CurrentlyEditing {
    Name,
    Hour,
    Minute,
    Second,
}

#[derive(Debug)]
pub struct Timer {
    pub name: String,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Timer {
    pub fn to_list_item(&self) -> ListItem {
        ListItem::new(Line::from(Span::styled(
            format!("{: <25}", self.name),
            Style::default().fg(Color::Yellow),
        )))
    }
}

#[derive(Debug)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Timer>,
    pub last_selected: Option<usize>,
}

impl StatefulList {
    fn new() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::new(),
            last_selected: None,
        }
    }
    fn add(&mut self, new_timer: Timer) {
        self.items.push(new_timer);
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
    fn set_last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
    pub fn get_selected_index(&self) -> Option<usize> {
        return self.state.selected();
    }
}

#[derive(Debug)]
pub struct App {
    pub name_input: String,
    pub hour_input: String,
    pub minute_input: String,
    pub second_input: String,

    pub timers: StatefulList,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,

    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            name_input: String::new(),
            hour_input: String::new(),
            minute_input: String::new(),
            second_input: String::new(),

            timers: StatefulList::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,

            running: true,
        }
    }

    pub fn save_new_timer(&mut self) {
        self.timers.add(Timer {
            name: self.name_input.clone(),
            hour: self.hour_input.clone().parse::<u32>().unwrap(),
            minute: self.minute_input.clone().parse::<u32>().unwrap(),
            second: self.second_input.clone().parse::<u32>().unwrap(),
        });

        self.name_input = String::new();
        self.hour_input = String::new();
        self.minute_input = String::new();
        self.second_input = String::new();
        self.currently_editing = None;
        self.timers.set_last();
    }

    pub fn tick(&self) {}

    pub fn exit(&mut self) {
        self.running = false;
    }
}
