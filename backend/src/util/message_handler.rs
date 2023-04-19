use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tungstenite::{Message, WebSocket};

use crate::structs::adminsessions::AdminSession;
use crate::structs::message_handler::MessageHandler;
use crate::structs::player::Player;
use crate::structs::question::Round;
use crate::structs::{nextphase::NextPhase, stats::Stats};
use crate::util::stats::evaluate_answers;

use super::generate_qr::generate_qr;

impl MessageHandler {
    pub fn handle_end_of_game(websocket: &mut WebSocket<TcpStream>) {
        if websocket
            .write_message(Message::Text("endOfGame".to_string()))
            .is_err()
        {
            // log to console
            println!("Client disconnected");
        }
    }

    // Handles the getPhase request from the client. Returns okay if the client is still connected, otherwise returns an error.
    pub fn handle_get_phase(
        websocket: &mut WebSocket<TcpStream>,
        next_phase: &Arc<Mutex<NextPhase>>,
    ) -> Result<(), String> {
        let phase = if *next_phase.lock().unwrap() == NextPhase::Question {
            NextPhase::Stats
        } else {
            NextPhase::Question
        };
        if websocket
            .write_message(Message::Text(format!("phase|{:?}", phase)))
            .is_err()
        {
            // log to console
            println!("Client disconnected");
            return Err("Client disconnected".to_string());
        }
        return Ok(());
    }

    pub fn handle_get_stats(
        websocket: &mut WebSocket<TcpStream>,
        stats: &Arc<Mutex<Stats>>,
        next_phase: &Arc<Mutex<NextPhase>>,
    ) -> Result<(), String> {
        if *next_phase.lock().unwrap() == NextPhase::Question {
            // lock the stats
            let stats_locked = stats.lock().unwrap();
            if websocket
                .write_message(Message::Text(
                    serde_json::to_string(&*stats_locked).unwrap(),
                ))
                .is_err()
            {
                // log to console
                println!("Client disconnected");
                return Err("Client disconnected".to_string());
            }
        }

        return Ok(());
    }

    pub fn handle_next(
        websocket: &mut WebSocket<TcpStream>,
        msg: &str,
        next_phase: &Arc<Mutex<NextPhase>>,
        admin_session: &Arc<Mutex<AdminSession>>,
        players: &Arc<Mutex<Vec<Player>>>,
        current_round: &Arc<Mutex<usize>>,
        rounds: &Vec<Round>,
        stats: &Arc<Mutex<Stats>>,
    ) -> Result<(), String> {
        // change to next phase
        let mut next_phase_mut = next_phase.lock().unwrap();

        // check if the admin session id is correct
        if msg[5..] == admin_session.lock().unwrap().session_id {
            // log to console
            println!("Admin requested next round");
            // check if there is a next round
            let mut current_round_value = current_round.lock().unwrap();
            // print the current round
            println!("Current round: {}", *current_round_value);
            // print current phase
            println!("Current phase: {:?}", *next_phase_mut);
            if *next_phase_mut == NextPhase::Stats {
                // calculate the stats
                let mut stats_locked = stats.lock().unwrap();
                *stats_locked =
                    evaluate_answers(&players.lock().unwrap(), &rounds[*current_round_value]);
                // initialize next round for all players
                for player in players.lock().unwrap().iter_mut() {
                    player.new_round();
                }
            } else if *current_round_value + 1 < rounds.len() {
                // reset the current question of all players
                for player in players.lock().unwrap().iter_mut() {
                    player.current_question = 0;
                }
                // increment the current round
                *current_round_value += 1;
                println!("Incremented current round to {}", *current_round_value);
                // send the message to client: nextSuccess
                if websocket
                    .write_message(Message::Text("nextSuccess".to_string()))
                    .is_err()
                {
                    // log to console
                    println!("Client disconnected");
                    return Err("Client disconnected".to_string());
                }
            } else {
                // end the game
                *next_phase_mut = NextPhase::End;
                if websocket.write_message(Message::Text("endOfGame".to_string())).is_err()
                {
                     // log to console
                     println!("Client disconnected");
                     return Err("Client disconnected".to_string());
                }
            }
            if *next_phase_mut == NextPhase::Question {
                *next_phase_mut = NextPhase::Stats;
            } else if *next_phase_mut == NextPhase::Stats {
                *next_phase_mut = NextPhase::Question;
            }
        }
        return Ok(());
    }

