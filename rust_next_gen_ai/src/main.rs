use kalosm::language::*;

#[tokio::main]
async fn main() {
    // Create default text model
    let mut llm = Llama::new().await.unwrap();

    // Create model text prompt
    let prompt = "The following is a 300 word essay about Paris:";

    // Output the text prompt
    println!("{}", prompt);

    // Get the Text Completion Builder from Llama model
    let mut stream = llm.complete(prompt);

    // Output stream to STD out
    stream.to_std_out().await.unwrap();
}
