use std::fs::File;
use std::io::prelude::*;

use ron::{error::SpannedError};
use serde::{Deserialize, Serialize};
use time::{Time, Date};
use tui::{Frame,backend::CrosstermBackend, layout::Rect};

use crate::ui::renderable::Renderable;

#[derive(Debug)]
enum Error {
    FileOpenErr(std::io::Error),
    FileReadErr(std::io::Error),
    FileWriteErr(std::io::Error),
    DeSerializationErr(SpannedError),
    Err(String)
}

#[derive(Deserialize, Serialize)]
struct SnapShot{
    date: (Date, Time),
    path: String,
}

#[derive(Deserialize, Serialize)]
struct SnapShotsFile{
    name: String,
    snap_shots: Vec<SnapShot>,
}

impl SnapShotsFile{
    const SNAP_SHOT_NAME : &str = "snap_shots.ron";

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


pub struct SnapShotPage{
}
}

impl Renderable for SnapShotPage{
    fn render<T: std::io::Write>(&self, display_area: Rect, frame: &mut Frame<CrosstermBackend<T>>) {
        todo!()
    }
}