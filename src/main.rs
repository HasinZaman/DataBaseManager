use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    Terminal, widgets::{Block, Borders, BorderType}, text::Span, style::{Color, Style}, layout::Rect
};

use crossterm::{
    execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};

pub mod ui;

fn main() {
    let mut stdout = io::stdout();

    let _result = execute!(stdout, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal;
    
    match Terminal::new(backend){
        Ok(val) => {
            terminal = val
        }
        Err(err) => panic!("{}", err)
    }

    let _result = terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()        
            .title("Block")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black));
        f.render_widget(block, size);

        let size = Rect::new(
            size.x,
            size.y,
            size.width/2,
            size.height/2
        );
        let block = Block::default()        
            .title("Block")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Black));
        f.render_widget(block, size);
    });
    
    thread::sleep(Duration::from_millis(5000));

    let _result = execute!(terminal.backend_mut(), LeaveAlternateScreen);
}