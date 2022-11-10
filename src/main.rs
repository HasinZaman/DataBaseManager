use std::{thread, time::Duration, collections::HashSet};
use tui::{
    layout::{Layout, Direction, Constraint}
};

use crossterm::{
    execute, terminal::{LeaveAlternateScreen}
};
use ui::{menu::Menu, renderable::Renderable, gen_terminal, input::Input};

use crate::{backend::{relation::{Relation, table::{Table, Attribute, AttributeType, Constraint as TableConstraint}}, query::Query}, ui::pages::relations::{relation_list::RelationListPage, relation_page::RelationPage}};

pub mod ui;
pub mod backend;

fn main() {
    let mut terminal = gen_terminal();
    let _result= terminal.show_cursor();

    let str_tmp = String::from("HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD - HELLO WORLD");
    
    let mut input = Input::from(
        String::from(""),
        Some(String::from("prompt")),
        true
    );

    let mut relations = vec![
        Relation::Table(
            Table {
                name: String::from("Tag"),
                attributes: vec![
                    Attribute{
                        name: String::from("id"),
                        data_type: AttributeType::Int(8),
                        constraint: HashSet::new(),
                    },
                    Attribute{
                        name: String::from("col"),
                        data_type: AttributeType::Char(6),
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                    Attribute{
                        name: String::from("name"),
                        data_type: AttributeType::VarChar(50),
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                    Attribute {
                        name: String::from("tag_type"),
                        data_type: AttributeType::Int(8),
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                ],
                primary_key: Some(0),
            },
        ),
        Relation::Table(
            Table {
                name: String::from("Related"),
                attributes: vec![
                    Attribute{
                        name: String::from("tag_1"),
                        data_type: AttributeType::Int(8),
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(
                                TableConstraint::ForeignKey{
                                    table_name: String::from("Tag"),
                                    attribute_name: String::from("id")
                                }
                            );

                            tmp
                        },
                    },
                    Attribute{
                        name: String::from("tag_2"),
                        data_type: AttributeType::Int(8),
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(
                                TableConstraint::ForeignKey{
                                    table_name: String::from("Tag"),
                                    attribute_name: String::from("id")
                                }
                            );

                            tmp
                        },
                    },
                ],
                primary_key: None,
            },
        ),
        Relation::Table(
            Table {
                name: String::from("Project"),
                attributes: vec![
                    Attribute{
                        name: String::from("repo"),
                        data_type: AttributeType::VarChar(50),
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                    Attribute{
                        name: String::from("start"),
                        data_type: AttributeType::TimeStamp,
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                    Attribute{
                        name: String::from("update"),
                        data_type: AttributeType::TimeStamp,
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                ],
                primary_key: None,
            },
        ),
        Relation::Table(
            Table {
                name: String::from("Dev_log"),
                attributes: vec![
                    Attribute{
                        name: String::from("timestamp"),
                        data_type: AttributeType::TimeStamp,
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                    Attribute{
                        name: String::from("content"),
                        data_type: AttributeType::Text(100),
                        constraint: {
                            let mut tmp: HashSet<TableConstraint> = HashSet::new();
                            tmp.insert(TableConstraint::NotNull);

                            tmp
                        },
                    },
                ],
                primary_key: None,
            },
        ),
    ];

    let relation_list : RelationListPage = RelationListPage::from(&relations);

    let relation_page: RelationPage = RelationPage::new(&relations[1]);

    for x in 0..str_tmp.len(){
        let _result = terminal.draw(|f| {
            let size = f.size();
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);
    
            let mut _menu = Menu::default();
    
            let tmp: Vec<char> = str_tmp.clone().chars().collect();
            input.add_char(tmp.get(x).unwrap().clone());
            
            _menu.render(chunks.get(0).unwrap().clone(), f);
            relation_list.render(chunks.get(1).unwrap().clone(), f);
            //relation_page.render(chunks.get(1).unwrap().clone(), f);
            input.render(chunks.get(2).unwrap().clone(), f);
        });
        
        thread::sleep(Duration::from_millis(50));
    }

    for x in 0..str_tmp.len(){
        let _result = terminal.draw(|f| {
            let size = f.size();
            
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);
    
            let mut _menu = Menu::default();
    
            let tmp: Vec<char> = str_tmp.clone().chars().collect();
            input.add_char(tmp.get(x).unwrap().clone());
            
            _menu.render(chunks.get(0).unwrap().clone(), f);
            relation_list.render(chunks.get(1).unwrap().clone(), f);
            //relation_page.render(chunks.get(1).unwrap().clone(), f);
            input.render(chunks.get(2).unwrap().clone(), f);
        });
        
        thread::sleep(Duration::from_millis(50));
    }
    /*for _x in 0..str_tmp.len()/2{
        let _result = terminal.draw(|f| {
            let size = f.size();
    
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);
    
            let mut _menu = Menu::default();

            input.cursor_left(1);
            
            _menu.render(chunks.get(0).unwrap().clone(), f);
            relation_list.render(chunks.get(1).unwrap().clone(), f);
            input.render(chunks.get(2).unwrap().clone(), f);
        });
        
        thread::sleep(Duration::from_millis(50));
    }
    for _x in 0..str_tmp.len()/2{
        let _result = terminal.draw(|f| {
            let size = f.size();
    
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(size);
    
            let mut _menu = Menu::default();

            input.del_char();
            
            _menu.render(chunks.get(0).unwrap().clone(), f);
            relation_list.render(chunks.get(1).unwrap().clone(), f);
            input.render(chunks.get(2).unwrap().clone(), f);
        });
        
        thread::sleep(Duration::from_millis(50));
    }
    */
    thread::sleep(Duration::from_millis(2500));

    let _result = execute!(terminal.backend_mut(), LeaveAlternateScreen);

    println!("{}", input);
}