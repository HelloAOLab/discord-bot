use std::env;

// Get the token from the environment variable
pub fn get_token() -> String {
    env::var("DISCORD_TOKEN").expect("Expected a token in the environment")
}
