use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    model: Llama,
    chat_memory: Chat<Llama>,
}

impl ChatbotV2 {

    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {

        let chat_memory = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        return ChatbotV2 {
            model: model,
            chat_memory: chat_memory,
        };  
            // Whatever you decide to store in the struct
            // you need to make sure you pass here!
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {

        // send user message to LLM
        let asynchronous_output = self.chat_memory.add_message(message);

        // wait for response
        let output  = asynchronous_output.await;

        // extract message text
        let response = output.unwrap(); // previously named reponse 

        return response.to_string(); // previously named reponse

    }
}