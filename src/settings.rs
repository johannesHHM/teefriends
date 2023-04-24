use std::{fs::File, io::{BufReader, BufRead}};

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