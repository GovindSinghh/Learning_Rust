use clap::{Parser,Subcommand};
use reqwest::{self, Url};
use owo_colors::OwoColorize;
use serde_json::Value;
const API_KEY:&str="cf919ec7a29d5ab4c5c31899c19e7987";

#[derive(Parser)]
#[command(name="Weather App CLI",version="1.0",about="A CLI app for Weather Information")]
struct CLI{
    #[command(subcommand)]
    command:Commands
}

#[derive(Subcommand)]
enum Commands{
    CurrentWeather{
        #[arg(short,long)]
        city:String,
    }

}

#[tokio::main]
async fn main() {
    let cli=CLI::parse();
    match cli.command{
        Commands::CurrentWeather{ city} =>{
            let data=fetch_current_weather(&city).await;

            println!("\n{} {} {}", "🌥️"," Weather Summary of".cyan().bold(),city.bright_yellow());
            println!("{}\t{}\t{}\t{}", 
                "Temp".magenta().bold(), 
                "feels".magenta().bold(), 
                "Humidity".magenta().bold(),
                "Wind-Speed".magenta().bold()
            );
            println!("{}\t{}\t{}\t\t{}", 
                data["main"]["temp"].green(), 
                data["main"]["feels_like"].green(), 
                data["main"]["humidity"].green(),
                data["wind"]["speed"].to_owned().green(),
            );
        }
    }

}

async fn fetch_current_weather(city:&str) ->Value{
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, API_KEY);
    let url = match Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            panic!("Invalid URL: {}", e);
        }
    };

    let res = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => {
            panic!("Request failed: {}", e);
        }
    };

    match res.json::<serde_json::Value>().await {
        Ok(json) => json,
        Err(e) => panic!("Failed to parse JSON: {}", e),
    }
}