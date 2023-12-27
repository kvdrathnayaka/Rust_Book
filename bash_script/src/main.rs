use std::error::Error;
use cmd_lib::{CmdResult, run_fun};
use std::{fmt, fs, io, string};
use std::collections::HashMap;
use std::fmt::Pointer;
use std::fs::{File, Metadata};
use std::io::{BufRead, BufReader, Read};
use std::os::linux::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use chrono::{DateTime, Timelike, Utc};
use md5::Digest;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use substring::Substring;
use crate::constants::{BASE_DIR, BIN, CPU_INFO, ETC_DIR, HTTP_BASE_URL, HTTP_CONTENT_TYPE, HUB, SECURITY_MODE_UPDATE_STATUS_REPORT_PATH, STATUS, STEP};

mod constants;

struct Data {
    id: String,
    version: String,
    r#type: String,
    importance: String,
    md5: String,
    status: String,
    sendFixes: bool,
    fixes: Vec<String>,
    createdDateTime: String,
    updatedDateTime: String
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {

    let status_path = String::from(SECURITY_MODE_UPDATE_STATUS_REPORT_PATH);
    let temp = format_string(status_path.as_str(), vec!["first", "second", "third"]);
    println!("status path: [{}]", temp);

    // let mut headers = HeaderMap::new();
    // let ticket: &'static str = "ticket";
    // let serial: &'static str = "serial";
    // let header_ticket = reqwest::header::HeaderName::from_static(ticket);
    // let header_serial = reqwest::header::HeaderName::from_static(serial);
    //
    // headers.insert(CONTENT_TYPE, HTTP_CONTENT_TYPE.parse().unwrap());
    // headers.insert(header_ticket, "ST-58-K3-GjPglUE5t-tC9dxNlECitHH8-dev-appserver-sg".parse().unwrap());
    // headers.insert(header_serial, "70220cea9f220ae6684f5369f61f3ef8".parse().unwrap());
    //
    // let mut body = HashMap::new();
    // // body.insert("email", "kavindu.rathnayaka@viewqwest.com");
    // // body.insert("password", "test123");
    // body.insert(STEP, "RUN_UPDATE_FINALIZE");
    // body.insert(STATUS, "DONE");
    // https_post(headers, body)?;

    let vec = vec![HTTP_BASE_URL, SECURITY_MODE_UPDATE_STATUS_REPORT_PATH].concat();
    // let s = vec.concat();
    println!("{:?}", vec);

    let count: u32 = 4;
    let timeout = 1.0;
    for cnt in 0..count {
        let res = &timeout / count as f64 / 2.0;
        println!("Count: [{}]", res as i32);
    }

    let seconds = 6.25;
    let milliseconds = (seconds * 1000.0) as u64;
    println!("Output: {}", milliseconds);

    let file_path = PathBuf::from(BIN).join(CPU_INFO);
    let content = match read_content(&file_path) {
        Ok(content) => content,
        Err(e) => return Err(e)
    };
    println!("File content: \n{}", content);

    let s1 = "kvd_randika";
    let s2 = "kvd";

    let sub_str = s2.substring(0, 3);
    println!("Sub String: {}", sub_str);
    // if s1.contains(&sub_str) {
    //     println!("Has matched...!");
    // } else {
    //     println!("Has not matched...!");
    // }


    Ok(())
}

pub fn run_script() -> CmdResult{
    run_fun!(echo "KRX")?;
    Ok(())
}

pub fn write(path: &PathBuf, value: &str) -> Result<(), Box<dyn Error>> {
    fs::write(path, value)?;
    Ok(())
}

pub fn read(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    for (_index, line) in reader.lines().enumerate() {
        if let Ok(s) = line {
            first_line = s;
        }
        break;
    }
    Ok(first_line)
}

pub fn read_content(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(&path)?;
    Ok(content)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn remove(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn find_line<'a>(path: &'a PathBuf, hint: &'a str) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut matched_line = String::new();
    for (_index, line) in reader.lines().enumerate() {
        if let Ok(s) = line {
            if s.contains(hint) {
                matched_line = s;
                break;
            }
        }
    }
    Ok(matched_line)
}

pub fn format_string(string: &str, args: Vec<&str>) -> String {
    let mut res = string.to_string();
    for arg in args {
        let replace = String::from("{}");
        res = res.replacen(&replace, arg, 1);
    }
    res
}

pub fn serial_read() -> Result<String, Box<dyn Error>> {
    let file_path = PathBuf::from("/home/kavindu/Desktop/Rust_Book/src/bash_script/src/bin/").join("cpuinfo");
    let processor_serial_line = find_line(&file_path, "Serial");
    let mut digest = md5::compute(b"");
    if let Ok(serial_line) = processor_serial_line {
        let line_parts = serial_line.split_whitespace();
        let serial = line_parts.last();
        if let Some(serial) = serial {
            digest = md5::compute(serial);
        }
    }
    let serial_number = format!("{:x}", digest);
    Ok(serial_number)
}

pub fn create_hub_file() -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from(BIN).join(HUB);
    let content = "My name is: [KRX\nKVD]";
    write(&file_path, content);
    Ok(())
}

pub fn is_file_valid(path: &PathBuf) -> bool {
    let metadata = fs::metadata(&path);
    let current_time: DateTime<Utc> = Utc::now();
    let created_time: DateTime<Utc> = match metadata {
        Ok(time) => {
            let time = match time.created() {
                Ok(t) => t,
                _ => {
                    panic!("Error getting system time !")
                }
            };
            time.into()
        },
        _ => {
            panic!("Error getting file metadata !")
        }
    };
    let diff_seconds = (current_time - created_time).num_seconds();
    println!("Time different: {:?} seconds", diff_seconds);
    if diff_seconds > i64::from(180) {
        false
    } else {
        true
    }
}

pub fn is_arm_footprint_available() -> Option<String> {
    let file_path = PathBuf::from(BIN).join(CPU_INFO);
    if Path::new(&file_path).exists() {
        let file_content = read(&file_path);
        if let Ok(content) = file_content {
            return Some(content);
        }
    }
    None
}

fn https_post(header_params: HeaderMap, body: HashMap<&str, &str>) -> Result<(), Box<dyn Error>> {
    let mut headers = HeaderMap::from(header_params);

    println!("headers: [{:?}]", headers);
    println!("body: [{:?}]", body);
    let client = reqwest::blocking::Client::new();
    let get_url = Url::from_str("https://dev-smarthome.vqbn.com/api/update/check/0.0.900")?;
    let post_url = "https://dev-smarthome.vqbn.com/api/update/status/dcfa8b2a35bd0d6265fc480873ec20c9/0.2";
    let response = client.get(get_url)
        .headers(headers)
        .send()?;
    // let response_body = response.text()?;
    // let res = &response_body[1..response_body.len()-1];
    // let parts: Vec<&str> = res.split(",").collect();
    // for part in parts {
    //     println!("Part: [{}]", part);
    // }
    println!("Response: [{:?}]", response.headers());
    Ok(())
}
