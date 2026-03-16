use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?

    model: Llama,
    username: HashMap<String, Chat<Llama>>, 
    //does not store chat like v2

    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.

    //dictionary - data linked to user (my comment)
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
        // Make sure you initialize your struct members here
            model: model,
            username: HashMap::new(),
        }; 
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        return String::from("Hello, I am not a bot (yet)!");
    }

    #[allow(dead_code)] //std2
    pub fn get_history(&self, username: String) -> Vec<String> {
        // Retrieve the chat session for this user from the HashMap
        if let Some(chat) = self.username.get(&username) {
            // Get the session and history from the chat
            if let Ok(session) = chat.session() {
                // Return the chat history as a vector of strings
                return session.history().iter().map(|msg| format!("{:?}", msg)).collect();
            }
        }
        Vec::new()
    }
}