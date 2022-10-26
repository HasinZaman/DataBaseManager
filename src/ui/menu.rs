use super::renderable::Renderable;
use strum::{EnumCount};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};
use tui::{
    widgets::{Widget, Tabs},
    text::{Span, Spans},
    style::{Style, Color}
};
use std::fmt;

#[derive(Clone, Copy)]
#[derive(Debug, EnumCountMacro, EnumIter)]
pub enum Tab {
    Schema,
    Relation,
    SnapShot
}

impl Tab{
}
impl Default for Tab {
    fn default() -> Tab {
        Tab::Schema
    }
}
impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tab::Schema => write!(f, "Schema"),
            Tab::Relation => write!(f, "Relation"),
            Tab::SnapShot => write!(f, "SnapShot"),
        }
    }
}

pub struct Menu {
    tabs: [Tab;3],
    selected: usize,
}

impl Menu {
    fn new(selected: usize) -> Menu {
        if(Tab::COUNT <= selected) {
            panic!("selected must be between [0,{})", Tab::COUNT);
        }

        Menu {
            tabs: [
                Tab::Schema,
                Tab::Relation,
                Tab::SnapShot
            ],
            selected: selected
        }
    }

    fn next(&mut self) -> &Tab {

        self.selected= (self.selected+1) % Tab::COUNT;

        &self.tabs[self.selected]
    }
    
    fn prev(&mut self) -> &Tab {
        self.selected = match self.selected {
            0 => Tab::COUNT - 1,
            i => i - 1
        };

        &self.tabs[self.selected]
    }

    fn select(&mut self, select: usize) -> Option<&Tab> {
        if(Tab::COUNT <= select) {
            return Option::None
        }

        self.selected = select;
        Option::Some(&self.tabs[self.selected])
    }
}

impl Renderable for Menu {
    fn render(&self) -> Vec<Box<dyn Widget>> {
        let titles : Vec<Spans> = vec![{
            let mut titles: Vec<Span> = self.tabs
                .into_iter()
                .map(
                    |t| {
                        Span::styled(
                            t.to_string(),
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

        vec![Box::new(Tabs::new(titles))]
    }
}