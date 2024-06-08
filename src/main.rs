#![allow(non_snake_case)] //This removes the warning from non snake case variable names

//Adding modules for functions
mod imports; //Imports needed for program
    pub use imports::*; //Use the imports
mod datatweaks; //Fetching and inserting data
mod compare; //Comparing the new data and the old data (from the database)
mod format; //Formatting data for the discord bot
mod message; //Take in message and respond to it

//Storing the data from ScoreSaber API
#[derive(Debug, Serialize, Deserialize, Default)] //This stores data under ScoreStats
pub struct ScoreStats {
    totalScore: i64,
    totalRankedScore: i64,
    averageRankedAccuracy: f64,
    totalPlayCount: i64,
    rankedPlayCount: i64,
    replaysWatched: i64,
}
//
#[derive(Debug, Serialize, Deserialize, Default)] //This is where all the data is stored (PlayerData)
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
#[derive(Debug, Serialize, Deserialize, Default)] //This struct has default in case of new user
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

pub struct Handler;// This struct is used for discord bot events
//
#[async_trait]
impl EventHandler for Handler{ //Implement EventHandler to handle discord events
    async fn ready(&self, _ctx: Context, ready: Ready) { //When discord triggers ready event
        println!("\n{} is connected!", ready.user.name); //Prints ready and name of bot
    }
    async fn message(&self, ctx: Context, msg: Message) { //Handle incoming messages
        let message = msg.content.as_str().split(' ').next().unwrap(); // First word of message taken in
        if message.starts_with("/") && msg.author.name != "RustBot" { //If message starts with a "/" (command) and message is not sent by rustbot
            message::react_to_msg(ctx, msg).await; //Function to react to message
        }
    }
}

//Function for sending or tracking stats
pub async fn send_stats(player_id: &str, ctx: Context, msg: Message) -> bool{
    let mut payload_new_user = true; //If new user
    let mut payload_data = PlayerData::default(); //Default data
    let mut payload_changes = Changes::default(); //Default changes
    let conn = Connection::open("player_data.db").expect("Failed to open database"); //Set up connection
    match datatweaks::fetch_player_data_from_db(&conn, player_id) {  //Checks if fetching data from database goes successfully
        Ok(data_from_db) => { //If functioned correctly
            match datatweaks::fetch_player_data(player_id).await { //Checks if fetching data from ScoreSabere API goes successfully
                Ok(data) => { //If functioned correctly
                    if let Err(e) = datatweaks::insert_player_data(&conn, &data, &player_id) { //Call function to insert data into databas
                        println!("Error inserting data into database: {}", e); //Prints if inputing data into database results in failure
                    }
                    match compare::compare_data(&data, &data_from_db) { //Call function to compare data
                        Ok(changes) => { //If functioned correctly
                            payload_new_user = false; //Not new user
                            payload_data = data; //Set data
                            payload_changes = changes; //Set changes
                        }
                        Err(e) => println!("Error comparing data: {}", e), //Prints if comparing data fails
                    }
                },
                Err(e) => {
                    println!("Error: {}", e); //Prints if fetching data from ScoreSaber API results in failure
                    return false; //Return false to show that function didn't operate properly
                }
            }
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => { //If player data is not in database
            // If no player data exists in the database, fetch it from the API and insert it
            match datatweaks::fetch_player_data(player_id).await { //Fetch data from ScoreSaber API
                Ok(data) => { //If functioned correctly
                    // Insert new data into the database
                    if let Err(e) = datatweaks::insert_player_data(&conn, &data, &player_id) { //Call function to insert data into databas
                        println!("Error inserting data into database: {}", e); //Prints if inputing data into database results in failure
                    } else { //If inserting the player data into the database works
                        payload_data = data; //Set data
                    }
                }
                Err(e) => {
                    println!("Error fetching player data from API: {}", e); //Prints if fetching data from ScoreSaber API results in failure
                    return false; //Return false to show that function didn't operate properly
                }
            }
        }
        Err(e) => println!("Error fetching player data from database: {}", e), //Prints if fetching data from database results in failure
    }
    let payload = format::formatdata(&payload_data, &payload_changes, payload_new_user); //Make payload using function to format data
    if let Err(why) = msg.channel_id.send_message(&ctx.http, payload).await { //If sending message has error
        println!("Error sending message: {:?}", why); //Print error
    }
    true
}

//Function to start the client
async fn start_client() {
    // Check if the token is properly retrieved
    let token = match env::var("DISCORD_TOKEN") { //Retrieve token from environment
        Ok(token) => token, // If works then set token
        Err(_) => { //If failed
            println!("Error: DISCORD_TOKEN environment variable not set."); // Print error
            return;
        }
    };

    // Enable necessary intents
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT; // Specifies events the bot will respond to

    let mut client = match Client::builder(&token, intents) //Set client and specify token/intents
        .event_handler(Handler) //Impl EventHandler
        .await 
    {
        Ok(client) => client, // If works then set client
        Err(e) => { //If failed
            println!("Error creating client: {:?}", e); // If error then give error
            return;
        }
    };

    if let Err(e) = client.start().await { //Run/start client
        println!("Client error: {:?}", e); // If failed then give error
        return;
    }
}

//Main function
#[tokio::main]
async fn main() {
    start_client().await; //Call function to start the client
}