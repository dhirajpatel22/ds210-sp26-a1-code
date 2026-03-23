use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }
    
    // Std2
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // Build a chat session and restore from disk when possible.
                let mut chat_session: Chat<Llama> = self
                    .model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");

                if let Ok(bytes) = std::fs::read(filename) {
                    if let Ok(existing_session) = LlamaChatSession::from_bytes(&bytes) {
                        chat_session = chat_session.with_session(existing_session);
                    }
                }

                let response = chat_session.add_message(message).await.unwrap_or_else(|err| {
                    eprintln!("chat_with_user: failed to get response: {err}");
                    String::from("Sorry, I could not generate a response.")
                });

                if let Ok(session) = chat_session.session() {
                    if let Ok(bytes) = session.to_bytes() {
                        if let Err(err) = std::fs::write(filename, bytes) {
                            eprintln!("chat_with_user: failed to save session file {filename}: {err}");
                        }
                    }
                }

                self.cache.insert_chat(username, chat_session);
                return response;
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                let response = chat_session.add_message(message).await.unwrap_or_else(|err| {
                    eprintln!("chat_with_user: failed to get response: {err}");
                    String::from("Sorry, I could not generate a response.")
                });

                if let Ok(session) = chat_session.session() {
                    if let Ok(bytes) = session.to_bytes() {
                        if let Err(err) = std::fs::write(filename, bytes) {
                            eprintln!("chat_with_user: failed to save session file {filename}: {err}");
                        }
                    }
                }

                return response;

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username); //get_chat updates this coversation to be most recent

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // load session from file if it exists
                if let Some(session) = file_library::load_chat_session_from_file(&filename) {
                    let mut chat = self.model.chat().with_session(session);
                    self.cache.insert_chat(username.clone(), chat);
                    
                    // get the cached chat to extract history
                    if let Some(chat_session) = self.cache.get_chat(&username) {
                        if let Ok(session) = chat_session.session() {
                            let history = session.history();
                            return history
                                .iter()
                                .filter(|msg| !matches!(msg.role(), kalosm::language::MessageType::SystemPrompt))
                                .map(|msg| msg.content().to_string())
                                .collect();
                        }
                    }
                }
                Vec::new()
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                
                //get the history from the cache
                if let Ok(session) = chat_session.session() {
                    let history = session.history();
                    println!("{:?}", history);

                    return history
                        .iter()
                        .filter(|msg| !matches!(msg.role(), kalosm::language::MessageType::SystemPrompt))
                        .map(|msg| msg.content().to_string())
                        .collect();
                }
                Vec::new() 
            }
        }
    }
}