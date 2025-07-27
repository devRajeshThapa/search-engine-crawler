use std::{fs::File, io::{self, BufRead}};
use reqwest::blocking::get;
use mysql::*;
use mysql::prelude::*;

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
        break;
    }
}

fn fetch_html(url: &String){
    let response = get(url).unwrap().text().unwrap();
    parse_html(&response, &url);
}

fn parse_html(html: &str, url: &str) {
    let plain_text = html2text::from_read(html.as_bytes(), 80);

    let stopwords = get_stopwords();

    for raw_word in plain_text.split_whitespace() { 
        let word = clean_word(raw_word);

        if !word.is_empty() && !stopwords.contains(word.as_str()) {
            insert_into_db(&word, &url);
        }
    }
}

fn insert_into_db(word: &str, url: &str){
    let conn_url = "mysql://root:your_password@localhost:3306/search_engine";

    // Create a connection pool
    let pool = Pool::new(conn_url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    println!("{} => {}", word, url);
    // Insert query
    conn.exec_drop(
        "INSERT INTO inverted_index (word, url) VALUES (:word, :url)",
        params! {
            "word" => word,
            "url" => url,
        },
    ).unwrap();
}

fn clean_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

fn get_stopwords() -> std::collections::HashSet<&'static str> {
    let stopwords = [
        // Articles
        "a", "an", "the",

        // Pronouns
        "i", "me", "my", "myself", "we", "our", "ours", "ourselves",
        "you", "your", "yours", "yourself", "yourselves",
        "he", "him", "his", "himself", "she", "her", "hers", "herself",
        "it", "its", "itself", "they", "them", "their", "theirs", "themselves",

        // Demonstratives
        "this", "that", "these", "those",

        // Relative pronouns
        "who", "whom", "whose", "which", "what",

        // Auxiliary verbs
        "am", "is", "are", "was", "were", "be", "been", "being",
        "have", "has", "had", "having", "do", "does", "did", "doing",

        // Modals
        "will", "would", "shall", "should", "can", "could", "may", "might", "must",

        // Adverbs (common)
        "not", "no", "nor", "only", "just", "also", "too", "very",

        // Conjunctions
        "and", "but", "if", "or", "because", "as", "until", "while",

        // Prepositions
        "of", "at", "by", "for", "with", "about", "against", "between", "into",
        "through", "during", "before", "after", "above", "below", "to", "from",
        "up", "down", "in", "out", "on", "off", "over", "under",

        // Determiners / Quantifiers
        "all", "any", "both", "each", "few", "more", "most", "other",
        "some", "such", "own", "same",

        // Miscellaneous
        "so", "than", "too", "very", "s", "t", "can", "will", "just", "don",
        "should", "now", "d", "ll", "m", "o", "re", "ve", "y", "ain", "aren",
        "couldn", "didn", "doesn", "hadn", "hasn", "haven", "isn", "ma", "mightn",
        "mustn", "needn", "shan", "shouldn", "wasn", "weren", "won", "wouldn"
    ];
    stopwords.iter().copied().collect()
}

