use std::{io::Stdout, sync::Mutex};

use regex::Regex;
use lazy_static::lazy_static;

use backend::relation::Relation;
use crossterm::event::{self, Event};
use tui::{Terminal, backend::{CrosstermBackend}, layout::{Layout, Direction, Constraint}};
use ui::{input::Input, gen_terminal, menu::Menu, renderable::Renderable, pages::{relations::{relation_list::RelationListPage, relation_page::RelationPage}, snapshot::SnapShotPage}};

use crate::ui::pages::Pages;

pub mod ui;
pub mod backend;

lazy_static!{
    static ref RELATIONS: Mutex<Vec<Relation>> = Mutex::new(Relation::get_relations().unwrap());
}
lazy_static!(
    static ref LAST_PAGE: Mutex<Pages> = Mutex::new(
        Pages::RelationList(
            RelationListPage::from(
                &RELATIONS.lock().unwrap()
            )
        )
    );
);

fn main() {
    let mut terminal = gen_terminal();
    let _result= terminal.show_cursor();

    let mut input = Input::default();
    let mut menu = Menu::default();

    update_terminal(&mut terminal, &menu, &LAST_PAGE.lock().unwrap(), &input);
    loop {
        if let Ok(Event::Key(event)) = event::read() {
            if let Some(cmd) = input.from_event(event) {
                get_cmd(cmd, &mut menu);
            }
            update_terminal(&mut terminal, &menu, &LAST_PAGE.lock().unwrap(), &input);
        }
    }
}

fn update_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>, menu: &Menu, page_content: &Pages, input: &Input ) {
    let _result = terminal.draw(|f| {
        let size = f.size();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
            .split(size);

        
        menu.render(chunks.get(0).unwrap().clone(), f);
        page_content.render(chunks.get(1).unwrap().clone(), f);
        input.render(chunks.get(2).unwrap().clone(), f);
    });
}

fn get_cmd(cmd: String, menu: &mut Menu) {
    lazy_static!{
        static ref SCHEMA_TAB : Regex = Regex::new("[Ss][Hh][Oo][Ww] (.+)").unwrap();
    };
    if SCHEMA_TAB.is_match(&cmd) {
        let capture = SCHEMA_TAB.captures(&cmd).unwrap().get(1).unwrap().as_str();
        let relations = RELATIONS.lock().unwrap();

        menu.select(0).unwrap();

        lazy_static!{
            static ref ALL_SCHEMA : Regex = Regex::new("^\\*$").unwrap();
        };
        lazy_static!{
            static ref ALL_TABLES : Regex = Regex::new("^[Tt][Aa][Bb][Ll][Ee][Ss]$").unwrap();
        };
        lazy_static!{
            static ref ALL_VIEWS : Regex = Regex::new("^[Vv][Ii][Ee][Ww][Ss]$").unwrap();
        };

        if ALL_SCHEMA.is_match(capture) {
            //println!("All tarbles");
            let mut last_page = LAST_PAGE.lock().unwrap();
            *last_page = Pages::RelationList(RelationListPage::from(&relations));
        }
        else if ALL_TABLES.is_match(capture) {
            let relations: Vec<Relation> = relations.iter()
                .filter(
                    |relation| {
                    match relation{
                        Relation::Table(_) => true,
                        Relation::View(_) => false,
                    }
                })
                .map(|relation| relation.clone())
                .collect();
            
            let mut last_page = LAST_PAGE.lock().unwrap();
            *last_page = Pages::RelationList(RelationListPage::from(&relations));
        }
        else if ALL_VIEWS.is_match(capture) {
            let relations: Vec<Relation> = relations.iter()
                .filter(
                    |relation| {
                    match relation{
                        Relation::Table(_) => false,
                        Relation::View(_) => true,
                    }
                })
                .map(|relation| relation.clone())
                .collect();
            
            let mut last_page = LAST_PAGE.lock().unwrap();
            *last_page = Pages::RelationList(RelationListPage::from(&relations));
        }
        else {
            let relations: Vec<Relation> = relations.iter()
                .map(
                    |relation| relation.name()
                ).enumerate()
                .filter(|(_index, relation)| relation == capture)
                .map(|(index, _relation)| relations[index].clone())
                .collect();
            
            if relations.len() == 1 {
                let mut last_page = LAST_PAGE.lock().unwrap();
                *last_page = Pages::Relation(RelationPage::new(&relations[0]));
            }
        }
    }
    //select * -> straight to relation
    //else do schema specific commands
    
}