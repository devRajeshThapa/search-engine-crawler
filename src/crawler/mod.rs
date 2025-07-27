use std::{fs::File, io::{self, BufRead}};
use reqwest::blocking::get;

pub fn crawl(){
    let file = File::open("src/crawler/urls.txt").unwrap(); 
    let urls: Vec<String> = io::BufReader::new(file)
        .lines() 
        .filter_map(Result::ok) // collect only successfull lines
        .collect();

    for url in urls.into_iter(){
        if url.chars().nth(0) == Some('#') || url == "" {
            continue;
        }
        fetch_html(&url);
    }
}

fn fetch_html(url: &String){
    let response = get(url).unwrap().text().unwrap();
    println!("fetched: {}", url);
}
