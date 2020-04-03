#[macro_use]
extern crate serde;
extern crate reqwest;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

fn main() -> Result<(), Error> {
    static APP_USER_AGENT: &str = concat!(
        env!("CARGO_PKG_NAME"),
        "/",
        env!("CARGO_PKG_VERSION"),
    );

    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let response = client.get(&request_url).send()?;
    //println!("{:#?}", response);

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}
