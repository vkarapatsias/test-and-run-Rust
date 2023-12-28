
use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zerpo_shot_classification::ZeroShotClassficationModel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn create_db() -> sqlite::Connection {
    let db = sqlit::open(":memory").unwrap();
    db.execute("CREATE TABLE zeroshotcandidates (id INTEGER PRIMARYKEY, label TEXT)").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) values ('rock')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) values ('pop')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) values ('hip hop')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) values ('country')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) values ('latin')").unwrap();
    db
}

pub fn get_all_zeroshotcandidates() -> Vec<String> {
    let db = create_db();
    let query = "SELECT label FROM zeroshotcandidates";
    let mut candidates: Vec<String> = Vec::new();
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.oter() {
            let value = value.unwrap();
            candidates.push(value.to_string());
        }
        true
    })
    .unwrap();
    candidates
}

pub fn read_lyrics(file: &str) -> Vec<String> {
    let mut lyrics: Vec<String> = Vec::new();
    let file = File::open(file).expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        lyrics.push(line);
    }
    lyrics
}

pub fn classify_lyrics(lyrics: Vec<String>) -> Vec<Vec<Label>> {
    
}