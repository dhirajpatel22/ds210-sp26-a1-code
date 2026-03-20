use kalosm::language::*;
use crate::solution::file_library;

pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);

        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // TODO: You have to implement the rest:
        // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
        // Think about what needs to happen if the function returns None vs Some(session).
        // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session

        return String::from("Hello, I am not a bot (yet)!");
    }

    pub async fn get_history(&self, username: String) -> Vec<String> { //changed pub fn to pub ASYNC fn
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => { //should use filename somewhere? 
            // TODO: what should happen here?
                let model = Llama::new_chat().await.unwrap();
                let mut chat = model.chat();
                // Add a message to the chat history
                chat("Hello, world!").to_std_out().await.unwrap();
                // Get the chat session
                let session = chat.session().unwrap();
                // Get the chat history
                let history = session.history();
                println!("{:?}", history);

            //reuse v3 basic_chatbot OR do it with the code from rust book?

                return Vec::new(); //given by kinan
            }
        }
    }
}

//probably the get_history part is wrong