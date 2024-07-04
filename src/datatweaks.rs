use crate::PlayerData; //PlayerData struct
use crate::ScoreStats; //ScoreStats struct
use crate::PlayersData; //PlayersData struct
use crate::imports::*; //Imports

//Function used to fetch/take in the data from Scoresaber
pub async fn fetch_player_data(player_id: &str) -> Result<PlayerData, Error> { //Name of function and stating return type
    let url = format!("https://scoresaber.com/api/player/{}/full", player_id); //The url used to take in data
    let response = reqwest::get(url).await?.json::<PlayerData>().await?; //Line to actually take the data in
    Ok(response) //Returns response
}

// Function to search for the ID of a player based on their name
pub async fn search_player(player_name: &str) -> Result<PlayersData, Error> {
    let url = format!("https://scoresaber.com/api/players?search={}&withMetadata=false", player_name); // Construct the URL with the player name
    let response = reqwest::get(url).await?.json::<PlayersData>().await?; // Send the request and get the response as a text
    Ok(response)
}

//Function to create database
pub fn create_db(conn: &Connection) -> Result<()> {
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
            firstSeen TEXT,
            discord TEXT
        )",
        params![],
    )?;
    Ok(())
}

// Function to insert data into database
pub fn insert_player_data(conn: &Connection, data: &PlayerData, new_user: bool) -> Result<()> {

    let mut stmt = conn.prepare("SELECT discord FROM player_data WHERE id = ?1")?;
    let mut discord_id = String::from("NULL");
    if !new_user { //If not new user
        discord_id = stmt.query_row(params![data.id], |row| row.get(0))?;
    }

    conn.execute(
        "INSERT OR REPLACE INTO player_data(
            id, name, profilePicture, country, pp, rank, countryRank, histories, banned, inactive, 
            totalScore, totalRankedScore, averageRankedAccuracy, totalPlayCount, rankedPlayCount, 
            replaysWatched, firstSeen, discord
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
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
            data.firstSeen,
            discord_id
        ],
    )?;
    Ok(())
}

//Fetch player data from database
pub fn fetch_player_data_from_db(conn: &Connection, player_id: &str) -> Result<PlayerData, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT *, name FROM player_data WHERE id = ?1")?; // Prepare the query
    let mut rows = stmt.query(params![player_id])?; // Execute the query

    if let Some(row) = rows.next()? { // If there is a row that matches the parameters
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
    } else { // If there is not a row that matches the paramaters
        Err(rusqlite::Error::QueryReturnedNoRows) // Send back an error letting know that no row was found
    }
}

// Function to add discord to database
pub async fn link_discord(discord_id: &str, player_id: &str) -> Result<bool> {
    let conn = Connection::open("player_data.db").expect("Failed to open database"); //Set up connection for database
    
    // Check if the row exists
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM player_data WHERE id = ?1")?;
    let id_value: i32 = stmt.query_row(params![player_id], |row| row.get(0))?;
    
    if id_value == 0 { //If player ID not found
        return Ok(false); // Return false to indicate failure
    }

    // Execute the SQL command to update the existing row
    conn.execute(
        "UPDATE player_data SET discord = ?1 WHERE id = ?2",
        params![discord_id, player_id],
    )?;

    Ok(true) // Return true to indicate success
}

// Function to check if Discord ID exists in the database
pub async fn discord_linked(discord_id: &str) -> Result<Option<String>> {
    let conn = Connection::open("player_data.db").expect("Failed to open database"); // Set up connection for database

    // Check if the row exists
    let mut stmt = conn.prepare("SELECT id FROM player_data WHERE discord = ?1")?;
    let player_id_result: Result<String, _> = stmt.query_row(params![discord_id], |row| row.get(0));

    match player_id_result {
        Ok(id) => Ok(Some(id.to_owned())), // If exists return player id
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None), // If discord not found return None
        Err(e) => {
            print!("\n\nQuery failed (!link): {}", e); // Print error
            Ok(None) // If query failed return None
        }
    }
}

//Function to delete old discord link
pub async fn delete_discord_if_exists(discord_id: &str) -> Result<bool> {
    let conn = Connection::open("player_data.db").expect("Failed to open database"); //Set up connection for database
    if let Some(player_id) = discord_linked(discord_id).await? { //If discord exists
        // Execute the SQL command to remove the discord from the row
        conn.execute(
            "UPDATE player_data SET discord = NULL WHERE id = ?1",
            params![player_id],
        )?;
    } else { //If discord not found
        return Ok(false); //Return false to indicate failure
    }
    return Ok(true); // Return true to indicate success
}