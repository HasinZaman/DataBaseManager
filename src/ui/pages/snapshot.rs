use std::{
    cmp::max,
    collections::HashSet,
    hash::{Hash, Hasher},
    {fs::File},
    io::prelude::*
};

use ron::{error::SpannedError};

use serde::{
    Deserialize,
    Serialize
};

use time::{OffsetDateTime};

use tui::{
    layout::{Constraint, Rect},
    text::Span,
    widgets::{Table, Row, Cell, Block, Borders},
    {Frame,backend::CrosstermBackend}
};

use crate::ui::renderable::Renderable;

#[derive(Debug)]
pub enum Error {
    PathDoesNotExist,
    FileOpenErr(std::io::Error),
    FileReadErr(std::io::Error),
    FileWriteErr(std::io::Error),
    DeSerializationErr(SpannedError),
    Err(String)
}

#[derive(Deserialize, Serialize, Eq, Clone)]
pub struct SnapShot{
    time_stamp: OffsetDateTime,
    path: String,
}

impl SnapShot {
    pub fn new(time_stamp: OffsetDateTime, path: String) -> Result<SnapShot, Error> {
        Ok(
            SnapShot {
                time_stamp: time_stamp,
                path: {
                    if !std::path::Path::new(&path).exists() {
                        return Err(Error::PathDoesNotExist); 
                    }

                    path
                }
            }
        )
    }
}

impl Hash for SnapShot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl PartialEq for SnapShot {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[derive(Deserialize, Serialize)]
struct SnapShotsFile{
    name: String,
    snap_shots: HashSet<SnapShot>,
}


impl SnapShotsFile{
    pub fn open(file_name: &str) -> Result<SnapShotsFile, Error> {
        let mut file: File = match File::open(file_name) {
            Ok(file) => file,
            Err(err) => return Err(Error::FileOpenErr(err))
        };

        let contents: String = {
            let mut contents: String = String::new();

            if let Err(err) = file.read_to_string(&mut contents) {
                return Err(Error::FileReadErr(err))
            };

            contents
        };

        match ron::from_str(&contents) {
            Ok(val) => Ok(val),
            Err(err) => Err(Error::DeSerializationErr(err))
        }
    }

    pub fn update(&mut self) {
        let tmp = SnapShotsFile::open(&self.name).unwrap();

        self.snap_shots = tmp.snap_shots;
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut file : File = match File::create(&self.name) {
            Ok(file) => file,
            Err(err) => return Err(Error::FileOpenErr(err))
        };

        let buffer = ron::to_string(&self).unwrap();
        let buffer = buffer.as_bytes();

        match file.write(buffer) {
            Ok(written_bytes) => {
                if written_bytes == buffer.len() {
                    return Ok(())
                }
                todo!();
                //return Err(Error::Err(String::from("")))
            },
            Err(err) => return Err(Error::FileWriteErr(err)),
        }

    }
}

impl Default for SnapShotsFile{
    fn default() -> Self {
        let path = "snap_shots.ron";
        let snap_shot_file = SnapShotsFile::open(path);

        match snap_shot_file {
            Ok(val) => val,
            Err(err) => {
                if let Error::FileOpenErr(_err) = err {
                    let snap_shot_file = Self {
                        name: String::from(path),
                        snap_shots: HashSet::new(),
                    };
                    
                    let _result = snap_shot_file.save();
    
                    snap_shot_file
                }
                else{
                    todo!()
                }
            }
        }
    }
}

#[derive(Default)]
pub struct SnapShotPage{
    snap_shots_file: SnapShotsFile
}

impl SnapShotPage{
    pub fn new(file: &str) -> Result<SnapShotPage, Error> {
        let file = SnapShotsFile::open(file)?;
        Ok(
            SnapShotPage{
                snap_shots_file: file
            }
        )
    }

    pub fn update(& mut self) {
        self.snap_shots_file.update();
    }

    pub fn add(& mut self, snap_shot: SnapShot) -> & mut Self{
        self.snap_shots_file
            .snap_shots
            .insert(snap_shot);
        
        self.save();
        self
    }

    pub fn del(& mut self, path: &str) -> Result<& mut Self, Error>{

        let del_snap_shot = {
            let val = self.snap_shots_file
            .snap_shots
            .iter()
            .filter(|s| s.path == path)
            .next();

            match val {
                Some(val) => Some(val.clone()),
                None => None
            }
        };

        match del_snap_shot {
            Some(val) => {
                self.snap_shots_file
                    .snap_shots
                    .remove(&val);
            
                    self.save();
                Ok(self)
            },
            None => Err(Error::PathDoesNotExist)
        }
    }

    fn save(&self) {
        if let Err(_err) = self.snap_shots_file.save() {
            //error pop notification
            todo!()
        }
    }

}

impl Renderable for SnapShotPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        let mut widths: [Constraint;2] = [Constraint::Length(10), Constraint::Length(4)];

        let table: Table = Table::new(
                {
                    let rows: Vec<Row> =self.snap_shots_file
                        .snap_shots
                        .iter()
                        .map(|snap_shot| {
                            Row::new(
                                vec![
                                    Cell::from(
                                        Span::from(
                                            {
                                                let tmp_str = snap_shot.time_stamp.to_string();

                                                widths[0] = match widths[0] {
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
                                                let tmp_str = snap_shot.path.to_string();
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
                                ]
                            )
                        })
                        .collect();

                    rows
                }
            ).header(
                Row::new(
                    vec![
                        Cell::from("Time Stamp"),
                        Cell::from("Path")
                    ]
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