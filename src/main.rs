#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json::json;
use reqwest::Error;
use tokio::time::{sleep, Duration};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct ScoreStats {
    totalScore: i64,
    totalRankedScore: i64,
    averageRankedAccuracy: f64,
    totalPlayCount: i64,
    rankedPlayCount: i64,
    replaysWatched: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerData {
    id: String,
    name: String,
    profilePicture: String,
    country: String,
    pp: f64,
    rank: i64,
    countryRank: i64,
    histories: String,
    banned: bool,
    inactive: bool,
    scoreStats: ScoreStats,
    firstSeen: String,
}

async fn fetch_player_data() -> Result<PlayerData, Error> {
    let url = "https://scoresaber.com/api/player/76561199396123565/full";
    let response = reqwest::get(url).await?.json::<PlayerData>().await?;
    Ok(response)
}

async fn send_to_discord(data: &PlayerData) -> Result<(), Error> {
    let webhook_url = env::var("WEBHOOK_URL").unwrap_or_else(|_| String::from("Invalid webhook URL"));
    if webhook_url == "Invalid webhook URL" {
        println!("{}", webhook_url);
    }
    
    let client = reqwest::Client::new();
    
    let firstSeen_formatted = &data.firstSeen[0..10];

    let payload = json!({
        "embeds": [{
            "author": {
                "name": format!("{} #{}", data.name, data.rank),
                "icon_url": format!("{}",data.profilePicture)
            },
            "color": 5505024,
            "fields": [
                {
                    "name": "Description",
                    "value": format!("Country Rank: **#{}** ({})\nFirst Seen: {}", data.countryRank, data.country, firstSeen_formatted)
                },
                {"name": "","value": ""},
                {
                    "name": "Total Score",
                    "value": format!("{}", data.scoreStats.totalScore),
                    "inline": true
                },
                {
                    "name": "Total Ranked Score",
                    "value": format!("{}", data.scoreStats.totalRankedScore),
                    "inline": true             
                },
                {"name": "","value": ""},
                {
                    "name": "Average Ranked Accuracy",
                    "value": format!("%{}", data.scoreStats.averageRankedAccuracy),
                    "inline": true
                },
                {
                    "name": "Performance Point (PP)",
                    "value": format!("{}", data.pp),
                    "inline": true             
                },
                {"name": "","value": ""},
                {
                    "name": "Ranked Play Count",
                    "value": format!("{}", data.scoreStats.rankedPlayCount),
                    "inline": true
                },
                {
                    "name": "Total Play Count",
                    "value": format!("{}", data.scoreStats.totalPlayCount),
                    "inline": true             
                }
            ]
        }]
    });
    
    let response = client.post(webhook_url)
        .json(&payload)
        .send()
        .await?;
    
    if response.status().is_success() {
        println!("Message sent successfully to Discord");
    } else {
        println!("Failed to send message to Discord: {}", response.status());
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    loop {
        match fetch_player_data().await {
            Ok(data) => {
                if let Err(e) = send_to_discord(&data).await {
                    println!("Error sending to Discord: {}", e);
                }
            },
            Err(e) => println!("Error: {}", e),
        }
        
        sleep(Duration::from_secs(600)).await;
    }
}