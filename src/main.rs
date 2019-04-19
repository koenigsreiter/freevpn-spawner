extern crate clap;
extern crate ctrlc;
extern crate reqwest;
extern crate scraper;
extern crate subprocess;

mod sites;

use clap::{App, Arg, ArgMatches};
use std::io;
use subprocess::Exec;

use crate::sites::FreeVPN;

fn main() -> Result<(), reqwest::Error> {
    const AUTH_FILE_LOCATION: &str = "/tmp/freevpn-auth.txt";

    // CLI-Parameters
    let matches: ArgMatches = App::new("FreeVPN spawner")
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
                .value_name("FREEVPN_URL")
                .help("The URL to the FreeVPN accounts page")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username_css_selector")
                .short("u")
                .long("username-selector")
                .value_name("USERNAME_CSS_SELECTOR")
                .help("The CSS Selector for the username")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password_css_selector")
                .short("p")
                .long("password-selector")
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

    let handle: sites::VpnHandle = match sites::get_handle_for_config(
        matches.value_of("file").expect("No Config file given!"),
    ) {
        Some(handle) => handle,
        None => sites::VpnHandle::new(
            matches.value_of("url").expect("No URL given!").to_string(),
            matches
                .value_of("username_css_selector")
                .expect("No Username CSS Selector given!")
                .to_string(),
            matches
                .value_of("password_css_selector")
                .expect("No Password CSS Selector given!")
                .to_string(),
        ),
    };
    handle
        .fetch_infos(AUTH_FILE_LOCATION, &matches)
        .expect(format!("Error while making the request to {}", handle.get_site()).as_str());

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
