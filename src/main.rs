#![allow(non_snake_case)] //This removes the warning from non snake case variable names

//Imports
use serde::{Deserialize, Serialize}; //Used for serializing (Convert for easy storage and use) data
use serde_json::json; //Used for creating JSon files
use reqwest::Error; //Used for handling errors related to HTTP requests
use tokio::time::{sleep, Duration}; //Used for async/await and time-based operations such as sleep
use std::env; //Used for environmental variables
use rusqlite::{params, Connection, Result}; //Used to integrate database (SQLite) functions
use std::collections::HashMap; //Used to create instances without needing to write the whole path each time

//Storing the data from ScoreSaber API
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

//Struct for changes in new-old
#[derive(Debug, Serialize, Deserialize)]
struct Changes{
    pp: bool,
    pp_change: f64,
    rAccuracy: bool,
    rAccuracy_change: f64,
    rank: bool,
    rank_change: i64,
    cRank: bool,
    cRank_change: i64,
    tScore: bool,
    tScore_change: i64,
    rScore: bool,
    rScore_change: i64,
    tCount: bool,
    tCount_change: i64,
    rCount: bool,
    rCount_change: i64,
    replays: bool,
    replays_change: i64,
}

//Function used to fetch/take in the data from Scoresaber
async fn fetch_player_data() -> Result<PlayerData, Error> { //Name of function and stating return type
    let id = "76561199396123565"; //My steam/score saber ID
    let url = "https://scoresaber.com/api/player/".to_owned()+id+"/full"; //The url used to take in date (In this case it is my username)
    let response = reqwest::get(url).await?.json::<PlayerData>().await?; //Line to actually take the data in
    Ok(response) //Returns response
}

// Function to insert data into database
fn insert_player_data(conn: &Connection, data: &PlayerData) -> Result<()> {
    //Creates a table if there is none
    conn.execute(
        "CREATE TABLE IF NOT EXISTS player_data (
            id TEXT PRIMARY KEY,
            name TEXT,
            profilePicture TEXT,
            country TEXT,
            pp REAL,
            rank INTEGER,
            countryRank INTEGER,
            histories TEXT,
            banned INTEGER,
            inactive INTEGER,
            totalScore INTEGER,
            totalRankedScore INTEGER,
            averageRankedAccuracy REAL,
            totalPlayCount INTEGER,
            rankedPlayCount INTEGER,
            replaysWatched INTEGER,
            firstSeen TEXT
        )",
        params![],
    )?;
    //Inserts or Replaces data in the data base
    conn.execute(
        "INSERT OR REPLACE INTO player_data (
            id, name, profilePicture, country, pp, rank, countryRank, histories, banned, inactive, 
            totalScore, totalRankedScore, averageRankedAccuracy, totalPlayCount, rankedPlayCount, 
            replaysWatched, firstSeen
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
        params![
            data.id,
            data.name,
            data.profilePicture,
            data.country,
            data.pp,
            data.rank,
            data.countryRank,
            data.histories,
            data.banned,
            data.inactive,
            data.scoreStats.totalScore,
            data.scoreStats.totalRankedScore,
            data.scoreStats.averageRankedAccuracy,
            data.scoreStats.totalPlayCount,
            data.scoreStats.rankedPlayCount,
            data.scoreStats.replaysWatched,
            data.firstSeen
        ],
    )?;
    Ok(())
}

//Fetch player data from database
fn fetch_player_data_from_db(conn: &Connection) -> Result<PlayerData, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT *, name FROM player_data")?; // Prepare the query
    let mut rows = stmt.query(params![])?; // Execute the query

    if let Some(row) = rows.next()? { // Fetch the first row
        let player_data = PlayerData { // Map the row to the PlayerData struct
            id: row.get(0)?,
            name: row.get(1)?,
            profilePicture: row.get(2)?,
            country: row.get(3)?,
            pp: row.get(4)?,
            rank: row.get(5)?,
            countryRank: row.get(6)?,
            histories: row.get(7)?,
            banned: row.get(8)?,
            inactive: row.get(9)?,
            scoreStats: ScoreStats {
                totalScore: row.get(10)?,
                totalRankedScore: row.get(11)?,
                averageRankedAccuracy: row.get(12)?,
                totalPlayCount: row.get(13)?,
                rankedPlayCount: row.get(14)?,
                replaysWatched: row.get(15)?,
            },
            firstSeen: row.get(16)?,
        };
        Ok(player_data) // Return the single PlayerData
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows) // Handle the case where no rows are returned
    }
}

