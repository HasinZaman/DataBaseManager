use super::renderable::{Renderable};
use strum::{EnumCount};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use tui::{
    widgets::{Tabs, Block, Borders},
    text::{Span, Spans},
    style::{Style, Color}, layout::Rect, Frame, backend::CrosstermBackend, symbols::DOT
};
use std::fmt;

/// Tab enum define all tab types
#[derive(Clone, Copy, Debug, EnumCountMacro, EnumIter, PartialEq, Eq)]
pub enum Tab {
    Schema,
    Query,
    SnapShot
}
impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tab::Schema => write!(f, "Schema"),
            Tab::Query => write!(f, "Query"),
            Tab::SnapShot => write!(f, "SnapShot"),
        }
    }
}

/// Menu struct defines state required to maintain a menu widget
pub struct Menu {
    tabs: [Tab;3],
    selected: usize,
}

impl Menu {
    /// new associative function generates menu - with selected tab
    pub fn new(selected: usize) -> Menu {
        if Tab::COUNT <= selected {
            panic!("selected must be between [0,{})", Tab::COUNT);
        }

        Menu {
            tabs: [
                Tab::Schema,
                Tab::Query,
                Tab::SnapShot
            ],
            selected: selected
        }
    }

    /// next method moves selected tab to the left
    pub fn next(&mut self) -> &Tab {

        self.selected= (self.selected+1) % Tab::COUNT;

        &self.tabs[self.selected]
    }
    
    /// prev method moves selected tab to the right
    pub fn prev(&mut self) -> &Tab {
        self.selected = match self.selected {
            0 => Tab::COUNT - 1,
            i => i - 1
        };

        &self.tabs[self.selected]
    }

    /// select method changes selected tab to the inputted select index
    pub fn select(&mut self, select: usize) -> Option<&Tab> {
        if Tab::COUNT <= select {
            return Option::None
        }

        self.selected = select;
        Option::Some(&self.tabs[self.selected])
    }

    pub fn get_tab(&self) -> &Tab{
        &self.tabs[self.selected]
    }
}
impl Default for Menu{
    fn default() -> Menu {
        Menu::new(0)
    }
}

impl Renderable for Menu {
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let titles : Vec<Spans> = vec![{
            let mut titles: Vec<Span> = self.tabs
                .into_iter()
                .map(
                    |t| {
                        Span::styled(
                            format!(" {} ", t.to_string()),
                            Style::default().bg(Color::Black).fg(Color::White)
                        )
                    }
                )
                .collect();

            titles.get_mut(self.selected)
                .unwrap()
                .style = Style::default().bg(Color::White).fg(Color::Black);

            Spans::from(titles)
        }];
        
        frame.render_widget(
            Tabs::new(titles)
                .block(
                    Block::default()
                        .title("Tabs")
                        .borders(Borders::ALL)
                    )
                .divider(DOT),
            display_area
        );
    }
}