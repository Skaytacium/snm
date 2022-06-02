use clap::Parser;
mod ui;

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum Lts {
    Version(String),
    False(bool),
}

#[derive(serde::Deserialize)]
struct Data {
    version: String,
    lts: Lts,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // (OS, extension)
    let file = match std::env::consts::OS {
        "linux" => ("linux-x64", "tar.xz"),
        "windows" => ("win-x64", "zip"),
        "macos" => ("darwin-x64", "tar.xz"),
        _ => panic!("OS not supported"),
    };

    let resp = reqwest::get("https://nodejs.org/dist/index.json")
        .await?
        .json::<Vec<Data>>()
        .await?;

    let latest = resp.get(0).unwrap().version.clone();
    let mut lts: Option<String> = None;

    for entry in &resp {
        match &entry.lts {
            Lts::False(_) => continue,
            Lts::Version(_) => {
                lts = Some(entry.version.clone());
                break;
            }
        }
    }

    drop(resp);

    let cli = ui::Cli::parse();

    Ok(())
}