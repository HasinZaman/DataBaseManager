use std::{io::Stdout, sync::Mutex};

use regex::Regex;
use lazy_static::lazy_static;

use backend::relation::Relation;
use crossterm::event::{self, Event};
use tui::{Terminal, backend::{CrosstermBackend}, layout::{Layout, Direction, Constraint}};
use ui::{input::Input, gen_terminal, menu::Menu, renderable::Renderable, pages::{schema::{relation_list::RelationListPage, relation_page::RelationPage}, snapshot::SnapShotPage}};

use crate::{ui::pages::{Pages, query::QueryPage}, backend::{sql::SQL, data_base::DatabaseExecute}};

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

static mut PAGE_SIZE: u16 = 0;

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

        unsafe {
            PAGE_SIZE = chunks[1].height;
        }
        

        menu.render(chunks[0].clone(), f);
        page_content.render(chunks[1].clone(), f);
        input.render(chunks[2].clone(), f);
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
    else if let Ok(SQL::Select(query)) = SQL::from(&cmd) {
        if let Ok(_) = query.execute(|_| ()) {
            menu.select(1).unwrap();

            let mut page = QueryPage::new(&query);
            let size: usize = unsafe {
                PAGE_SIZE.clone() as usize - 3usize
            };

            page.update(size);

            let mut last_page = LAST_PAGE.lock().unwrap();
            *last_page = Pages::Query(page);
        }
    }
    else if cmd.to_ascii_lowercase() == "snapshot" {
        menu.select(2).unwrap();

        let mut last_page = LAST_PAGE.lock().unwrap();
        *last_page = Pages::SnapShot(SnapShotPage::default());
    }
    else if let Ok(sql) = SQL::from(&cmd) {
        let _result =sql.execute(|_| ());
    }
    else {
        match menu.get_tab(){
            ui::menu::Tab::Schema => {
                //no
            },
            ui::menu::Tab::Query => {
                let mut last_page = LAST_PAGE.lock().unwrap();

                match &*last_page {
                    Pages::Query(query) => {
                        
                        let mut next_page: QueryPage = query.clone();
                    
                        let cmd = cmd.to_ascii_lowercase();
                        let cmd: &str = &cmd;
                        match cmd {
                            "next" => {
                                let size: usize = unsafe {
                                    PAGE_SIZE.clone() as usize - 3usize
                                };
                                next_page.next_row(size).update(size);
                            },
                            "prev" => {
                                let size: usize = unsafe {
                                    PAGE_SIZE.clone() as usize - 3usize
                                };
                                next_page.prev_row(size).update(size);
                            }
                            _=> {
                            }
                        };

                        *last_page = Pages::Query(next_page);
                    },
                    _=>{}
                };
            },
            ui::menu::Tab::SnapShot => {
                let cmd = cmd.to_ascii_lowercase();
                let cmd: &str = &cmd;

                lazy_static!{
                    static ref ADD_SNAPSHOT : Regex = Regex::new("^[Aa][Dd][Dd]$").unwrap();
                };
                lazy_static!{
                    static ref ADD_SNAPSHOT_FILE : Regex = Regex::new("^[Aa][Dd][Dd] (.+)$").unwrap();
                };
                lazy_static!{
                    static ref REMOVE_SNAPSHOT : Regex = Regex::new("^[Rr][Ee][Mm][Oo][Vv][Ee] (.+)$").unwrap();
                };
                lazy_static!{
                    static ref ROLLBACK_SNAPSHOT : Regex = Regex::new("^[Rr][Oo][Ll][Ee][Bb][Aa][Cc][Kk] (.+)$").unwrap();
                };

                

                
                //add snapshot
                //remove snapshot
                //rollback to
            },
        }
    }
    
}
