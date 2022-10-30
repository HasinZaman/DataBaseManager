use std::io::{self, Stdout};

use crossterm::{execute, terminal::EnterAlternateScreen};
use tui::{Terminal, backend::CrosstermBackend};

pub mod renderable;
pub mod menu;
pub mod input;
pub mod pages;

pub fn gen_terminal() -> Terminal<CrosstermBackend<Stdout>> {
    let mut stdout = io::stdout();

        let _result = execute!(stdout, EnterAlternateScreen);

        let backend = CrosstermBackend::new(stdout);
        match Terminal::new(backend){
            Ok(val) => return val,
            Err(err) => panic!("{}", err)
        };
}