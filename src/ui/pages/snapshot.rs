use std::{cmp::{max, min}, mem, path::{ PathBuf}};

use tui::{layout::{Constraint, Rect}, Frame, backend::CrosstermBackend, widgets::{Table, Row, Cell, Block, Borders}, text::Span};

use crate::{ui::renderable::Renderable, backend::{snapshot::{SnapShotsFile, SnapShot}, data_base::DataBase, sql::SQL}};

pub struct SnapShotPage{
    snap_shots: Vec<SnapShot>,
    offset: usize,
}

impl SnapShotPage{

    pub fn next(&mut self, offset: usize) {
        self.offset = min(self.offset + offset, self.snap_shots.len() - 1);
    }

    pub fn prev(&mut self, offset: usize) {
        if let Some(val) = self.offset.checked_sub(offset) {
            self.offset = val;
        }
    }

    pub fn update(&mut self){
        let mut snap_shot = SnapShotPage::default();
        snap_shot.next(self.offset);


        *self = mem::take(&mut snap_shot);
    }

    pub fn add(&mut self, snapshot: SnapShot) -> & mut Self{
        let mut file = SnapShotsFile::default();
        file.add_snapshot(snapshot);

        self.update();

        self
    }

    fn del_at_index(&mut self, index: usize) -> &mut Self{
        if self.snap_shots.len() < index {
            return self;
        }
        
        self.snap_shots.remove(index);

        SnapShotsFile::default().replace_snapshots(&self.snap_shots);

        self.update();

        self
    }

    fn del_by_name(&mut self, identifier: &str) -> &mut Self {
        let index = self.snap_shots
            .iter()
            .map(|snapshot| PathBuf::from(&snapshot.path))
            .enumerate()
            .find(|file_path| file_path.1.file_name().unwrap() == identifier || file_path.1.to_str().unwrap() == identifier);
        
        if index == None {
            return self;
        }

        let index = index.unwrap().0;


        self.del_at_index(index)
    }

    pub fn del(& mut self, identifier: &str) {

        match usize::from_str_radix(identifier, 10) {
            Ok(index) => self.del_at_index(index),
            Err(_) => self.del_by_name(identifier),
        };
    }

    fn rollback_by_name(&self, identifier: &str) {
        let index = self.snap_shots
            .iter()
            .map(|snapshot| PathBuf::from(&snapshot.path))
            .enumerate()
            .find(|file_path| file_path.1.file_name().unwrap() == identifier || file_path.1.to_str().unwrap() == identifier);
        
        if index == None {
            return ;
        }

        let index = index.unwrap().0;


        self.rollback_by_index(index)
    }

    fn rollback_by_index(& self, index: usize) {
        if self.snap_shots.len() < index {
            return ;
        }

        let db = DataBase::from_env().unwrap();

        let _result = db.rollback(SQL::from_file(&self.snap_shots[index].path).unwrap());
    }

    pub fn rollback(&self, identifier: &str) {
        match usize::from_str_radix(identifier, 10) {
            Ok(index) => self.rollback_by_index(index),
            Err(_) => self.rollback_by_name(identifier),
        }
    }
}

impl Default for SnapShotPage {
    fn default() -> Self {
        let file = SnapShotsFile::default();


        let mut snap_shots: Vec<SnapShot> = file.snap_shots
            .iter()
            .map(|s| s.clone())
            .collect();

        
        snap_shots
            .sort_by(|a, b| a.time_stamp.partial_cmp(&b.time_stamp).unwrap());

        SnapShotPage{
            snap_shots: snap_shots,
            offset: 0
        }
    }
}

impl Renderable for SnapShotPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        const HEADERS: [&str;3] = ["", "TimeStamp", "Path"];
        let mut widths: [Constraint;3] = [
            Constraint::Length(HEADERS[0].len() as u16),
            Constraint::Length(HEADERS[1].len() as u16),
            Constraint::Length(HEADERS[2].len() as u16)
        ];

        let table: Table = Table::new(
                {
                    let rows: Vec<Row> = self.snap_shots
                        .iter()
                        .enumerate()
                        .skip(self.offset)
                        .map(|(i, snapshot)| {
                            Row::new(
                                vec![
                                    Cell::from(
                                        Span::from(
                                            {
                                                let index_str: String = format!("{}", i);

                                                widths[0] = match widths[0] {
                                                    Constraint::Length(l) => {
                                                        Constraint::Length(max(l, index_str.len() as u16))
                                                    }
                                                    _ => {
                                                        panic!()
                                                    }
                                                };

                                                index_str
                                            }
                                        )
                                    ),
                                    Cell::from(
                                        Span::from(
                                            {
                                                let timestamp = snapshot.time_stamp;
                                                let tmp_str = format!(
                                                    "{}-{}-{} {}:{}",
                                                    timestamp.month(),
                                                    timestamp.day(),
                                                    timestamp.year(),

                                                    timestamp.hour(),
                                                    timestamp.minute()
                                                );

                                                widths[1] = match widths[1] {
                                                    Constraint::Length(l) => {
                                                        Constraint::Length(max(l, tmp_str.len() as u16))
                                                    }
                                                    _ => {
                                                        panic!()
                                                    }
                                                };

                                                tmp_str
                                            }
                                        )
                                    ),
                                    Cell::from(
                                        Span::from(
                                            {
                                                let path: PathBuf = snapshot.path.clone().into();

                                                let file_name = &path.file_name().unwrap().to_str().unwrap();

                                                let tmp_str = file_name.to_string();

                                                widths[2] = match widths[2] {
                                                    Constraint::Length(l) => {
                                                        Constraint::Length(max(l, tmp_str.len() as u16))
                                                    }
                                                    _ => {
                                                        panic!()
                                                    }
                                                };

                                                tmp_str
                                            }
                                        )
                                    ),
                                    
                                ]
                            )
                        })
                        .collect();
                    rows
                }
            ).header(
                Row::new(
                    HEADERS.iter()
                        .map(|header| Cell::from(*header))
                        .collect::<Vec<Cell>>()
                )
            ).widths(&widths)
            .column_spacing(3)
            .block(
                Block::default()
                    .title("Snap Shots")
                    .borders(Borders::ALL)
            );



        frame.render_widget(table, display_area);
    }
}