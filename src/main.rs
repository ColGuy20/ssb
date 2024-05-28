#![allow(non_snake_case)] //This removes the warning from non snake case variable names

//Imports
use serde::{Deserialize, Serialize}; //Used for serializing (Convert for easy storage and use) data
use serde_json::json; //Used for creating JSon files
use reqwest::Error; //Used for handling errors related to HTTP requests
use tokio::time::{sleep, Duration}; //Used for async/await and time-based operations such as sleep
use std::env; //Used for environmental variables

//Storing the data
#[derive(Debug, Serialize, Deserialize)] //This stores data under ScoreStats
struct ScoreStats {
    totalScore: i64,
    totalRankedScore: i64,
    averageRankedAccuracy: f64,
    totalPlayCount: i64,
    rankedPlayCount: i64,
    replaysWatched: i64,
}
//
#[derive(Debug, Serialize, Deserialize)] //This is where all the data is stored (PlayerData)
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

//Function used to fetch/take in the data
async fn fetch_player_data() -> Result<PlayerData, Error> { //Name of function and stating return type
    let url = "https://scoresaber.com/api/player/76561199396123565/full"; //The url used to take in date (In this case it is my username)
    let response = reqwest::get(url).await?.json::<PlayerData>().await?; //Line to actually take the data in
    Ok(response) //Returns response
}

//Function to send data to discord
async fn send_to_discord(data: &PlayerData, success_count: &mut i64) -> Result<(), Error> { // Name of function and stating return type
    let webhook_url = env::var("WEBHOOK_URL").unwrap_or_else(|_| String::from("Invalid webhook URL")); //Pulls WEBHOOK_URL from environment, if unable then prints invalid
    if webhook_url == "Invalid webhook URL" {
        println!("{}", webhook_url);
    }
    
    let client = reqwest::Client::new(); //Setting the client
    
    let firstSeen_formatted = &data.firstSeen[0..10]; //Condencing the firstSeen variable to exclude time

    //Formatting the data being sent
    let payload = json!({ //The data is put in a JSon file for the discord webhook
        "embeds": [{
            "author": {
                "name": format!("{} #{}", data.name, data.rank), //Username and global rank
                "icon_url": format!("{}",data.profilePicture) //Profile Picture
            },
            "color": 5505024,
            "fields": [
                {
                    "name": "Description",
                    "value": format!("Country Rank: **#{}** ({})\nFirst Seen: {}", data.countryRank, data.country, firstSeen_formatted) //Country, Country Rank, and date first seen
                },
                {"name": "","value": ""}, //These are just for cosmetic purposes, they ensure that only two fields are in each row
                {
                    "name": "Total Score",
                    "value": format!("{}", data.scoreStats.totalScore), //Total Score
                    "inline": true //This states that this field and up to 2 other fields are in the same row
                },
                {
                    "name": "Total Ranked Score",
                    "value": format!("{}", data.scoreStats.totalRankedScore), //Total Ranked Score
                    "inline": true             
                },
                {"name": "","value": ""},
                {
                    "name": "Average Ranked Accuracy",
                    "value": format!("%{}", data.scoreStats.averageRankedAccuracy), //Average Ranked Accuracy
                    "inline": true
                },
                {
                    "name": "Performance Point (PP)",
                    "value": format!("{}", data.pp), //Performance Points
                    "inline": true
                },
                {"name": "","value": ""},
                {
                    "name": "Ranked Play Count",
                    "value": format!("{}", data.scoreStats.rankedPlayCount), //Ranked Play Count
                    "inline": true
                },
                {
                    "name": "Total Play Count",
                    "value": format!("{}", data.scoreStats.totalPlayCount), //Total Play Count
                    "inline": true             
                }
            ]
        }]
    });
    
    //Sends to discord
    let response = client.post(webhook_url)
        .json(&payload)
        .send()
        .await?;
    
    //Checks if it worked properly
    if response.status().is_success() {
        *success_count += 1; // Increment the success count
        println!("Message sent successfully to Discord [Success Count: {}]", success_count); // Prints if successful
    } else {
        println!("Failed to send message to Discord: {}", response.status()); //Prints if failure
    }
    
    Ok(()) //Returns OK
}

//Main function
#[tokio::main]
async fn main() {
    let mut success_count: i64 = 0; //Goes up every time the loop runs
    loop { //Loops every 10 minutes
        match fetch_player_data().await { //Checks if fetching data goes successfully
            Ok(data) => {
                if let Err(e) = send_to_discord(&data, &mut success_count).await {
                    println!("Error sending to Discord: {}", e); //Prints if sending to discord results in failure
                }
            },
            Err(e) => println!("Error: {}", e), //Prints if fetching data results in failure
        }
        sleep(Duration::from_secs(6)).await; //Waits 10 minutes before looping
    }
}