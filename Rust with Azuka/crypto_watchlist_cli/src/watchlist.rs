use std::{fs, io};

use crate::watchlist;

pub fn save_watchlist(watchlist: &[String]){
    let mut lists: String = String::new();
    for watch in watchlist {
      let list =  format!("{}\n", watch);
      lists.push_str(&list);
    }

    fs::write("watchlist.csv", lists).expect("Failed to write");
}

pub fn load_watchlist()->Vec<String> {
  let watchlist = match fs::read_to_string("watchlist.csv") {
    Ok(c) => c,
    Err(_) => return Vec::new(),
  };
  let lines:Vec<&str> = watchlist.lines().collect();
  let mut watchlists =  Vec::new();
    for line in lines {
    watchlists.push(line.trim().to_string());
};
watchlists

}
