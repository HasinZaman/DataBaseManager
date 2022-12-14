use std::{fs::{File, self}, io::Write};

pub struct FileEnv{
    file_name: String
}
impl FileEnv{
    pub fn new(file_name: &str, content: &str) -> FileEnv {

        let mut file = File::create(&file_name).unwrap();
        file.write(&content.as_bytes()).unwrap();

        FileEnv{ file_name: file_name.to_string() }
    }
}
impl Drop for FileEnv{
    fn drop(&mut self) {
        println!("Dropping {}", &self.file_name);
        fs::remove_file(&self.file_name).unwrap();
    }
}