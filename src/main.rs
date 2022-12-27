use tokio::prelude::*;
use tokio::timer::Interval;
use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::time::{Duration, Instant};

extern crate lettre;
extern crate lettre_email;
extern crate mime;

mod config;


fn main() {
    let cli = config::CliOpts::init();

    let config = cli.parse();
    let email_receiver = config.email.address;
    let mine_email = config.email.smtp_host;
    let smtp_server = config.email.smtp_server;
    let password = config.email.password; //需要生成应用专用密码

    let email = Email::builder()
        .to(email_receiver)
        .from((mine_email.clone(),"Deeper Network"))
        .subject("Important Notification")
        .html("Dear user:<br /> Your Deeper Connect device has not interacted effectively with DeeperChain for the past 24 hours.
Please restart/ reboot your Deeper Connect to reconnect to Deeper Chain. If the latest block still did not display in https://deeperscan.io within 2 hours after the reboot, please contact our support desk. (http://support.deeper.network/)
<br />Thank you for your support for the Deeper Network.")
        .build()
        .unwrap();


    let creds = Credentials::new(
        mine_email.to_string(),
        password.to_string(),
    );

    // Open connection to Gmail
    let mut mailer = SmtpClient::new_simple(&smtp_server)
        .unwrap()
        .credentials(creds)
        .transport();

    // Send the email
    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    print!("{:?}", result);
    mailer.close();

    println!("fire; instant=wferf");
}