    pub fn handle_login(
        websocket: &mut WebSocket<TcpStream>,
        msg: &str,
        password: &str,
        admin_session: &Arc<Mutex<AdminSession>>,
    ) -> Result<(), String> {
        if &msg[6..] == password {
            // log to console
            println!("Admin logged in");
            // set the admin session id to hash of  admin + the current time
            admin_session.lock().unwrap().session_id = format!(
                "{:x}",
                md5::compute(format!("admin{}", chrono::Utc::now().timestamp()))
            );
            // send the message to client: loginSucess|session_id
            if websocket
                .write_message(Message::Text(format!(
                    "loginSuccess|{}",
                    admin_session.lock().unwrap().session_id
                )))
                .is_err()
            {
                // log to console
                println!("Client disconnected");
                return Err("Client disconnected".to_string());
            }
        } else {
            // send the message to client: loginFailed
            if websocket
                .write_message(Message::Text("loginFailed".to_string()))
                .is_err()
            {
                // log to console
                println!("Client disconnected");
                return Err("Client disconnected".to_string());
            }
        }
        return Ok(());
    }

    pub fn handle_add_player(msg: &str, players: &Arc<Mutex<Vec<Player>>>) {
        // create a new player
        let player = Player::new(msg[9..].to_string());
        println!("Added new player with session_id: {}", player.session_id);

        players.lock().unwrap().push(player);
    }

    pub fn handle_get_question(
        websocket: &mut WebSocket<TcpStream>,
        players: &Arc<Mutex<Vec<Player>>>,
        msg: &str,
        current_round: &Arc<Mutex<usize>>,
        rounds: &Vec<Round>,
    ) -> Result<(), String> {
        // find the player with the session_id
        let player_opt = players.lock().unwrap();
        let player = player_opt
            .iter()
            .find(|&p| p.session_id == msg[12..].to_string());
        // check if the player exists
        if player.is_some() {
            let player = player.unwrap();
            //log to console
            println!(
                "Sending question to player with session_id: {}",
                player.session_id
            );

            // check if the player has answered all questions of the current round
            let current_round_value = current_round.lock().unwrap();

            if player.current_question == rounds[*current_round_value].questions.len() {
                // send the end of round message to the client
                if websocket
                    .write_message(Message::Text("endOfRound".to_string()))
                    .is_err()
                {
                    // log to console
                    println!("Client disconnected");
                    return Err("Client disconnected".to_string());
                }
                // check if the player has answered all questions of the game
            } else if player.current_question == rounds.len() {
                // send the end of game message to the client
                if websocket
                    .write_message(Message::Text("endOfGame".to_string()))
                    .is_err()
                {
                    // log to console
                    println!("Client disconnected");
                    return Err("Client disconnected".to_string());
                }
                // send the next question to the client
            } else {
                // send the question to the client and check if the client is still connected
                if websocket
                    .write_message(Message::Text(
                        serde_json::to_string(
                            &rounds[*current_round_value].questions[player.current_question],
                        )
                        .unwrap(),
                    ))
                    .is_err()
                {
                    // log to console
                    println!("Client disconnected");
                    return Err("Client disconnected".to_string());
                }
            }
        }

        return Ok(());
    }

    pub  fn handle_get_qr(websocket: &mut WebSocket<TcpStream>) -> Result<(), String>{
        let qr = generate_qr();
        // print "qr code requested"
        println!("QR code requested");
        // send the qr code to the client
        if websocket
            .write_message(Message::Text(qr))
            .is_err()
        {
            // log to console
            println!("Client disconnected");
            return Err("Client disconnected".to_string());
        }
        return Ok(());
    }
}
