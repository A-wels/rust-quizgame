use std::thread;
mod frontend;
mod backend;
use ctrlc;
fn main() {
    // start frontend server in a thread
    let handler = ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C! Exiting...");
        std::process::exit(0);
    });
    if handler.is_err() {
        println!("Error setting Ctrl-C handler");
        std::process::exit(1);
    }

    // print working directory
    println!("Starting frontend server");
    println!("Current directory: {}", std::env::current_dir().unwrap().display());
    thread::spawn(|| {
        frontend::frontend::start().unwrap();
    });

    println!("Starting backend server");
    // start backend server in a thread
    thread::spawn(|| {
        backend::backend::start();
    });
    println!("waiting for ctrl+c");
    // wait for ctrl+c and exit
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();



}
