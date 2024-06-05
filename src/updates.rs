#![allow(unused_imports, dead_code)]

use reqwest::Client;
use semver::Version;
use serde_derive::Deserialize;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const API_URL: &str = "https://api.github.com/repos/uzervlad/rosu-np/releases";

#[derive(Deserialize)]
struct GithubRelease {
  html_url: String,
  tag_name: String,
}

pub async fn check_for_updates() {
  println!("Checking for updates...");

  let client = Client::new();

  match client.get(API_URL)
    .header("User-Agent", "rosu-np")
    .send()
    .await 
  {
    Ok(res) => match res.json::<Vec<GithubRelease>>().await {
      Ok(releases) => {
        let latest = match releases.first() {
          Some(release) => release,
          None => {
            println!("No releases found?");
            return;
          },
        };

        let current_version = Version::parse(CURRENT_VERSION).unwrap();
        let latest_version = match Version::parse(&latest.tag_name) {
          Ok(version) => version,
          Err(e) => {
            println!("Unexpected error while parsing release version: {}", e);
            return;
          },
        };

        if latest_version > current_version {
          println!("Update available: {} -> {}", current_version, latest_version);
          println!("Download: {}", latest.html_url);
          println!();
        } else {
          println!("You are running the latest version: {}", current_version);
        }
      },
      Err(e) => {
        println!("Unable to parse releases JSON: {}", e);
      }
    },
    Err(e) => {
      println!("Unable to fetch latest release: {}", e);
    }
  }
}