// struct for a question
// a question consists of a question string and a list of answers

#[derive(Debug,serde::Serialize, serde::Deserialize, Clone)]
pub struct Question {
    pub question: String,
    pub answer1: String,
    pub answer2: String,
    pub answer3: String,
    pub answer4: String,
    pub correct_answer: usize,
    pub round: usize,
}
#[derive(Debug,serde::Serialize, serde::Deserialize, Clone)]
pub struct Round {
    pub questions: Vec<Question>,
}