use kalosm::language::*;

// Look at the docs for std::fs
// https://doc.rust-lang.org/std/fs/index.html
// std::fs provides functions that write to a file, read from a file,
// check if a file exists, etc.
use std::fs;

// LlamaChatSession provides helpful functions for loading and storing sessions.
// Look at https://docs.rs/kalosm/latest/kalosm/language/trait.ChatSession.html#saving-and-loading-sessions
// for some examples!

// Implement this
pub fn save_chat_session_to_file(filename: &str, session: &LlamaChatSession) {
    match session.to_bytes() {
        Ok(bytes) => { //if the .to_bytes suceeds & converts the session to bytes
            if let Err(err) = fs::write(filename, bytes) { 
                eprintln!("save_chat_session_to_file: could not write {}: {err}", filename); //if there is an error with writing to the file
            }
        }
        Err(err) => { //if .to_bytes returns an error
            eprintln!("save_chat_session_to_file: could not serialize {}: {err}", filename);
        }
    }
}

// Std2 Implement this
pub fn load_chat_session_from_file(filename: &str) -> Option<LlamaChatSession> {
        
    let data: Vec<u8> = match fs::read(filename) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("No file found");
            return None;
        }
    };
        match LlamaChatSession::from_bytes(&data) {
        Ok(session) => 
            return Some(session),
        Err(_) => {
            println!("Could not load chat session");
            return None;
        }
    }
}
    // look at fs::read(...)
    // also look at LlamaChatSession::from_bytes(...)
    //unimplemented!("Loading chat session from file {filename}");