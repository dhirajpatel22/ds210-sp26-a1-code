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

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");

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