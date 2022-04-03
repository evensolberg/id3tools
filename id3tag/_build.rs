// Example build script. Doesn't really do what I need, though. Argh!

use serde::Deserialize;
use std::{env, error::Error, fs::File, io::Read};
use toml;

#[derive(Debug, Default, Clone, Deserialize)]
struct Cargo {
    name: String,
    version: String,
    authors: String,
    description: String,
    homepage: String,
    repository: String,
    license: String,
    license_file: String,
    keywords: String,
    edition: String,
}

impl Cargo {
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the config from the supplied TOML file.
    fn load_config(filename: &str) -> Result<Self, Box<dyn Error>> {
        let mut config_toml = String::new();

        let mut file = File::open(&filename)
            .map_err(|err| format!("Config file {} not found. Error: {}", filename, err))?;
        println!("file = {:?}", file);

        file.read_to_string(&mut config_toml)?;
        let config = match toml::from_str(&config_toml) {
            Ok(config) => config,
            Err(err) => {
                println!(
                    "Unable to read config from {}. Using defaults. Error message: {}",
                    filename,
                    err.to_string()
                );
                Self::new()
            }
        };
        println!("{:?}", config);

        Ok(config)
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Cargo::load_config("Cargo.toml")?;
    println!("Cargo: {:?}", config);

    // println!("cargo:rustc-env=ID3TAG_VER={}", config.version);
    println!("cargo:rustc-env=ID3TAG_VER={}", "0.11.2");

    env::set_var("ID3TAG_NAME", config.name);
    // env::set_var("ID3TAG_VER", config.version);
    env::set_var("ID3TAG_AUTHORS", config.authors);
    env::set_var("ID3TAG_DESCRIPTION", config.description);
    env::set_var("ID3TAG_HOMEPAGE", config.homepage);
    env::set_var("ID3TAG_REPOSITORY", config.repository);
    env::set_var("ID3TAG_LICENSE", config.license);
    env::set_var("ID3TAG_LICENSE_FILE", config.license_file);
    env::set_var("ID3TAG_KEYWORDS", config.keywords);
    env::set_var("ID3TAG_EDITION", config.edition);

    // assert!(env::var("ID3TAG_VER").is_ok());

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0, // everying is hunky dory - exit with code 0 (success)
        Err(err) => {
            println!("{}", err.to_string().replace("\"", ""));
            1 // exit with a non-zero return code, indicating a problem
        }
    });
}
