use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    {fs::{File, remove_file}},
    io::prelude::*, env, fs
};

use ron::{error::SpannedError};

use serde::{
    Deserialize,
    Serialize
};

use time::{OffsetDateTime};

use super::{sql::SQL, data_base::DataBase};


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
    pub time_stamp: OffsetDateTime,
    pub path: String,
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

impl From<DataBase> for SnapShot {
    fn from(db: DataBase) -> Self {
        let mut path = env::current_dir().unwrap();
        path.push("snap_shots");
        let metadata = fs::metadata(&path);

        if metadata.is_err() {
            let _result = fs::create_dir_all(path.as_path());
        }

        let timestamp = OffsetDateTime::now_utc();

        path.push(format!("snap_shot_{}.sql", timestamp.unix_timestamp()));

        let file_path = path.as_path().to_str().unwrap();
        let _result = SQL::save_to_file(file_path, &db.get_snapshot());

        SnapShot{
            time_stamp: timestamp,
            path: file_path.to_string(),
        }
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
pub struct SnapShotsFile{
    pub name: String,
    pub snap_shots: HashSet<SnapShot>,
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

    pub fn add_snapshot(&mut self, snapshot: SnapShot) {
        self.snap_shots.insert(snapshot);
        let _result = self.save();
        self.update();
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

    pub fn replace_snapshots(&mut self, snapshots: &Vec<SnapShot>) {
        let mut snap_shots: HashSet<SnapShot> = HashSet::new();

        snapshots.iter()
            .for_each(|snapshot| {snap_shots.insert(snapshot.clone());});

    
        self.snap_shots.difference(&snap_shots)
            .into_iter()
            .for_each(|snapshot| {
                let _result = remove_file(&snapshot.path);
            });
        
      self.snap_shots.clear();

      snapshots.iter()
            .for_each(|snapshot| {self.snap_shots.insert(snapshot.clone());});

     let _result = self.save();
    }
}

impl Default for SnapShotsFile{
    fn default() -> Self {
        const PATH: &str = "snap_shots.ron";

        let snap_shot_file = SnapShotsFile::open(PATH);

        match snap_shot_file {
            Ok(val) => val,
            Err(err) => {
                if let Error::FileOpenErr(_err) = err {
                    let snap_shot_file = Self {
                        name: String::from(PATH),
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
