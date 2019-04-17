extern crate clap;
extern crate ctrlc;
extern crate scraper;
extern crate subprocess;

use clap::{App, Arg};
use scraper::{Html, Selector};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use subprocess::Exec;

fn get_url_based_on_config(config: &str) -> Option<String> {
    let filename = Path::new(config).file_stem().unwrap();
    let basepath: String = {
        let tmp = filename
            .to_str()
            .unwrap()
            .split("-")
            .nth(0)
            .unwrap()
            .to_string();
        tmp.replace(",", ".")
    };
    Some(format!("https://{}/accounts", basepath.to_lowercase()))
}

fn main() -> Result<(), reqwest::Error> {
    const AUTH_FILE_LOCATION: &str = "/tmp/freevpn-auth.txt";

    // CLI-Parameters
    let matches = App::new("FreeVPN spawner")
        .version("0.1")
        .author("Königsreiter Simon")
        .arg(
            Arg::with_name("file")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .help("The OpenVPN config file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short("a")
                .long("address")
                .default_value("https://freevpn.me/accounts")
                .value_name("FREEVPN_URL")
                .help("The URL to the FreeVPN accounts page")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username_css_selector")
                .short("u")
                .long("username-selector")
                .default_value("div.span4:nth-child(3) > ul:nth-child(1) > li:nth-child(1)")
                .value_name("USERNAME_CSS_SELECTOR")
                .help("The CSS Selector for the username")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password_css_selector")
                .short("p")
                .long("password-selector")
                .default_value("div.span4:nth-child(3) > ul:nth-child(1) > li:nth-child(2)")
                .value_name("PASSWORD_CSS_SELECTOR")
                .help("The CSS Selector for the password")
                .takes_value(true),
        )
        .get_matches();

    // Clean-Up Routine
    ctrlc::set_handler(|| {
        println!("Removing {} before exit!", AUTH_FILE_LOCATION);
        std::fs::remove_file(AUTH_FILE_LOCATION).expect("Couldn't delete File!");
    })
    .expect("Could not set CTRL-C Handler!");

    // Get HTML Page; scrape username & pasword and write to file;
    {
        let url =
            match get_url_based_on_config(matches.value_of("file").expect("No Config file given!"))
            {
                Some(url) => url,
                None => String::from(matches.value_of("url").expect("No default URL given!")),
            };
        let mut resp = reqwest::get(url.as_str())?;
        assert!(resp.status().is_success());

        let body = resp.text().unwrap();
        let fragment = Html::parse_document(&body);
        let username = Selector::parse(matches.value_of("username_css_selector").unwrap()).expect("Invalid CSS Selector for username!");
        let password = Selector::parse(matches.value_of("password_css_selector").unwrap()).expect("Invalid CSS Selector for password!");

        let username_str: String = {
            let tmp: String = fragment
                .select(&username)
                .next()
                .expect("No username found for CSS Selector!")
                .text()
                .collect::<String>();
            tmp.split(' ').nth(1).unwrap().to_string()
        };
        let password_str: String = {
            let tmp: String = fragment
                .select(&password)
                .next()
                .expect("No password found for CSS Selector!")
                .text()
                .collect::<String>();
            tmp.split(' ').nth(1).unwrap().to_string()
        };

        println!(
            "URL: {}",
            url
        );
        println!(
            "Config File: {}",
            matches.value_of("file").expect("No config file given!")
        );
        println!("Username: {}", username_str);
        println!("Password: {}", password_str);

        let mut file = File::create(AUTH_FILE_LOCATION).unwrap();
        file.write_all(format!("{}\n{}", username_str, password_str).as_bytes())
            .expect("Error while writing to file!");
    }

    // Start OpenVPN process;
    let process = Exec::cmd("openvpn")
        .arg("--config")
        .arg(matches.value_of("file").expect("No config file provided!"))
        .arg("--auth-user-pass")
        .arg(AUTH_FILE_LOCATION);
    println!("Starting process {:?}!", process.to_cmdline_lossy());

    // Connect OpenVPN Output to program output;
    let mut output = io::stdout();
    let mut proc_output = process.stream_stdout().unwrap();
    io::copy(&mut proc_output, &mut output).unwrap();

    Ok(())
}
