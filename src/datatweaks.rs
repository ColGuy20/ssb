use crate::PlayerData; //PlayerData struct
use crate::ScoreStats; //ScoreStats struct
use crate::imports::*; //Imports

//Function used to fetch/take in the data from Scoresaber
pub async fn fetch_player_data(player_id: &str) -> Result<PlayerData, Error> { //Name of function and stating return type
    let url = "https://scoresaber.com/api/player/".to_owned()+player_id+"/full"; //The url used to take in date (In this case it is my username)
    let response = reqwest::get(url).await?.json::<PlayerData>().await?; //Line to actually take the data in
    Ok(response) //Returns response
}

// Function to insert data into database
pub fn insert_player_data(conn: &Connection, data: &PlayerData, player_id: &str) -> Result<()> {
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
        "INSERT OR REPLACE INTO player_data(
            id, name, profilePicture, country, pp, rank, countryRank, histories, banned, inactive, 
            totalScore, totalRankedScore, averageRankedAccuracy, totalPlayCount, rankedPlayCount, 
            replaysWatched, firstSeen
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
        params![
            player_id,
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