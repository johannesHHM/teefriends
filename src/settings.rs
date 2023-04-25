use std::{fs::File, io::{BufReader, BufRead}};
use std::fs;

pub fn read_friends(settings_path: String) -> Result<Vec<String>, std::io::Error>{
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

pub fn store_data(online_friends: &Vec<String>, store_path: String) -> Result<(), std::io::Error> {
    fs::write(store_path, online_friends.join("\n"))?;
    Ok(())
}


pub fn read_store_data(online_friends: &mut Vec<String>, store_path: String) -> Result<(), std::io::Error> {
    let file = File::open(store_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        online_friends.push(line?);
    }
    Ok(())
}