// read rounds of questions from a file. The file format is:
// round starts with a line of ===
// questions start with a line of ---
// next 4 lines after a question are the answers
// the correct answer is marked with a * at the end of the line

use std::fs::File;
use csv::ReaderBuilder;

// load struct for questions from /structs/question.rs
use crate::backend::structs::question::{Question, Round};

// Read questions from questions.csv:
// First row contains table headers

pub fn load_questions(filepath: String) -> Vec<Round>{
    let mut last_round = 0;
    let mut rounds: Vec<Round> = Vec::new();
    let file = File::open(filepath).expect("File not found");
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    
    for result in reader.deserialize() {
        let question: Question = result.expect("Error reading question");
        if question.round != last_round {
            rounds.push(Round{questions: Vec::new()});
            last_round = question.round;
        }
        let len = rounds.len();
        rounds[len-1].questions.push(question);
    }
    return rounds;
    }