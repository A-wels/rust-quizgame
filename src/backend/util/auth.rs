use std::sync::Arc;

use mutex::Mutex;
use rand::{ prelude::*};

use crate::backend::structs::{
    adminsessions::AdminSession,
    player::{Player},
};

pub fn generate_game_id() -> String {
    let mut rng = rand::thread_rng();
    let mut game_id = String::new();
    for _ in 0..8 {
        let n: u8 = rng.gen_range(0..10);
        game_id.push_str(&n.to_string());
    }
    return game_id;
}

// generate a random session key consisting of characters and numbers 128 characters long
pub fn generate_session_key() -> String {
    let mut rng = rand::thread_rng();
    let mut session_key = String::new();
    for _ in 0..128 {
        let n: u8 = rng.gen_range(0..10);
        let c: char = rng.gen_range('a'..='z');
        if n % 2 == 0 {
            session_key.push_str(&n.to_string());
        } else {
            session_key.push(c);
        }
    }
    return session_key;
}

pub fn validate_session(
    players: &Arc<Mutex<Vec<Player>>>,
    session_id: &String,
    admin_session: &Arc<Mutex<AdminSession>>,
) -> bool {
    // check if a player with the session_id exists
    // check if unwrap panics#
    if admin_session.lock().unwrap().session_id == session_id.clone() {
        return true; 
    }
    for player in players.lock().unwrap().iter() {
        if player.session_id == session_id.clone() {
            return true;
        }
    }


    return false;
}
