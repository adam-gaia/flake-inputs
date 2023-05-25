use anyhow::bail;
use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Original {
    #[serde(rename = "type")]
    type_: String,
    url: String,
    owner: Option<String>,
    repo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Locked {
    #[serde(rename = "lastModified")]
    last_modified: u64,
    #[serde(rename = "narHash")]
    nar_hash: String,
    #[serde(rename = "type")]
    type_: String,
    url: Option<String>,
    owner: Option<String>,
    repo: Option<String>,
    rev: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    inputs: serde_json::Map<String, serde_json::Value>,
    locked: Locked,
    original: Original,
}

#[derive(Debug, Serialize, Deserialize)]
struct Locks {
    nodes: serde_json::Map<String, Node>,
    root: String,
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct FlakeInputs {
    original: Original,
    #[serde(rename = "originalUrl")]
    original_url: String,
    resolved: Original,
    #[serde(rename = "resolvedUrl")]
    resolved_url: String,
    locked: Locked,
    #[serde(rename = "lockedUrl")]
    locked_url: Option<String>,
    description: String,
    path: String,
    revision: Option<String>,
    #[serde(rename = "revCount")]
    rev_count: Option<u64>,
    #[serde(rename = "lastModified")]
    last_modified: u64,
    locks: Locks,
}

impl FlakeInputs {
    fn from(buffer: Vec<u8>) -> Result<Self> {
        let inputs = serde_json::from_slice(&buffer)?;
        Ok(inputs)
    }

    fn print_names(&self) {
        for input_name in self.locks.nodes.keys() {
            println!("{}", input_name);
        }
    }

    fn print_urls(&self) -> Result<()> {
        for input_name in self.locks.nodes.keys() {
            let input = self.locks.nodes.get(input_name).unwrap();

            let url = todo!("Need to reconstruct the url from the node's input and locked fields");

            println!("{}", url);
        }
        Ok(())
    }

    fn print_names_and_urls(&self) {
        for input_name in self.locks.nodes.keys() {
            todo!("get url like in get_url()");
            println!("{}: {}", input_name, url);
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Name,
    Url,
    Lookup { name: String },
}

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut buffer = Vec::new();
    match io::stdin().read_to_end(&mut buffer) {
        Ok(_) => {
            let inputs = match FlakeInputs::from(buffer) {
                Ok(inputs) => inputs,
                Err(_) => bail!("Unable to parse flake inputs json from stdin"),
            };
            match args.command {
                Some(command) => match command {
                    Command::Name => {
                        inputs.print_names();
                    }
                    Command::Url => {
                        inputs.print_urls()?;
                    }
                    Command::Lookup { name } => {
                        let Some(input) = inputs.locks.nodes.get(&name) else {
                            bail!("Unable to find input {}", name);
                        };
                        let Some(url) = input.get("url") else {
                            bail!("Unable to get url for input {}", name);
                        };
                        println!("{}", url);
                    }
                },
                None => {
                    inputs.print_names_and_urls();
                }
            }
        }
        Err(_) => bail!("Unable to read stdin"),
    };
    Ok(())
}
