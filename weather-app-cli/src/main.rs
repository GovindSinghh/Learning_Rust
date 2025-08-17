use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
mod methods;
use methods::*;

#[derive(Parser)]
#[command(
    name = "Weather App CLI",
    version = "1.0",
    about = "A CLI app for Weather Information"
)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CurrentWeather {
        #[arg(short, long)]
        city: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = CLI::parse();
    match cli.command {
        Commands::CurrentWeather { city } => {
            let data = fetch_current_weather(&city).await;

            println!(
                "\n{} {} {}",
                "🌥️",
                " Weather Summary of".cyan().bold(),
                city.bright_yellow()
            );
            println!(
                "{}\t{}\t{}\t{}",
                "Temp".magenta().bold(),
                "feels".magenta().bold(),
                "Humidity".magenta().bold(),
                "Wind-Speed".magenta().bold()
            );
            println!(
                "{}\t{}\t{}\t\t{}",
                data["main"]["temp"].green(),
                data["main"]["feels_like"].green(),
                data["main"]["humidity"].green(),
                data["wind"]["speed"].to_owned().green(),
            );
        }
    }
}
