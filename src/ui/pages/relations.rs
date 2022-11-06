use std::{collections::HashMap, u8, cmp::max};

use tui::{
    Frame,
    style::{Color, Style, Modifier},
    widgets::{Table, Row, Cell, Block, Borders},
    backend::CrosstermBackend,
    layout::{Rect, Constraint}, text::{Spans, Span}
};

use rand::prelude::*;

use crate::{ui::renderable::Renderable, backend::{query::Query}};
use crate::backend::relation::Relation;

fn rand_col() -> Color {
    Color::Rgb(
        rand::random::<u8>() / 2,
        rand::random::<u8>() / 2,
        rand::random::<u8>() / 2,
    )
}

pub struct RelationListPage{
    relations: Vec<Relation>,
    col_map: HashMap<String, Color>

}

impl RelationListPage {
    pub fn from(relations: Vec<Relation>) -> RelationListPage {
        let mut col_map = HashMap::new();

        relations.clone().into_iter()
            .filter(
                |relation| {
                    match relation {
                        Relation::Table(table) => {
                            match table.primary_key {
                                Some(_) => true,
                                None => false
                            }
                        },
                        Relation::View(query)=> {
                            return false;
                            match query{
                                Query::Select(cmd) => {
                                    todo!()
                                },
                                _ => false,
                            }
                        },
                    }
                }
            ).for_each(
                |relation| {
                    match relation {
                        Relation::Table(table) => {
                            col_map.insert(
                                table.attributes.get(table.primary_key.unwrap()).unwrap().name.clone(),
                                rand_col()
                            );
                        },
                        Relation::View(_query) => {
                            todo!()
                        }
                    }
                }
            );

        RelationListPage{ col_map: col_map, relations: relations }
    }

    
}


impl Renderable for RelationListPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let mut col_len : (u16, u16, u16) = (5, "Name".len() as u16, "Definition".len() as u16);

        let rows: Vec<Row> = self.relations.iter()
            .map(|r| {
                    Row::new(
                        vec![
                            Cell::from(
                                match r {
                                    Relation::Table(table) => format!("Table"),
                                    Relation::View(view) => format!("View")
                                }
                            ),
                            Cell::from(
                                match r {
                                    Relation::Table(table) => {
                                        col_len.1 = max(col_len.1, table.name.len() as u16);
                                        table.name.clone()
                                    },
                                    Relation::View(view) => todo!()
                                }
                            ),
                            Cell::from(
                                {
                                    let spans: Spans = match r {
                                        Relation::Table(table) => {
                                            Spans::from(
                                                {
                                                    let spans_vec: Vec<Span> = table.attributes.iter()
                                                    .map(|a| Span::from(a.name.clone()))
                                                    .collect();

                                                    if let Some(key) = table.primary_key {
                                                        spans_vec.get(key).unwrap().style.bg(
                                                            self.col_map.get(
                                                                &table.attributes
                                                                .get(key)
                                                                .unwrap()
                                                                .name
                                                                .clone()
                                                            ).unwrap()
                                                            .clone()
                                                        );
                                                    }

                                                    spans_vec
                                                }
                                            )
                                        },
                                        Relation::View(view) => {
                                            Spans::from(
                                                if let Query::Select(query) = view {
                                                    query.clone()
                                                }
                                                else {
                                                    panic!()
                                                }
                                            )
                                        }
                                    };
                                    col_len.2 = max(col_len.2, spans.width() as u16);
                                    spans
                                }
                            ),
                        ]
                    )
                }
            ).collect();

        let widths = [
            Constraint::Length(col_len.0),
            Constraint::Length(col_len.1),
            Constraint::Min(col_len.2)
        ];

        let table = Table::new(rows)
            .header(
                Row::new(
                    vec![
                        Cell::from("Type".to_string()),
                        Cell::from("Name".to_string()),
                        Cell::from("Definition".to_string()),
                    ]
                )
            )
            .widths(&widths).column_spacing(3)
            .block(
                Block::default()
                .title("Relations")
                .borders(Borders::ALL)
            );
        
        frame.render_widget(table, display_area);
    }
}

pub struct RelationPage{
    pub relation: Relation
}

impl RelationPage {
    fn new(relation: Relation) -> RelationPage {
        RelationPage { relation: relation }
    }
}

impl Renderable for RelationPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}