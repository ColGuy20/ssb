#![allow(non_snake_case)] //This removes the warning from non snake case variable names

//Adding modules for functions
mod imports; //Imports needed for program
    pub use imports::*; //Use the imports
mod datatweaks; //Fetching and inserting data
mod compare; //Comparing the new data and the old data (from the database)
mod send; //Formatting and sending data to the discord webhook

//Storing the data from ScoreSaber API
#[derive(Debug, Serialize, Deserialize)] //This stores data under ScoreStats
pub struct ScoreStats {
    totalScore: i64,
    totalRankedScore: i64,
    averageRankedAccuracy: f64,
    totalPlayCount: i64,
    rankedPlayCount: i64,
    replaysWatched: i64,
}
//
#[derive(Debug, Serialize, Deserialize)] //This is where all the data is stored (PlayerData)
pub struct PlayerData {
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
#[derive(Debug, Serialize, Deserialize, Default)] //This struct has default in case of new user (for send_stats_to_discord)
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

//Main function
#[tokio::main]
async fn main() {
    let player_id = "76561199576904140"; // Steam/ScoreSaber ID (In this case mine)
    let mut success_count: i64 = 0; //Goes up every time the loop runs
    let conn = Connection::open("player_data.db").expect("Failed to open database"); //Set up connection
    loop { //Loops every 10 minutes
        match datatweaks::fetch_player_data_from_db(&conn, player_id) {  //Checks if fetching data from database goes successfully
            Ok(data_from_db) => {
                match datatweaks::fetch_player_data(player_id).await { //Checks if fetching data from ScoreSabere API goes successfully
                    Ok(data) => {
                        if let Err(e) = datatweaks::insert_player_data(&conn, &data, &player_id) { //Call function to insert data into databas
                            println!("Error inserting data into database: {}", e); //Prints if inputing data into database results in failure
                        }
                        match compare::compare_data(&data, &data_from_db) { //Call function to compare data
                            Ok(changes) => {
                                if let Err(e) = send::send_stats_to_discord(&data, &changes, &mut success_count, false).await { //Call function to send data to discord
                                println!("Error sending to Discord: {}", e); //Prints if sending to discord results in failure
                                }
                            }
                            Err(e) => println!("Error comparing data: {}", e), //Prints if comparing data fails
                        }
                    },
                    Err(e) => println!("Error: {}", e), //Prints if fetching data from ScoreSaber API results in failure
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => { //If player data is not in database
                // If no player data exists in the database, fetch it from the API and insert it
                match datatweaks::fetch_player_data(player_id).await { //Fetch data from ScoreSaber API
                    Ok(data) => {
                        // Insert new data into the database
                        if let Err(e) = datatweaks::insert_player_data(&conn, &data, &player_id) { //Call function to insert data into databas
                            println!("Error inserting data into database: {}", e); //Prints if inputing data into database results in failure
                        } else { //If inserting the player data into the database works
                            if let Err(e) = send::send_stats_to_discord(&data, &Changes::default(), &mut success_count, true).await { //Call function to send data to discord
                                println!("Error sending to Discord: {}", e); //Prints if sending to discord results in failure
                            }
                        }
                    }
                    Err(e) => println!("Error fetching player data from API: {}", e), //Prints if fetching data from ScoreSaber API results in failure
                }
            }
            Err(e) => println!("Error fetching player data from database: {}", e), //Prints if fetching data from database results in failure
        }
        let cooldown:u64 = 7200; //Sets cooldown (in secs, 10 mins)
        sleep(Duration::from_secs(cooldown)).await; //Waits 10 minutes before looping
    }
}