use std::{io, thread, time::Duration};
use tui::{
    widgets::{Block, Borders, BorderType},
    style::{Color, Style},
    layout::{Rect, Layout, Direction, Constraint}
};

use crossterm::{
    execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};
use ui::{menu::Menu, renderable::Renderable, gen_terminal};

pub mod ui;

fn main() {

    let mut terminal = gen_terminal();
    
    let _result = terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()        
            .title("Block")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black));


        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(size);

        let mut _menu = Menu::default();

       _menu.render(chunks.get(0).unwrap().clone(), f);

        f.render_widget(block, chunks.get(1).unwrap().clone());
    });
    
    thread::sleep(Duration::from_millis(5000));

    let _result = execute!(terminal.backend_mut(), LeaveAlternateScreen);
}