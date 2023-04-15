// import load_questions() from ./util/read_questions.rs
mod structs;
mod util;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use structs::adminsessions::AdminSession;
use structs::nextphase::NextPhase;
use structs::player::Player;
use structs::question::Round;
use structs::stats::Stats;
use tungstenite::accept;
use util::read_questions;

use crate::structs::message_handler::MessageHandler;

fn main() {
    // Read config
    let config = structs::config::Config::new();
    let password = config.password;
    let question_file = config.question_file;

    // Start websocket server
    let server = TcpListener::bind("127.0.0.1:8001").unwrap();

    // Load questions and setup game data
    let rounds: Vec<Round> = read_questions::load_questions(question_file);
    let current_round: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let players: Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));
    let next_phase: Arc<Mutex<NextPhase>> = Arc::new(Mutex::new(NextPhase::Stats));
    let admin_session: Arc<Mutex<AdminSession>> = Arc::new(Mutex::new(AdminSession {
        session_id: "".to_string(),
    }));
    let stats: Arc<Mutex<Stats>> = Arc::new(Mutex::new(Stats::new(rounds[0].questions.clone())));

    // Listen for incoming connections
    for stream in server.incoming() {
        // Clone all data that is needed in the thread
        let current_round = current_round.clone();
        let rounds = rounds.clone();
        let players = players.clone();
        let password = password.clone();
        let admin_session = admin_session.clone();
        let next_phase = next_phase.clone();
        let stats = stats.clone();

        // Start a new thread for each connection
        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            // log to console
            // sent the current round of questions to the client
            // check for incoming messages and if the client is still connected
            loop {
                let msg = websocket.read_message();
                if msg.is_err() {
                    // log to console
                    println!("Client disconnected");
                    break;
                }
                let msg = msg.unwrap();

                if msg.is_text() {
                    // log to console
                    println!("Received: {}", msg.to_text().unwrap());
                    let msg = msg.to_text().unwrap();
                    // check if the message starts with register| and if the client is not already registered
                    let result = MessageHandler::handle_message(
                        &mut websocket,
                        msg,
                        &password,
                        &admin_session,
                        &next_phase,
                        &players,
                        &current_round,
                        &rounds,
                        &stats,
                    );
                    if result.is_err() {
                        // log to console
                        println!("{}", result.err().unwrap());
                        break;
                    }
                }
            }
        });
    }
}
