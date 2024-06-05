# Main
The purpose of this rust program is to take in data every 10 minutes from the [Score Saber API](https://scoresaber.com/api).
- Last updated: 2024-06-05
- Last commit version: **_(V1.5)_**
- Project Status: **_Active_**
## Set-up
Before starting, certain dependencies and imports are required for the program to function appropriately.
### Dependencies
The dependencies in the cargo.toml are:
```
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.28.0", features = ["bundled"] }
```
### Imports
The program uses 5 imports (located in `imports.rs`):
```
use serde::{Deserialize, Serialize}; 
use serde_json::json;
use reqwest::Error;
use tokio::time::{sleep, Duration}; 
use std::env;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
```
- Serde, more specifically `serde::{Deserialize, Serialize}`, is used to serialize data, or convert for easy storage and managment. This is used when creating the structs that store the data fetched from Score Saber.
- Serde Json, or `serde_json::json`, is used to create JSon files. This is used to create the JSon file that is formatted for discord and sent to the webhook.
- Reqwest Error, or `reqwest::Error`, is helpful when handling errors from HTTP requests. This is used to provide feedback when an operation, such as fetching the data or sending to discord, fails.
- Tokio Time, or `tokio::time::{sleep, Duration} `, is used for asynchronous and time-based operations. This is used for the async functions and the sleep() function used to wait 10 minutes.
- Standard Environment, or `std::env`, is used to interact with environmental variables. This is used to take in the WEBHOOK_URL variable that is hidden for security concerns.
- Rust Sqlite, or `rusqlite::{params, Connection, Result}`, is used to integrate sqlite (extension for databases) into rust. This is used to create, add data, and fetch data from the player_data database.
- Standard HashMap, or `std::collections::HashMap`, is used to make code more concise by enabling the ability to use the current scope without having to type the full path. This is used when the new-old data is being compared for diferences when serializing the data.
### Modules
After finishing the base code, modules were implemented to increase organization and readability of the code.
They are:
- `imports.rs` contains all the imports used throughout the code.
- `datatweaks.rs` contains all the functions used to fetch and store/insert data.
- `compare.rs` contains the function used to compare new and old data.
- `send.rs` contains all the functions used to format and send data to the discord webhook.
Main:
- `main.rs` is where all the code is actually run, where the structs are made, and everything is connected.
## Requirements
> [!WARNING]
> Key information users need to run the program. This part of the set-up needed its own section due to them varying depending on the user, system, and use. Make sure you follow these steps to ensure the program functions accordingly.
### Running the program
**When your program is ready to run type in: `.\start.cmd` into the terminal. Make sure that you are inside the `\sswh` directory.** This calls the start.cmd which is an essential part of this program because it will **NOT** function without it.
The start command should look like this:
```
cargo build -r
set LIB=PATH_TO_SQLITE3_LIB\sqlite3;%LIB%
set WEBHOOK_URL=YOUR_URL
start PATH_TO_SSWH\sswh.exe
```
The first line `cargo build -r` simply builds the cargo. The next line will be skipped for now and will be referenced in the Database category next up. As for the last two, the first one sets the `WEBHOOK_URL` to your discord webhook's url, so you will have to replace `YOUR_URL`. As for the last one, this is the line that actually runs the program. Make sure to replace `PATH_TO_SSWH` with your own path to sswh. If you are sharing this, then it is recommended to add the start.cmd to your `.gitignore` (An example would be `start*`). 
### Setting up the Database (SQLite)
To set up the data base for your program follow along these steps:
1. Go to the official [SQLite Download Page](https://www.sqlite.org/download.html). Under "Precompiled Binaries for Windows" download **"sqlite-dll-win-x64-3460000.zip"** (64-bit) or the appropriate version.
2. Extract the ZIP file, this should give you the **sqlite3.dll** and the **sqlite3.def** files.
3. Move the files into a know location. A good place would be a new folder called **sqlite3** found in `C:\sqlite3`.
4. To generate the **.lib** file, launch **Developer Command Prompt for Visual Studio** and navigate to the directory where your files are located. Then run the following command: `lib /def:sqlite3.def /out:sqlite3.lib /machine:x64`. If done properly, this will generate the **sqlite3.lib** file.
5. Back in your `start.cmd` file, make sure to add the line: `set LIB=PATH_TO_SQLITE3_LIB\sqlite3;%LIB%`. This adds the directory to your library path.
6. Make sure you added the dependency to your `cargo.toml` and you imported the rusqlite correctly into the program. This is already done but it is always better to double check.
This just adds the directory to the library path.
## Structs
Structs are a way to layout and group together related pieces of data. They provide a way of grouping distinct data types into one organized data stucture. 
### PlayerData (ScoreStats) Struct
In this case, the structs were used to store the data taken in from the Score Saber API. There are for this section of the program. The second is the main struct and used to store **all** the data. The first is a struct within a struct used to organize the data inside of ScoreStats to make the fetching more efficient.
```
#[derive(Debug, Serialize, Deserialize)] //This stores data under ScoreStats
struct ScoreStats {
    totalScore: i64,
    totalRankedScore: i64,
    averageRankedAccuracy: f64,
    totalPlayCount: i64,
    rankedPlayCount: i64,
    replaysWatched: i64,
}
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
```
The `#[derive(Debug, Serialize, Deserialize)]` is used to provide the three properties to the struct. The most important are serialize and deserialize because they allow the alteration of the data into a JSon file, formatted for the webhook.

Next, the first line of a struct, in this case `struct PlayerData`, is used to name the struct.

Lastly, in this specific struct the fields are named like in `id: String`. The first part is to choose the name of the field and the second is to choose the data type.
### Changes Struct
This struct was used to store whether or not there is a change and what the change is (if any). If there was no change, then the first value (boolean) would be `false` and the second (i64/f64) would be `0` or `0.0` depending on the variable type. In the case of this program, change was used when seeing the differences between the old data (from the database) and the new data (from the ScoreSaber API).
```
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
```
## Functionality
The following will be brief descriptions on how the rest of the code operates.
### Fetch Player Data from ScoreSaber
This async function serves to extract data of the `player_id` from the Score Saber API and store it in the struct of PlayerData (See [ScoreSaber Public API](https://docs.scoresaber.com)). It uses the url to take in the data as a json (via `reqwest::get` and `.json`) and returns it structured in the `PlayerData` struct.
```
async fn fetch_player_data(player_id: &str) -> Result<PlayerData, Error> { //Name of function and stating return type
    let url = "https://scoresaber.com/api/player/".to_owned()+player_id+"/full"; //The url used to take in date (In this case it is my username)
    let response = reqwest::get(url).await?.json::<PlayerData>().await?; //Line to actually take the data in
    Ok(response) //Returns response
}
```
### Fetch Player Data from DB
This function serves to fetch the data inside of the database and store it in the same struct as the previous function (`fetch_player_data`). It takes in the connection to the database and `player_id` and returns the data formatted in the `PlayerData` struct. First, it begins by preparing the query (a request for data) by stating what data it is taking in, in this case it is taking in all the data in `player_data`. Then, it actually executes the query and looks for a row that has an `id` that matches the `player_id` from the beginning. If it find a row, then it places the data in an instance variable of the `PlayerData` struct and returns it as `Ok(player_data)` to signal everything worked. Otherwise, if a row matching the paramaters is not found, then it returns a specific error letting the program know that a row under the certain ID does not exists yet (`Err(rusqlite::Error::QueryReturnedNoRows)`).
### Comparing Data
This function's purpose is to compare the new-old data and store the changes (or lack of changes) for later use. It begins by serializing the data, data from the database, and both the scoreStats structures inside each so they can be easily compared. Then, it makes an instance variable of the `Changes` struct to store any changes made. After it begins a loop for each field in the data, if the data is the same, excluded, or not comparable then it loop again. If none of these conditions are true, then it sets the first field as true to show that there is a change and the second field as the value of the change. Also, if the difference is inside of scoreStats, then it enters another loop and checks all the data in score stats to single out and store the difference. After all the changes are stored in the instance variable, it is then returned as `Ok(changes)` signaling that everything went right and sending back the now-filled struct.
### Insert Player Data
This function is made up of two parts but ultimately serves the purpose of storing the fetched data into a database. The first part creates a table if none exist yet. This table is inside the connection and named, you guessed it, `player_data`. The second part either inserts or replaces the data inside the databased with the new, updated data fetched from the API. It finishes off by returning `OK(())` to show that everything went well.
### Send Stats to Discord
This async function's purpose is to send the data to the discord server (via webhook). It starts off by taking in the webhook url from the environment and using it to state the client. Following, it formatts any data needing formatting and implements any changes made between the new-old data by adding it to the formatted data. This was completely optional, but there is an if statement that changed the default black color of the embed message to dark red if the user if "ColGuy20" (me). Then, the function creates the JSon payload (`let payload = json!({`), the file being sent to the webhook containing all the data and formatting, via use of the embed discord webhook format (See [Discord Webhooks Guide](https://birdie0.github.io/discord-webhooks-guide/discord_webhook.html)). Next, it actually sends the message and checks if it was a success. Lastly, if the code functioned then it returns `OK(())`.
### Main Function
This is the main method of the program. Again, this is an async function given the main property by tokio (`#[tokio::main]`). It begins by creating three things: the `success_count`, the `player_id` (will be changed), and connection for the database (`let conn = Connection::open("player_data.db").expect("Failed to open database");`). It then enters a loop that repeats every ten minutes by using `sleep(Duration::from_secs(600)).await;` imported from tokio. In the loop, it calls and checks the fetch_player_data, fetch_player_data_from_db, compare_data, insert_player_data, and send_stats_to_discord functions. It responds accordingly depending on if they functioned, had a specific error (such as `rusqlite::Error::QueryReturnedNoRows`), or if they failed.
## Additional Functions 
This category are function that aren't necessarily needed (cosmetic, formatting, extra, etc.).
#### Add Comma
This function serves one purpose, to add commas to the number taken in. It takes in `(i64,bool)` and returns a String. It begins by making an integer for the count (`count`) and the answer (`result`). Following, it checks if the number is negative (`num < 0`) and multiplies by -1 if it is in order to proceed with the code correctly. Also, if the number is zero then it proceeds to change the answer to zero in order to avoid any errors. Then, as long as there is more to the number, it begins to loop (`while num > 0`). If the count is divisible by three, then a comma is added. Next, no matter the digit, it is added to the `result`. After being the digit is added to the answer, the next number is divided by 10 to check the next digit and the count is incremented. When the loop ends, if the number is negative then a "-" symbol is added to the beginning. If the number is not negative and the bool entered in through the parameter (which gives the option to include a positive symbol) is true, then a "+" symbol is added to the beginning. The function finishes by returning the answer (`result`).
## Additional Information
This section includes helpful information such as commands you can use to test varaibles in the database.
### SQLite Commands
This command **selects** all the data in the player_data table
```
SELECT *
FROM player_data
```

This command **deletes** the row in player_data containing a certain id
```
DELETE FROM player_data WHERE id = [PLACE_ID_HERE];
```

This command **changes** (or updates) the listed data in the player_data table
```
UPDATE player_data
SET totalScore = 123454321, totalPlayCount = 342, rank = 3232;
```
## Brief Log
***V1.5*** (Implemented Modules)
- <1.5> Added four modules to hold functions (and imports) and shortened `main.rs` (`compare.rs`, `datatweaks.rs`, `imports.rs`, and `send.rs`)

***V1.4*** (DB Multiple Users)
- <1.4> Added support for multiple users and made all fetch/insert functions depend on a single varaibe
    - Added support for new users that aren't in the data base

***V1.3*** (DB Comparison)
- <1.3> Added `comparing_data` for identifying and sending differences between new-old data
    - Added more advanced formatting and implemented changes in `send_stats_to_discord`

***V1.2*** (DB Integrated)
- <1.2.2> Added `fetch_player_data_from_db` for fetching data from DB
- <1.2.1> Added `add_comma` function for formatting
    - Made variables for ID used in ScoreSaber API URL and cooldown for sleep function
- <1.2> Added `insert_player_data` function to store fetched data in the database (`player_data.db`)

***V1.1*** (Pre-DB)
- <1.1> Added `success_count` to log amount of loops
> [!NOTE]
> Some of the commits may be incorrect, extra, outdated, or a version too far ahead.
## Credits
- Website used for ScoreSaber API generation and guide: https://docs.scoresaber.com
- Website used for formatting JSon to send to the discord webhook: https://birdie0.github.io/discord-webhooks-guide/discord_webhook.html
- Website used for converting RGB and HEX into decimal: https://www.mathsisfun.com/hexadecimal-decimal-colors.html
- ScoreSaber official website: https://scoresaber.com
- ScoreSaber discord: https://discord.com/invite/scoresaber
- Official GitHub Guide for github syntax: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax
- Big Thanks to @coinchimp for all the support: https://github.com/coinchimp/