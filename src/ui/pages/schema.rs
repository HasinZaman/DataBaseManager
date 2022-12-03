use std::cmp::{max, min};

use regex::{Regex, Captures};
use lazy_static::lazy_static;

use tui::{Frame,backend::CrosstermBackend, layout::{Rect, Constraint}, widgets::{Table, Row, Cell, Block, Borders},};

use crate::{ui::renderable::Renderable, backend::{data_base::DataBase, sql::{QDL, SQL}}};

struct QueryCache{
    pub start_col: usize,
    pub columns: Vec<(String, usize)>,
    pub rows: Vec<Vec<String>>
}

pub struct QueryPage<'a>{
    query: &'a QDL,
    query_offset: usize,
    query_cache: QueryCache
}

impl<'a> QueryPage<'a> {
    pub fn new(query: &'a QDL) -> QueryPage<'a> {
        QueryPage {
            query: query,
            query_offset: 0,
            query_cache: QueryCache {
                start_col: 0,
                columns: Vec::new(),
                rows: Vec::new()
            }
        }
    }

    pub fn next_row(&mut self, skip: usize) -> &mut Self{
        self.query_offset += skip;

        self
    }
    
    pub fn prev_row(&mut self, skip: usize) -> &mut Self{
        match self.query_offset.checked_sub(skip) {
            Some(new_offset) => self.query_offset = new_offset,
            None => self.query_offset = 0usize,
        }

        self
    }

    pub fn start_col(&mut self, start_col: usize) -> &mut Self{
        self.query_cache.start_col = start_col;
        self
    }

    pub fn update(&mut self, row_count: usize) -> &mut Self{
        match DataBase::from_env() {
            Ok(db) => {

                let mut columns: Vec<(String, usize)> = Vec::new();
                let mut rows: Vec<Vec<String>> = Vec::new();

                let _result: Result<Vec<Option<bool>>, _> = db.execute(
                    &SQL::Select(self.get_query(row_count)),
                    |row| {
                        let row = row.unwrap();

                        let mut row_tmp: Vec<String> = Vec::new();

                        row.clone()
                            .unwrap()
                            .iter()
                            .zip(
                                row.columns()
                                    .iter()
                            )
                            .enumerate()
                            .map(|(i1, (val, col))| (i1, val, col))
                            .for_each(|(i1, val, col)| {
                                let val = format!("{:?}", val);

                                match columns.get(i1) {
                                    Some((_name, max_size)) => {
                                        columns[i1].1 = max(*max_size, val.len());
                                    },
                                    None => {
                                        columns.push({
                                            let col_name = col.name_str();
                                            (
                                                col_name.to_string(),
                                                max(
                                                    col_name.len(),
                                                    val.len()
                                                )
                                                
                                            )
                                        })
                                    }
                                }
                                
                                row_tmp.push(val);
                            });
                        
                        rows.push(row_tmp);

                        None
                    }
                );

                self.query_cache = QueryCache {
                    start_col: min(self.query_cache.start_col, columns.len()-1),
                    columns: columns,
                    rows: rows
                }
            },
            Err(_err) => {
                todo!()
            }
        }

        self
    }

    fn get_query(&self, row_count: usize) -> QDL {
        lazy_static! {
            static ref EXIST_OPERATION_REGEX : Regex = Regex::new("^[Ss][Ee][Ll][Ee][Cc][Tt] .+ [Ll][Ii][Mm][Ii][Tt] (\\d+),?([\\d]+)?$").unwrap();
        };

        let tmp_query = self.query.to_string();

        return QDL(
            match EXIST_OPERATION_REGEX.is_match(&tmp_query) {
                true => {
                    let captures = EXIST_OPERATION_REGEX.captures(&tmp_query).unwrap();
                
                    EXIST_OPERATION_REGEX.replace(
                        &tmp_query,
                        |_cap: &Captures| {
                            format!(
                                "{}{},{}",
                                &tmp_query[
                                    0
                                    ..
                                    {
                                        let tmp = captures.get(1).unwrap();

                                        tmp.start()
                                    }
                                ],
                                self.query_offset,
                                row_count
                            )
                        }
                    ).to_string()
                }
                false => {
                    format!("{} LIMIT {},{}", tmp_query, self.query_offset, row_count)
                }
            }
        )
    }

    pub fn len(&self) -> (usize, usize) {
        (self.query_cache.columns.len(), self.query_cache.rows.len())
    }
}

impl<'a> Renderable for QueryPage<'a>{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let height: usize = (display_area.height - 2 - 1) as usize; 

        let constraints : Vec<Constraint>;

        let table: Table = Table::new(
            self.query_cache
                .rows
                .iter()
                .map(
                    |columns| {
                        Row::new({
                                let cells: Vec<Cell> = columns.iter()
                                    .skip(self.query_cache.start_col)
                                    .map(
                                        |col| {
                                            Cell::from(col.to_string())
                                        }
                                    ).collect();

                                cells
                            }
                        )
                    }
                )
        ).header(
            Row::new({
                let tmp: Vec<Cell> = self.query_cache
                    .columns
                    .iter()
                    .skip(self.query_cache.start_col)
                    .map(|name| Cell::from(name.0.to_string()))
                    .collect();

                    tmp
                }
            )
        ).widths({
                constraints = self.query_cache
                    .columns
                    .iter()
                    .skip(self.query_cache.start_col)
                    .map(|col| Constraint::Length(col.1 as u16))
                    .collect();

                &constraints
            }
        ).column_spacing(3)
        .block(
            Block::default()
            .title(format!(
                "Query: {}",
                self.get_query(height).to_string()
            ))
            .borders(Borders::ALL)
        );


        frame.render_widget(table, display_area);
    }
}