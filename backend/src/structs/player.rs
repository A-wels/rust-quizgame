#[derive(Debug,serde::Serialize, serde::Deserialize, Clone)]
pub struct Player {
    pub session_id: String,
    pub answers: Vec<usize>,
    pub current_answers: Vec<usize>,
    pub current_question: usize,
}
impl Player {
    pub fn new(session_id: String) -> Player {
        Player {
            session_id,
            answers: Vec::new(),
            current_answers: Vec::new(),
            current_question: 0,
        }
    }
    pub fn new_round(&mut self) {
        self.answers.append(&mut self.current_answers);
        self.current_answers = Vec::new();
        self.current_question = 0;
    }
}