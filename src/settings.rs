use std::{fs::File, io::{BufReader, BufRead}};
use std::fs;
use dirs::data_local_dir;

pub fn read_friends(settings_path: &String) -> Result<Vec<String>, std::io::Error>{
    let file = File::open(settings_path)?;
    let reader = BufReader::new(file);
    let mut friends: Vec<String> = vec![];

    for line in reader.lines() {
        let l = line?;
        if l.starts_with("add_friend") {
            let split: Vec<&str> = l.split("\"").collect();
            friends.push(split[1].to_string());
        }
    }
    Ok(friends)
}

pub fn store_data(online_friends: &Vec<String>, store_path: &String) -> Result<(), std::io::Error> {
    fs::create_dir_all(store_path)?;
    let file_path = store_path.to_owned() + "/friends.txt";
    fs::write(file_path, online_friends.join("\n") + "\n")?;
    Ok(())
}

pub fn read_store_data(online_friends: &mut Vec<String>, store_path: &String) -> Result<(), std::io::Error> {
    let file_path = store_path.to_owned() + "/friends.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        online_friends.push(line?);
    }
    Ok(())
}

pub fn get_data_dir() -> Option<String> {
    let mut data_dir: Option<String> = None;
    match data_local_dir() {
        Some(path_buf) => {
            match path_buf.to_str() {
                Some(path_string) => data_dir = Some(path_string.to_owned()),
                None => (),
            }
        },
        None => (),
    }
    return data_dir;
}