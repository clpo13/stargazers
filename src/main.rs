#[macro_use]
extern crate serde;
extern crate reqwest;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    static APP_USER_AGENT: &str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    );

    static OWNER: &str = "rust-lang-nursery";
    static REPO: &str = "rust-cookbook";

    let request_url = format!("https://api.github.com/repos/{OWNER}/{REPO}/stargazers");

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let response = client.get(&request_url).send().await?;

    let users: Vec<User> = response.json().await?;

    println!("First 30 stargazers of {OWNER}/{REPO}:");
    for user in users.iter() {
        println!("  {} (id: {})", user.login, user.id);
    }

    Ok(())
}
