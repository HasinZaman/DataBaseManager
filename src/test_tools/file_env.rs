use std::{fs::{File, self}, io::Write};

/// Struct representing a file environment
pub struct FileEnv{
    /// The name of the file
    file_name: String
}
impl FileEnv{
    /// Creates a new FileEnv with the given file name and content
    ///
    /// # Arguments
    ///
    /// * `file_name` - The name of the file to create
    /// * `content` - The content to write to the file
    pub fn new(file_name: &str, content: &str) -> FileEnv {

        let mut file = File::create(&file_name).unwrap();
        file.write(&content.as_bytes()).unwrap();

        FileEnv{ file_name: file_name.to_string() }
    }
}
impl Drop for FileEnv{
    /// Deletes the file when the FileEnv is dropped
    fn drop(&mut self) {
        println!("Dropping {}", &self.file_name);
        fs::remove_file(&self.file_name).unwrap();
    }
}