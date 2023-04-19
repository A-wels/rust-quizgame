use crate::structs::{question::Round, player::Player, stats::Stats};

// function to elavuate all answers
pub fn evaluate_answers(players: &Vec<Player>, round: &Round) -> Stats {
    let mut stats = Stats::new(round.questions.clone());

// loop through all questions
        // loop through all players
        for player in players{
            let mut index: usize = 0;
            for answer in &player.current_answers{
                // count answer
                    if *answer <5{
                        stats.answers.answers[index][*answer] += 1;
                    }
                    
                index +=1
            }
        }
        
    return stats;
}