use std::{fs::File, io::Read};

use regex::Regex;
use lazy_static::lazy_static;

pub fn is_sql_file(file_path: &str) -> bool {
    lazy_static! {
        static ref SQL_FILE_REGEX: Regex = Regex::new(".[sS][qQ][lL]$").unwrap();
    };

    std::path::Path::new(file_path).exists() && SQL_FILE_REGEX.is_match(file_path)
}

pub fn parse_sql_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    if !is_sql_file(file_path) {
        let _f: File = File::open(file_path)?;
    }

    let mut f: File = File::open(file_path).unwrap();

    const BUFFER_SIZE: usize = 100;

    let mut buffer = [0; BUFFER_SIZE];
    let mut query_cmd : String = String::from("");

    let mut results: Vec<String> = Vec::new();
    loop {
        match f.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {

                let mut buffer_iter = buffer[..n].iter();
                while !append_to_cmd(&mut query_cmd, &mut buffer_iter) {
                    extract_cmd(&mut query_cmd, &mut results);
                };
                extract_cmd(&mut query_cmd, &mut results);
            }
            Err(err) => {
                return Err(err)
            }
        }
    }

    Ok(results)
}

fn extract_cmd(query_cmd: &mut String, results: &mut Vec<String>) {
    lazy_static! {
        static ref COMMENT_CHECK_REGEX: Regex = Regex::new("--.*\n$").unwrap();
    };
    //remove all command
    *query_cmd = COMMENT_CHECK_REGEX.replace_all(&*query_cmd, " ").to_string();

    
    //check if just bunch of \n
    lazy_static! {
        static ref CMD_TRIM_REGEX: Regex = Regex::new("^[\r\n\t ]+").unwrap();
    };
    *query_cmd = CMD_TRIM_REGEX.replace_all(&*query_cmd, "").to_string();

    lazy_static! {
        static ref CMD_END_CHECK_REGEX: Regex = Regex::new(";\r?\n?$").unwrap();
    };
    //check if command ends
    if CMD_END_CHECK_REGEX.is_match(query_cmd) {
        let query_cmd_tmp = CMD_END_CHECK_REGEX.replace(query_cmd, "");
        println!("str: \"{}\"", query_cmd_tmp);
        results.push(query_cmd_tmp.to_string());
        query_cmd.clear();
    }
}

fn append_to_cmd<'a, I>(tmp_cmd: & mut String, buffer: &mut I) -> bool where I: Iterator<Item = &'a u8> + Sized {

    while let Some(c) = buffer.next() {
        let c = *c as char;

        tmp_cmd.push(c);

        if c == '\n' {
            return false;
        }
    }

    true
}