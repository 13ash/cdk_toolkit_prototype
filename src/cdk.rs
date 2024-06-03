use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use colored::*;
use dialoguer::Confirm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Display {
        #[arg(long, short)]
        input: String,
    }
}

#[derive(Debug, Deserialize)]
struct Resource {
    #[serde(rename = "ResourceName")]
    resource_name: String,
    #[serde(rename = "Service")]
    service: String,
    #[serde(rename = "Free-Tier Eligible")]
    free_tier_eligible: String,
    #[serde(rename = "Free-Tier Threshold")]
    free_tier_threshold: i32,
    #[serde(rename = "Usage")]
    usage: i32,
    #[serde(rename = "UnitCost")]
    unit_cost: f64,
    #[serde(rename = "Unit")]
    unit: String,
}

fn read_json_file<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Resource>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let resources: Vec<Resource> = serde_json::from_reader(reader)?;
    Ok(resources)
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Display { input } => {
            match read_json_file(input) {
                Ok(resources) => {
                    for resource in resources {
                        println!("{}: {}", "Resource".green(), resource.resource_name.cyan());
                        println!("  {}: {}", "Service".green(), resource.service.blue());
                        println!("  {}: {}", "Free-Tier Eligible".green(), resource.free_tier_eligible.yellow());
                        println!("  {}: {}", "Free-Tier Threshold".green(), resource.free_tier_threshold.to_string().yellow());
                        println!("  {}: {}", "Current Billing Cycle Usage".green(), resource.usage.to_string().magenta());
                        println!("  {}: {}", "Current Cost per Unit".green(), format!("{}", resource.unit_cost).red());
                        println!("  {}: {}", "Unit".green(), resource.unit);
                        println!();
                    }
                    println!("{}", "Please visit your billing console for more details https://console.aws.amazon.com/console/home?nc2=h_ct&src=header-signin".bold().bright_green());
                    if Confirm::new().with_prompt("Do you want to deploy?").interact().unwrap() {
                        println!("{}", "Deploying...".bold().green());

                    } else {
                        println!("{}", "Deployment aborted.".red().bold());
                    }
                }

                Err(e) => eprintln!("{}: {}", "Error reading file".red().bold(), e),
            }
        }
    }
}
