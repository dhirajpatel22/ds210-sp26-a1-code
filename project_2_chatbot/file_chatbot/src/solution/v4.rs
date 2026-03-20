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
        let filename = &format!("{}.txt", username); // create a filename based on the username of each user

        let mut chat_session: Chat<Llama> = self.model  //create the chat session like normal
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        if let Ok(bytes) = std::fs::read(&filename) { //check if reading the file returns bytes
            if let Ok(existing_session) = LlamaChatSession::from_bytes(&bytes) { //if there is an existing session, replace it
                chat_session = chat_session.with_session(existing_session); 
            }
        }

        let response = chat_session.add_message(message).await.unwrap_or_else(|err| { //adds user message & waits for AI response
            eprintln!("chat_with_user: failed to get response: {err}"); //handles errors from the model
            String::from("Sorry, I could not generate a response.") 
        });

        if let Ok(session) = chat_session.session() { // get current session
            if let Ok(bytes) = session.to_bytes() { // convert session to bytes
                if let Err(err) = std::fs::write(&filename, bytes) { //write bytes to file
                    eprintln!("chat_with_user: failed to save session file {filename}: {err}");
                }
            }
        }

        return response;
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