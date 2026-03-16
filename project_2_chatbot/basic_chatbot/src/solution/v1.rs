use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV1 {
    model: Llama,
}

impl ChatbotV1 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV1 {
        return ChatbotV1 { model: model };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let mut chat_session: Chat<Llama> = self.model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        // send user message to LLM
        let asynchronous_output = chat_session.add_message(message);

        // wait for response
        let output = asynchronous_output.await;

        // extract message text
        let response = output.unwrap();

        return response.to_string();

        // You need to add your code here

        // You can add the user message to the chat session with the `add_message` method
        //let mut response_stream = chat.add_message(prompt);
        // And then stream the result to std out
        //response_stream.to_std_out().await.unwrap(); THIS OVERLAPS WITH WHAT I HAVE

        // You must find a way to add the given message to the chat_session!
        // consider https://docs.rs/kalosm/0.4.0/kalosm/language/struct.Chat.html#method.add_message
        // Hint: make sure you transform/extract the response message as a **String**.
        }
    }

// Your task is to:
// 1 complete the implementation of chat_with_user by passing the user-provided message to the chat session, 
// 2. and then retrieving and returning the LLM response.

// // We suggest you look at the add_message function provided by the kalosm library. Look at the given example for how it can be used.

// // The add_message function returns the response asynchronously. You can instruct Rust to wait until that entire response is ready by 
// invoking .await on what it returns. For example

// // let asynchronous_output = chat_session.add_message(...);
// // let output = asynchronous_output.await;
// notice lack of (), await is not a function; it is a special keyword!
// Look at the type of output. How can you extract the response message string from it? Hint: it is similar to (but not exactly the same 
// as) Option and can be dealt with using the same approaches we have seen before for Option.