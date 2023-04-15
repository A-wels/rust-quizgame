use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tungstenite::WebSocket;

use super::adminsessions::AdminSession;
use super::nextphase::NextPhase;
use super::player::Player;
use super::question::Round;
use super::stats::Stats;

pub struct MessageHandler {}
impl MessageHandler {
    pub fn handle_message(
        mut websocket: &mut WebSocket<TcpStream>,
        msg: &str,
        password: &String,
        admin_session: &Arc<Mutex<AdminSession>>,
        next_phase: &Arc<Mutex<NextPhase>>,
        players: &Arc<Mutex<Vec<Player>>>,
        current_round: &Arc<Mutex<usize>>,
        rounds: &Vec<Round>,
        stats: &Arc<Mutex<Stats>>,
    ) -> Result<(), String> {
        if msg.starts_with("register|")
            && players
                .lock()
                .unwrap()
                .iter()
                .find(|&p| p.session_id == msg[9..].to_string())
                .is_none()
        {
            // create a new player with the session_id
            MessageHandler::handle_add_player(msg, &players);

            // check for question request
        } else if msg.starts_with("getQuestion|") && *next_phase.lock().unwrap() == NextPhase::Stats
        {
            let result = MessageHandler::handle_get_question(
                &mut websocket,
                &players,
                &msg,
                &current_round,
                &rounds,
            );
            return result;
            // check if the client sent an answer of the form answer|answer-id|session-id
        } else if msg.starts_with("answer|") {
            // get the session id and the answer from the message
            let session_id = msg[7..].split("|").collect::<Vec<&str>>()[1];
            let answer = msg[7..].split("|").collect::<Vec<&str>>()[0];
            // log to console
            println!(
                "Received answer: {} from player with session_id: {}",
                answer, session_id
            );
            // find the player with the session_id
            let mut player_opt = players.lock().unwrap();
            let player = player_opt
                .iter_mut()
                .find(|p| p.session_id == session_id.to_string());
            // check if the player exists
            if player.is_some() {
                let player = player.unwrap();
                // log to console
                println!(
                    "Adding answer to player with session_id: {}",
                    player.session_id
                );
                // add the answer to the player
                player
                    .current_answers
                    .push(answer.parse::<usize>().unwrap());
                player.current_question += 1;
            }
        } else if msg.starts_with("login|") {
            // Handle login attempt
            let result =
                MessageHandler::handle_login(&mut websocket, &msg, &password, &admin_session);
            return result;
        } else if msg.starts_with("next|") {
            // change to the next phase
            let result = MessageHandler::handle_next(
                &mut websocket,
                &msg,
                &next_phase,
                &admin_session,
                &players,
                &current_round,
                &rounds,
                &stats,
            );
            return result;
        } else if msg.starts_with("getStats|") {
            // send the stats to the client if the nextphase is questions
            let result = MessageHandler::handle_get_stats(&mut websocket, &stats, &next_phase);
            return result;
        } else if msg.starts_with("getQuestion|") && *next_phase.lock().unwrap() == NextPhase::End {
            // send the end of game message to the client
            MessageHandler::handle_end_of_game(&mut websocket);
        } else if msg == "getPhase".to_string() {
            // send the phase to the client
            let result = MessageHandler::handle_get_phase(&mut websocket, &next_phase);
            return result;
        }
        return Ok(());
    }
}
