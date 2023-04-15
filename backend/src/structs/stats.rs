use super::question::Question;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    pub questions : Vec<Question>,
    pub answers: AggregatedAnswers

}
impl Stats {
    pub fn new(questions: Vec<Question>) -> Stats {
        Stats {
            answers: AggregatedAnswers::new(questions.len()),
            questions,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AggregatedAnswers {
    // Vector containing an element for each question, each element is a vector containing the amount of answers for each answer
    pub answers: Vec<Vec<usize>>,

}
impl AggregatedAnswers {
    pub fn new(questions: usize) -> AggregatedAnswers {
        let mut answers = Vec::new();
        for _ in 0..questions {
            answers.push(Vec::new());
        }
        for answer in &mut answers {
            for _ in 0..4 {
                answer.push(0);
            }
        }
        AggregatedAnswers {
            answers,
        }
    }
}