use std::{thread, time::Duration};
use tui::{
    widgets::{Block, Borders},
    style::{Color, Style},
    layout::{Layout, Direction, Constraint}
};

use crossterm::{
    execute, terminal::{LeaveAlternateScreen}
};
use ui::{menu::Menu, renderable::Renderable, gen_terminal, input::Input};

pub mod ui;

fn main() {

    let mut terminal = gen_terminal();
    let _result= terminal.show_cursor();

    let str_tmp = "HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD".to_string();
    
    let mut input = Input::from(
        "".to_string(),
        None,
        true
    );

    for x in 0..str_tmp.len(){
        let _result = terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()        
                .title("Block")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default().bg(Color::Black));
    
    
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);
    
            let mut _menu = Menu::default();
    
            let tmp: Vec<char> = str_tmp.clone().chars().collect();
            input.add_char(tmp.get(x).unwrap().clone());
            
            _menu.render(chunks.get(0).unwrap().clone(), f);
            f.render_widget(block, chunks.get(1).unwrap().clone());
            input.render(chunks.get(2).unwrap().clone(), f);
        });
        
        thread::sleep(Duration::from_millis(50));
    }
    for _x in 0..str_tmp.len()/2{
        let _result = terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()        
                .title("Block")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default().bg(Color::Black));
    
    
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);
    
            let mut _menu = Menu::default();

            input.cursor_left(1);
            
            _menu.render(chunks.get(0).unwrap().clone(), f);
            f.render_widget(block, chunks.get(1).unwrap().clone());
            input.render(chunks.get(2).unwrap().clone(), f);
        });
        
        thread::sleep(Duration::from_millis(50));
    }
    for _x in 0..str_tmp.len()/2{
        let _result = terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()        
                .title("Block")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .style(Style::default().bg(Color::Black));
    
    
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);
    
            let mut _menu = Menu::default();

            input.del_char();
            
            _menu.render(chunks.get(0).unwrap().clone(), f);
            f.render_widget(block, chunks.get(1).unwrap().clone());
            input.render(chunks.get(2).unwrap().clone(), f);
        });
        
        thread::sleep(Duration::from_millis(50));
    }
    
    thread::sleep(Duration::from_millis(2500));

    let _result = execute!(terminal.backend_mut(), LeaveAlternateScreen);

    print!("{}", input);
}