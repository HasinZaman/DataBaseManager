use std::{fs::File, io::{Read, Error}};

use regex::{Regex, Captures};
use lazy_static::lazy_static;

/// Replaces occurrences of the pattern "#file:([file_path] as [read_type])" in `cmd`
/// with the contents of the specified file.
///
/// The file path must contain only alphabetical, numeric, forward-slash (/), and underscore (_)
/// characters, and must end in a file extension consisting of one or more alphabetical characters.
///
/// The read type must be either "B" for binary data or "S" for a string.
///
/// If the file cannot be opened or read, an error is returned.
///
/// # Examples
///
/// ```rust
/// let file_path = "test.txt";
/// let file_contents = "Hello, world!";
///
/// let mut file = File::create(file_path)?;
/// file.write_all(file_contents.as_bytes())?;
///
/// let cmd = format!("#file:({} as S)", file_path);
/// let contents = contents(&cmd)?;
///
/// assert_eq!(contents, file_contents);
/// ```
pub fn contents(cmd: &str) -> Result<String, Error> {
    lazy_static! {
        /// Regular expression for matching the pattern "#file:([file_path] as [read_type])".
        static ref FILE_INPUT_REGEX: Regex = Regex::new("#file:\\(([a-zA-Z][a-zA-Z0-9:/\\\\_ ]+.[a-zA-Z]+) as ([BS])\\)").unwrap();
    };

    let mut cmd = cmd.to_string();

    let mut err_flag:Option<Error> = None;

    while FILE_INPUT_REGEX.is_match(&cmd) && err_flag.is_none() {
        cmd = FILE_INPUT_REGEX.replace(
            &cmd,
            |caps: &Captures| {
                let file_path = &caps[1];
                let read_type = &caps[2];

                let file = File::open(file_path);

                if let Err(err) = file {
                    err_flag = Some(err);
                    return format!("");
                }

                let mut file = file.unwrap();

                match read_type {
                    "B" => todo!(),//binary
                    "S" => { //string
                        let mut contents: String = String::new();
                        let result = file.read_to_string(&mut contents);

                        if let Err(err) = result{
                            err_flag = Some(err);
                            return format!("");
                        }

                        return contents
                    },
                    _=> panic!()
                }
            }
        ).to_string();
    }

    if err_flag.is_some() {
        return Err(err_flag.unwrap())
    }

    Ok(cmd.to_string())
}