//Comparing new-old data main function
fn compare_data(data: &PlayerData, data_from_db: &PlayerData) -> Result<Changes, Error>{
    //Serializing data for comparing
    let serialized_data: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data)).unwrap();
    let serialized_data_from_db: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data_from_db)).unwrap();
    let serialized_scoreStats: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data.scoreStats)).unwrap();
    let serialized_scoreStats_from_db: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data_from_db.scoreStats)).unwrap();

    //Make instance variable based of Changes struct
    let mut changes = Changes {
        pp: false,
        pp_change: 0.0,
        rAccuracy: false,
        rAccuracy_change: 0.0,
        rank: false,
        rank_change: 0,
        cRank: false,
        cRank_change: 0,
        tScore: false,
        tScore_change: 0,
        rScore: false,
        rScore_change: 0,
        tCount: false,
        tCount_change: 0,
        rCount: false,
        rCount_change: 0,
        replays: false,
        replays_change: 0,
    };

    //Checks and responds if data changed
    for (key, value) in &serialized_data { //Loops to check differences in all data
        if let Some(value_from_db) = serialized_data_from_db.get(key){ //If the data exists in both
            //Make variable that excludes certain fields of PlayerData
            let exclude: bool = matches!(key.as_str(),"id"|"name"|"country"|"firstSeen"|"banned"|"inactive"|"profilePicture"|"histories");
            if value != value_from_db && !exclude { //If the old data does not match the new data and is not excluded
                if key == "scoreStats" { //If the key being checked is scoreStats
                    for (key, value) in &serialized_scoreStats { //Loops to check differences in scoreStats
                        if let Some(value_from_db) = serialized_scoreStats_from_db.get(key) { //If the data exists in both
                            if value != value_from_db { //If the data is different
                                if key == "averageRankedAccuracy" { //Since this is a f64, then it is seperate
                                    changes.rAccuracy = true; //There is a change
                                    changes.rAccuracy_change = value.as_f64().unwrap() - value_from_db.as_f64().unwrap(); //Puts change in the instance
                                } else { //If the data in scoreStats is not averageRankedAccuracy
                                    let change = value.as_i64().unwrap() - value_from_db.as_i64().unwrap();
                                    match key.as_str() { //Responds according to the variable (identified by the key)
                                        "totalScore" => { //If the key is "totalScore" and so on..
                                            changes.tScore = true; //There is a change
                                            changes.tScore_change = change; //Puts the change in the instance
                                        }
                                        "totalRankedScore" => {
                                            changes.rScore = true;
                                            changes.rScore_change = change;
                                        }
                                        "totalPlayCount" => {
                                            changes.tCount = true;
                                            changes.tCount_change = change;
                                        }
                                        "rankedPlayCount" => {
                                            changes.rCount = true;
                                            changes.rCount_change = change;
                                        }
                                        "replaysWatched" => {
                                            changes.replays = true;
                                            changes.replays_change = change;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                } else { //If the key being checked is not in score stats
                    match key.as_str() { //Responds according to the variable (identified by the key)
                        "pp" => { //If the key is "pp" and so on...
                            changes.pp = true; //There is a change
                            changes.pp_change = value.as_f64().unwrap() - value_from_db.as_f64().unwrap();//Puts the change in the instance
                        }
                        "rank" => {
                            changes.rank = true;
                            changes.rank_change = value.as_i64().unwrap() - value_from_db.as_i64().unwrap();
                        }
                        "countryRank" => {
                            changes.cRank = true;
                            changes.cRank_change = value.as_i64().unwrap() - value_from_db.as_i64().unwrap();
                        }
                        _ => {}
                    }
                }
            }
        } 
    }
    Ok(changes) //Return information about chnages
}

//Function to add commas
fn add_commas(mut num: i64, include_pos: bool) -> String {
    let mut result = String::new(); //Sets new string
    let mut count = 0; //Sets up count (Every 3 nums)
    let is_negative = num < 0; // Check if the number is negative
    if is_negative { // If negative
        num = -num; // Work with the absolute value
    }
    if num == 0 { //If num is 0
        result.push('0'); // Handle the case where num is 0
    }
    while num > 0 { //while there is still more to the number
        if count != 0 && count % 3 == 0 { //If the count is divisible by three then add comma
            result.insert(0, ',');
        }
        result.insert(0, (b'0' + (num % 10) as u8) as char); //adds num to answer (result)
        num /= 10; //shifts to next place
        count += 1; //increment
    }
    if is_negative { //If negative
        result.insert(0, '-'); // Add negative sign if the number was negative
    }
    else if include_pos{ //If positive and positive sign is wanted
        result.insert(0, '+'); // Add postive sign
    }
    result //return answer
}

//Function to send data to discord
async fn send_to_discord(data: &PlayerData, changes: &Changes, success_count: &mut i64) -> Result<(), Error> { // Name of function and stating return type
    let webhook_url = env::var("WEBHOOK_URL").unwrap_or_else(|_| String::from("Invalid webhook URL")); //Pulls WEBHOOK_URL from environment, if unable then prints invalid
    if webhook_url == "Invalid webhook URL" {
        println!("{}", webhook_url);
    }
    
    let client = reqwest::Client::new(); //Setting the client

    //Formatting data
    let mut totalScore_formatted = add_commas(data.scoreStats.totalScore, false); //add commas
    let mut totalRankedScore_formatted = add_commas(data.scoreStats.totalRankedScore, false); //add commas
    let mut totalPlayCount_formatted = add_commas(data.scoreStats.totalPlayCount, false); //add commas
    let mut rankedPlayCount_formatted = add_commas(data.scoreStats.rankedPlayCount, false); //add commas
    let mut rank_formatted = add_commas(data.rank, false); //add commas
    let mut countryRank_formatted = add_commas(data.countryRank, false); //add commas
    let mut replaysWatched_formatted = format!("{}", data.scoreStats.replaysWatched); //Make formatted var
    let mut pp_formatted = format!("{}", data.pp); //Make formatted var
    let mut averageRankedAccuracy_formatted = format!("{}", data.scoreStats.averageRankedAccuracy); //Make formatted var
    let firstSeen_formatted = &data.firstSeen[0..10]; //Condencing the firstSeen variable to exclude time

    //Implementing changes
    if changes.tScore{ //If there is a change
        totalScore_formatted += &format!("\n`{}`", add_commas(changes.tScore_change, true)); //Add the change after adding commas to the change
    }
    if changes.rScore{
        totalRankedScore_formatted += &format!("\n`{}`", add_commas(changes.rScore_change, true));
    }
    if changes.tCount{
        totalPlayCount_formatted += &format!(" `{}`", add_commas(changes.tCount_change, true));
    }
    if changes.rCount{
        rankedPlayCount_formatted += &format!(" `{}`", add_commas(changes.rCount_change, true));
    }
    if changes.replays{
        replaysWatched_formatted += &format!("\n`{}`", add_commas(changes.replays_change, true));
    }
    if changes.rank{
        rank_formatted += &format!(" `{}`", add_commas(changes.rank_change, true));
    }
    if changes.cRank{
        countryRank_formatted += &format!(" `{}`", add_commas(changes.cRank_change, true));
    }
    if changes.pp{
        pp_formatted += &format!("\n`{}`", changes.pp_change); //Add the change
    }
    if changes.rAccuracy{
        averageRankedAccuracy_formatted += &format!("\n`{}`", changes.rAccuracy_change);
    }

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
                    "value": format!("Rank: **#{}**\nCountry Rank ({}): **#{}**\nFirst Seen: {}", rank_formatted, data.country, countryRank_formatted, firstSeen_formatted) //Rank, Country/Rank, and date first seen
                },
                {"name": "","value": ""}, //These are just for cosmetic purposes, they ensure that only two fields are in each row
                {
                    "name": "Total Score",
                    "value": format!("{}", totalScore_formatted), //Total Score
                    "inline": true //This states that this field and up to 2 other fields are in the same row
                },
                {
                    "name": "Total Ranked Score",
                    "value": format!("{}", totalRankedScore_formatted), //Total Ranked Score
                    "inline": true             
                },
                {"name": "","value": ""},
                {
                    "name": "Average Ranked Accuracy",
                    "value": format!("%{}", averageRankedAccuracy_formatted), //Average Ranked Accuracy
                    "inline": true
                },
                {
                    "name": "Performance Point (PP)",
                    "value": format!("{}", pp_formatted), //Performance Points
                    "inline": true
                },
                {"name": "","value": ""},
                {
                    "name": "Ranked Play Count",
                    "value": format!("{} ", rankedPlayCount_formatted), //Ranked Play Count
                    "inline": true
                },
                {
                    "name": "Total Play Count",
                    "value": format!("{} ", totalPlayCount_formatted), //Total Play Count
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
    let conn = Connection::open("player_data.db").expect("Failed to open database"); //Set up connection
    loop { //Loops every 10 minutes
        match fetch_player_data_from_db(&conn) {  //Checks if fetching data from database goes successfully
            Ok(data_from_db) => {
                match fetch_player_data().await { //Checks if fetching data from ScoreSabere API goes successfully
                    Ok(data) => {
                        if let Err(e) = insert_player_data(&conn, &data) { //Call function to insert data into databas
                            println!("Error inserting data into database: {}", e); //Prints if inputing data into database results in failure
                        }
                        match compare_data(&data, &data_from_db) { //Call function to compare data
                            Ok(changes) => {
                                if let Err(e) = send_to_discord(&data, &changes, &mut success_count).await { //Call function to send data to discord
                                println!("Error sending to Discord: {}", e); //Prints if sending to discord results in failure
                                }
                            }
                            Err(e) => println!("Error comparing data: {}", e), //Prints if comparing data fails
                        }
                    },
                    Err(e) => println!("Error: {}", e), //Prints if fetching data from ScoreSaber API results in failure
                }
            }
            Err(err) => {
                eprintln!("Error: {:?}", err); //Prints if fetching data from database results in failure
            }
        }
        let cooldown:u64 = 600; //Sets cooldown (in secs, 10 mins)
        sleep(Duration::from_secs(cooldown)).await; //Waits 10 minutes before looping
    }
}