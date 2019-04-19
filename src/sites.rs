use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::scraper::{Html, Selector};
pub trait FreeVPN {
    fn get_site(&self) -> String;
    fn get_username(&self, fragment: &Html) -> String;
    fn get_password(&self, fragment: &Html) -> String;
}

pub struct VpnHandle {
    site: String,
    username_css_selector: String,
    password_css_selector: String,
}

impl VpnHandle {
    pub fn new(site: String, username_css_selector: String, password_css_selector: String) -> Self {
        VpnHandle {
            site,
            username_css_selector,
            password_css_selector,
        }
    }

    // Get HTML Page; scrape username & pasword and write to file;
    pub fn fetch_infos(
        &self,
        auth_file: &str,
        matches: &crate::ArgMatches,
    ) -> Result<(), reqwest::Error> {
        let mut resp = reqwest::get(self.get_site().as_str())?;
        assert!(resp.status().is_success());

        let body = resp.text().unwrap();
        let fragment = Html::parse_document(&body);
        let username = self.get_username(&fragment);
        let password = self.get_password(&fragment);

        println!("URL: {}", self.get_site());
        println!(
            "Config File: {}",
            matches.value_of("file").expect("No config file given!")
        );
        println!("Username: {}", username);
        println!("Password: {}", password);

        let mut file = File::create(auth_file).unwrap();
        file.write_all(format!("{}\n{}", username, password).as_bytes())
            .expect("Error while writing to file!");
        Ok(())
    }
}

impl FreeVPN for VpnHandle {
    fn get_site(&self) -> String {
        self.site.clone()
    }
    fn get_username(&self, fragment: &Html) -> String {
        let selector: Selector = Selector::parse(&self.username_css_selector)
            .expect("Invalid CSS Selector for username!");
        fragment
            .select(&selector)
            .next()
            .expect("No Username found!")
            .text()
            .collect::<String>()
            .split(' ')
            .nth(1)
            .unwrap()
            .to_string()
    }
    fn get_password(&self, fragment: &Html) -> String {
        let selector: Selector = Selector::parse(&self.password_css_selector)
            .expect("Invalid CSS Selector for password!");
        fragment
            .select(&selector)
            .next()
            .expect("No Password found!")
            .text()
            .collect::<String>()
            .split(' ')
            .nth(1)
            .unwrap()
            .to_string()
    }
}

fn get_url_base_of_config(config: &str) -> Option<String> {
    Some(
        Path::new(config)
            .file_stem()?
            .to_str()?
            .split("-")
            .nth(0)?
            .to_string()
            .replace(",", ".")
            .to_lowercase()
            .to_string(),
    )
}

pub fn get_handle_for_config(config: &str) -> Option<VpnHandle> {
    match get_url_base_of_config(config)?.as_str() {
        "freevpn.me" => Some(VpnHandle{
            site: "https://freevpn.me/accounts".to_string(),
            username_css_selector: "div.span4:nth-child(3) > ul:nth-child(1) > li:nth-child(1)".to_string(),
            password_css_selector: "div.span4:nth-child(3) > ul:nth-child(1) > li:nth-child(2)".to_string()
        }),
        "freevpn.se" => Some(VpnHandle{
            site: "https://freevpn.se/accounts".to_string(),
            username_css_selector: "div.row:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(1)".to_string(),
            password_css_selector: "div.row:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(2)".to_string()
        }),
        "freevpn.im" => Some(VpnHandle{
            site: "https://freevpn.im/accounts".to_string(),
            username_css_selector: "div.row:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(1)".to_string().to_string(),
            password_css_selector: "div.row:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(2)".to_string().to_string(),
        }),
        "freevpn.it" => Some(VpnHandle{
            site: "https://freevpn.it/accounts".to_string(),
            username_css_selector: "div.row:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(1)".to_string(),
            password_css_selector: "div.row:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(2)".to_string()
        }),
        "freevpn.be" => Some(VpnHandle{
            site: "https://freevpn.be/accounts/".to_string(),
            username_css_selector: "div.row:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(1)".to_string(),
            password_css_selector: "div.row:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(2)".to_string()
        }),
        "freevpn.co.uk" => Some(VpnHandle{
            site: "https://freevpn.co.uk/accounts/".to_string(),
            username_css_selector: ".plan > div:nth-child(2) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(1)".to_string(),
            password_css_selector: ".plan > div:nth-child(2) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(2)".to_string()
        }),
        "freevpn.eu" => Some(VpnHandle{
            site: "https://freevpn.eu/accounts/".to_string(),
            username_css_selector: "div.row:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(1)".to_string(),
            password_css_selector: "div.row:nth-child(3) > div:nth-child(1) > div:nth-child(1) > div:nth-child(1) > ul:nth-child(2) > li:nth-child(2)".to_string()
        }),
        _ => None,
    }
}
