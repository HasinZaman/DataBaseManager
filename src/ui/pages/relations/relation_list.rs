use std::{collections::HashMap, u8, cmp::max};

use tui::{
    Frame,
    style::{Color, Style},
    widgets::{Table, Row, Cell, Block, Borders},
    backend::CrosstermBackend,
    layout::{Rect, Constraint}, text::{Spans, Span}
};

use crate::{ui::{renderable::Renderable}, backend::{query::Query, relation::table}};
use crate::backend::relation::Relation;

/// rand_col function returns a random colour
fn rand_col() -> Color {
    let range = 2;
    Color::Rgb(
        rand::random::<u8>() / range + 255 / range,
        rand::random::<u8>() / range + 255 / range,
        rand::random::<u8>() / range + 255 / range,
    )
}

/// RelationListPage struct defines the states required in-order to maintain a Relation List Page on the relation tab
#[derive(Debug)]
pub struct RelationListPage<'a>{
    relations: &'a Vec<Relation>,
    col_map: HashMap<(String, String), Color>

}

impl <'a>RelationListPage<'a> {
    /// from associative function defines RelationListPage from a vector of Relations
    pub fn from(relations: &'a Vec<Relation>) -> RelationListPage<'a> {
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
                        Relation::View(_query)=> {
                            return false;
                            /*match query{
                                Query::Select(cmd) => {
                                    todo!()
                                },
                                _ => false,
                            }*/
                        },
                    }
                }
            ).for_each(
                |relation| {
                    match relation {
                        Relation::Table(table) => {
                            col_map.insert(
                                (
                                    table.name.clone(),
                                    table.attributes.get(table.primary_key.unwrap()).unwrap().name.clone()
                                ),
                                rand_col()
                            );
                        },
                        Relation::View(_query) => {
                            todo!()
                        }
                    }
                }
            );

        RelationListPage{ col_map: col_map, relations: &relations }
    }

}

impl <'a>Renderable for RelationListPage<'a>{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let mut column_length : (u16, u16, u16) = (5, "Name".len() as u16, "Definition".len() as u16);

        //define rows of table
        let rows: Vec<Row> = self.relations.iter()
            .map(|r| {
                    Row::new(
                        {
                            //column = [relation type, relation name, relation definition]
                            let mut column: [Cell; 3] = [Cell::from(""),Cell::from(""),Cell::from("")];

                            match r {
                                Relation::Table(table) => {
                                    column[0] = Cell::from("Table");
                                    column[1] = Cell::from({
                                        column_length.1 = max(column_length.1, table.name.len() as u16);
                                        table.name.clone()
                                    });
                                    column[2] = Cell::from({
                                        let spans: Spans = Spans::from({
                                            let spans_vec: Vec<Span> = {
                                                let mut spans_vec: Vec<Span> = Vec::new();
                                                table.attributes.iter()
                                                    .enumerate()
                                                    .map(
                                                        |(index,a)| {
                                                            Span::styled(
                                                                a.to_string(),
                                                                match table.primary_key {
                                                                    //apply primary key style
                                                                    Some(primary_key) => {
                                                                        if primary_key == index {
                                                                            Style::default()
                                                                            .fg(Color::Black)
                                                                            .bg(
                                                                            self.col_map
                                                                                .get(
                                                                                    &(
                                                                                        table.name.clone(),
                                                                                        a.name.clone()
                                                                                    )
                                                                                )
                                                                                .unwrap()
                                                                                .clone()
                                                                            )
                                                                        }
                                                                        else {
                                                                            Style::default()
                                                                        }
                                                                    },
                                                                    //check for foreign key style
                                                                    None => {
                                                                        let foreign_key = a.constraint
                                                                            .iter()
                                                                            .filter(|a| {
                                                                                if let table::Constraint::ForeignKey{..} = a {
                                                                                    return true;
                                                                                }
                                                                                return false;
                                                                            })
                                                                            .nth(0);
                                                                        
                                                                        match foreign_key {
                                                                            Some(val) => {
                                                                                match val {
                                                                                    table::Constraint::ForeignKey{table_name, attribute_name} => {
                                                                                        Style::default()
                                                                                            .fg(Color::Black)
                                                                                            .bg(
                                                                                                self.col_map.get(
                                                                                                    &(
                                                                                                        table_name.clone(),
                                                                                                        attribute_name.clone()
                                                                                                    )
                                                                                                ).unwrap()
                                                                                                .clone()
                                                                                            )
                                                                                    },
                                                                                    _ => panic!()
                                                                                }
                                                                            },
                                                                            None => Style::default()
                                                                        }
                                                                    }
                                                                }
                                                            )
                                                        }
                                                    )
                                                    .for_each(//add "," between attributes
                                                        |s| {
                                                            spans_vec.push(s);
                                                            spans_vec.push(Span::from(", "))
                                                        }
                                                    );

                                                spans_vec.pop();
                                                spans_vec
                                            };

                                            spans_vec
                                        });
                                        column_length.2 = max(column_length.2, spans.width() as u16);
                                        spans
                                    });
                                },
                                Relation::View(_view) => {
                                    column[0] = Cell::from("View");
                                    column[1] = Cell::from({
                                        todo!();
                                        ""
                                    });
                                    column[2] = Cell::from(
                                        Spans::from(
                                            {
                                                todo!();
                                                ""
                                            }
                                            // if let Query::Select(query) = _view {
                                            //     query.clone()
                                            // }
                                            // else {
                                            //     panic!()
                                            // }
                                        )
                                    );
                                }
                            }

                            column
                        }
                    )
                }
            ).collect();

        let widths = [
            Constraint::Length(column_length.0),
            Constraint::Length(column_length.1),
            Constraint::Min(column_length.2)
        ];

        let table = Table::new(rows)
            .header(
                Row::new(
                    vec![
                        Cell::from(String::from("Type")),
                        Cell::from(String::from("Name")),
                        Cell::from(String::from("Definition")),
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