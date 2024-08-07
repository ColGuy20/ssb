# General Information
The purpose of this rust program is to provide player statistics from the [Score Saber API](https://scoresaber.com/api).
- Last updated: 2024-07-03
- Last commit version: **_(V2.2.1)_**
- Project Status: **_Hiatus_**
# Set-up
Before starting, certain dependencies and imports are required for the program to function appropriately.
## Dependencies
The dependencies in the cargo.toml are:
```
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.28.0", features = ["bundled"] }
serenity = "0.12"
```
## Imports
The following imports are needed to operate the code correctly
### Imports (Excluding Serenity)
The program uses multiple imports (located in `imports.rs`):
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
- Serde Json, or `serde_json::json`, is used to create JSon files. This is used when taking in the data from the ScoreSaber API and serializing the data when comparing.
- Reqwest Error, or `reqwest::Error`, is helpful when handling errors from HTTP requests. This is used to provide feedback when an operation, such as fetching the data or sending to discord, fails.
- Tokio Time, or `tokio::time::{sleep, Duration} `, is used for asynchronous and time-based operations. This is used for the async functions and the sleep() function used to wait 10 minutes.
- Standard Environment, or `std::env`, is used to interact with environmental variables. This is used to take in the DISCORD_TOKEN variable that is hidden for security concerns.
- Rust Sqlite, or `rusqlite::{params, Connection, Result}`, is used to integrate sqlite (extension for databases) into rust. This is used to create, add data, and fetch data from the player_data database.
- Standard HashMap, or `std::collections::HashMap`, is used to make code more concise by enabling the ability to use the current scope without having to type the full path. This is used when the new-old data is being compared for diferences when serializing the data.
### Imports (Serenity)
The program also uses multiple imports that are part of serenity used to operate the discord bot (they are also located in `imports.rs`):
```
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage};
use serenity::async_trait;
use serenity::prelude::*;
```
- Message Channel Model, or `model::channel::Message`, is used to create a struct that can represent a discord message. This was used when sending/receiving messages to and from discord.
- Gateway Models, or `model::gateway::{GatewayIntents, Ready}`, are used to represent events. In this case, `GatewayIntents` shows what events the bot is seeking to recieve and `Ready` represents the events in which the bot is connected and ready.
- Builders, or `builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage}`, are used to create embeds and messages. In this case, they were used to embed messages when formatting before sending them to discord.
- Async Trait, or `async_trait`, allows the use of async functions in implementations. This was used for the EventHandler implementation because it uses `.await`.
- Prelude, or `prelude::*`, is used for commonly used types and traits used with discord bots. In this case, it was used for the EventHandler implementation, creating the client, context when sending/receiving messages, and etc.
## Modules
After finishing the base code, modules were implemented to increase organization and readability of the code.

They are (excluding `main.rs`):
- `imports.rs` contains all the imports used throughout the code.
- `datatweaks.rs` contains all the functions used to fetch and store/insert data.
- `compare.rs` contains the function used to compare new and old data.
- `format.rs` contains all the functions used to format/embed data.
= `message.rs` contains all functions used to react to and send messages to and from discord.

Main:
- `main.rs` is where all the code is actually run, where the structs are made, and everything is connected.
## Requirements
> [!WARNING]
> Key information users need to run the program. This part of the set-up needed its own section due to them varying depending on the user, system, and use. Make sure you follow these steps to ensure the program functions accordingly.
### Running the program
**When your program is ready to run type in: `.\start.cmd` into the terminal. Make sure that you are inside the `\ssb` directory.** This calls the start.cmd which is an essential part of this program because it will **NOT** function without it.
The start command should look like this:
```
cargo build -r
set LIB=PATH_TO_SQLITE3_LIB\sqlite3;%LIB%
set DISCORD_TOKEN=YOUR_TOKEN
start PATH_TO_SSB\ssb.exe
```
The first line `cargo build -r` simply builds the cargo. The next line will be skipped for now and will be referenced in the Database category next up. As for the last two, the first one sets the `DISCORD_TOKEN` to your discord bot token, so you will have to replace `YOUR_TOKEN`. As for the last one, this is the line that actually runs the program. Make sure to replace `PATH_TO_SSB` with your own path to ssb. If you are sharing this, then it is recommended to add the start.cmd to your `.gitignore` (An example would be `start*`). 
### Setting up the Database (SQLite)
To set up the data base for your program follow along these steps:
1. Go to the official [SQLite Download Page](https://www.sqlite.org/download.html). Under "Precompiled Binaries for Windows" download **"sqlite-dll-win-x64-3460000.zip"** (64-bit) or the appropriate version.
2. Extract the ZIP file, this should give you the **sqlite3.dll** and the **sqlite3.def** files.
3. Move the files into a know location. A good place would be a new folder called **sqlite3** found in `C:\sqlite3`.
4. To generate the **.lib** file, launch **Developer Command Prompt for Visual Studio** and navigate to the directory where your files are located. Then run the following command: `lib /def:sqlite3.def /out:sqlite3.lib /machine:x64`. If done properly, this will generate the **sqlite3.lib** file.
5. Back in your `start.cmd` file, make sure to add the line: `set LIB=PATH_TO_SQLITE3_LIB\sqlite3;%LIB%`. This adds the directory to your library path.
6. Make sure you added the dependency to your `cargo.toml` and you imported the rusqlite correctly into the program. This is already done but it is always better to double check.
This just adds the directory to the library path.
# Structs and Implementations
The following are structs and implementations used to run the code.
## Structs
Structs are a way to layout and group together related pieces of data. They provide a way of grouping distinct data types into one organized data stucture. 
### PlayerData (and ScoreStats) Struct
In this case, the structs were used to store the data taken in from the Score Saber API. There are for this section of the program. The second is the main struct and used to store **all** the data. The first is a struct within a struct used to organize the data inside of ScoreStats to make the fetching more efficient.
```
pub struct ScoreStats {
    totalScore: i64,
    totalRankedScore: i64,
    averageRankedAccuracy: f64,
    totalPlayCount: i64,
    rankedPlayCount: i64,
    replaysWatched: i64,
}
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
```
The `#[derive(Debug, Serialize, Deserialize, Default)]` attribute is used to provide the multiple properties to the struct. The most important are *serialize/deserialize* because they allow the alteration of the data into/from a JSon file and *default* which is helpful when creating empty instances.

Next, the first line of a struct, in this case `pub struct PlayerData`, is used to name the struct and set it public.

Lastly, in this specific struct the fields are named like in `id: String`. The first part is to choose the name of the field and the second is to choose the data type.
### PlayersData Struct (Not to be confused with PlayerData)
While having a similar name to PlayerData, this struct is very short and only contains one field. It is used to store multiple users, primarily for the seach/id command.
```
#[derive(Debug, Serialize, Deserialize, Default)] //This is used for storing multiple players (for search function)
pub struct PlayersData {
    players: Vec<PlayerData>,
}
```
### Changes Struct
This struct was used to store the differences between the old data (from the database) and the new data (from the ScoreSaber API).
```
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
```
### Handler Struct
The following are the contents of the struct: `pub struct Handler;`. The struct is quite short by itself, but it is used to implement `EventHandler` which responds to events related to the bot.
## Implementations
Implementations are used to define methods and functions for a struct, allowing you to add behavior and functionality to the data grouped in the struct.
### EventHandler Struct
This is an **implementation** for the `Handler` struct. It serves to respond to events that occur either with the bot or related to the bot in discord. Examples of this would be the bot `ready()` event (triggers when bot is ready to operate) or the `message()` event (triggers when a message is sent).
## Static Variables
This program uses static variables that are useful because they can be accessed throughout the whole module.
The module `message.rs` has the following static variables:
```
pub static mut TRACK: bool = true; //Used to stop tracking
pub static mut COUNT: i64 = 0; //Goes up every time stats sent successfully
pub static mut TRACKING: bool = false; //Shows if bot is tracking
pub static mut LINKED: bool = false; //Shows if discord is linked to a ScoreSaber account
```
- Track is used to figure out when to exit the tracking loop.
- Count is used to show the amount of times the stats have successfully been sent by incrementing.
- Tracking is used to show the status of tracking.
- Linked is used to show if the account is linked.
# Functionality
The following will be brief descriptions on how the functions in the code operate.
## Datatweaks Module
This module in used to fetch, store, and tweak data.
### Fetch Player Data
This async function retrieves detailed data for a player identified by `player_id` from the Score Saber API ([ScoreSaber Public API](https://docs.scoresaber.com)). It fetches JSON data from the specified URL and structures it into a `PlayerData` object.
### Search Player
This async function searches for players based on a provided player name or search term using the Score Saber API ([ScoreSaber Public API](https://docs.scoresaber.com)). It collects data for potential matches and stores them in a `PlayersData` struct.
### Create Database
This function initializes a SQLite database with a table named `player_data` if it doesn't already exist. The table schema includes fields to store various player attributes and statistics.
### Insert Player Data
This function manages the storage of fetched player data into the SQLite database. It first ensures the existence of a `player_data` table within the provided database connection. Then, it either inserts new data or updates existing records if data for a player with the same ID already exists.
### Fetch Player Data from DB
This function retrieves player data stored in the SQLite database based on the provided `player_id`. It prepares a query to fetch all fields associated with the player from the `player_data` table. If a matching record is found, it constructs a `PlayerData` object; otherwise, it returns an error indicating no such player exists.
### Link Discord
This async function associates a Discord ID with a player identified by `player_id` in the SQLite database. It first checks if the player exists in the database. If found, it updates the `discord` field in the `player_data` table with the provided Discord ID.
### Discord Linked
This async function checks if a Discord ID is currently linked to any player in the SQLite database. If a match is found, it returns the associated player ID; otherwise, it returns `None`.
### Delete discord if exists
This async function removes a Discord ID link from player data in the SQLite database, if it exists. It first checks if the Discord ID is linked to any player. If a link is found, it updates the `discord` field to `NULL` in the `player_data` table; otherwise, it indicates no action was taken.
## Compare Module
This module is used to send back any differences between the old and new data/statistics.
### Compare Data
This function compares new player data with existing data in the database and identifies any changes.

1. **Serialize Data**: Both the new data (`data`) and the existing data from the database (`data_from_db`) are serialized into `HashMap` objects for comparison.
2. **Initialize Changes**: A `Changes` struct instance is created to store any detected changes.
3. **Check Differences**: The function loops through the serialized data to find differences:
   - Excluded fields: Certain fields (`id`, `name`, `country`, `firstSeen`, `banned`, `inactive`, `profilePicture`, `histories`) are ignored in the comparison.
   - **Score Stats**: If differences are found in the `scoreStats` field, it further checks each statistic (`totalScore`, `totalRankedScore`, `totalPlayCount`, `rankedPlayCount`, `replaysWatched`, `averageRankedAccuracy`) and records changes.
   - **Other Fields**: It also checks other fields (`pp`, `rank`, `countryRank`) for differences.
4. **Record Changes**: If a difference is found, it updates the `Changes` struct with the specific change and its magnitude.
5. **Return Changes**: The function returns the `Changes` struct containing all the detected changes.
## Format Module
This module is used to format the player statistics.
### Format Data
This function formats all the data into an embed message (`CreateMessage`). It starts by formatting all the fields that will be included in the message, then applies any changes made. If the player name is "ColGuy20", it sets the color to dark red. Finally, it creates an embed with all the formatted content and returns it (`payload`).
### Add Comma
This function adds commas to a number. It takes in `(i64, bool)` and returns a `String`. It initializes the count (`count`) and the answer (`result`). If the number is negative (`num < 0`), it is multiplied by -1 to proceed correctly. If the number is zero, the answer is set to zero to avoid errors. As long as there are digits left, it loops (`while num > 0`). If the count is divisible by three, a comma is added. Each digit is added to `result`, the number is divided by 10 to check the next digit, and the count is incremented. After the loop, a "-" is added if the number is negative. If the number is positive and the bool parameter is true, a "+" is added. The function returns the result (`result`).
## Message Module
This module serves to respond to and send messages from and to discord.
### Send Simple Format
This async function sends a simple embedded message to a Discord channel. It constructs a basic embed with a specified title and description, then sends it to the channel specified by `msg`.
### React to Message
This async function processes incoming messages from Discord. It parses commands and parameters from the message content, validates them, and triggers corresponding actions:
- Checks if a Discord ID is linked to a ScoreSaber account.
- Handles commands like `!stats`, `!track`, `!untrack`, `!link`, `!unlink`, `!id`, and `!help`.
- Sends appropriate error messages if commands are not formatted correctly or if operations fail.
- Manages tracking intervals and sends stat messages periodically based on user input.
- Utilizes unsafe mutable static variables (`TRACK`, `COUNT`, `TRACKING`, `LINKED`) for state management across function calls.
## Main Module
This module contains the structs, implementations, and the functions to actually run the program, client, etc. 
### Send Stats
This async function begins by setting up the connection to the data base (`player_data.db`), intiating any variables used, and creating the `player_data.db` if it does not exist. It then calls and checks the `fetch_player_data`, `fetch_player_data_from_db`, `compare_data`, and `insert_player_data` functions. After fetching and storing the data, it formats the data (by using `format_data` to create the embedded message) and send it to discord. Throughout, it responds accordingly depending on if they functioned or if they failed.'
### Start Client
This function creates and starts the client for the discord bot using the token. It first retrieves the token from the environment. Then, it sets the intents. Next, it actually makes the client using the token and intents. Lastly, it starts the client (`client.start().await`).
### Main Function
This is an async function given the main property by tokio (#[tokio::main]). It only serves to run the client (`start_client().await`).
# Additional Information
This section includes helpful information such as commands you can use to test varaibles in the database.
## SQLite Commands
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
**V2.2** (Added New Commands)
- <2.2.1> Minor bug fix (When inserting data of new player into db, lack of discord led to error).
- <2.2> Commands added include: `!link`, `!unlink`, `!id`, and `!help` (functions for commands also added to `message.rs` and `datatweaks.rs`).

**V2.1** (Webhook converted to Bot)
- <2.1> Changed `main.rs`, added to `import.rs`, changed `format.rs` (previously `send.rs`), and added `message.rs`.

***--- V2 RELEASE ---***

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
# Credits
- Website used for ScoreSaber API generation and guide: https://docs.scoresaber.com
- Website used for formatting JSon to send to the discord webhook: https://birdie0.github.io/discord-webhooks-guide/discord_webhook.html
- Website used for converting RGB and HEX into decimal: https://www.mathsisfun.com/hexadecimal-decimal-colors.html
- Serenity official website: https://docs.rs/serenity/latest/serenity/
- ScoreSaber official website: https://scoresaber.com
- ScoreSaber discord: https://discord.com/invite/scoresaber
- Official GitHub Guide for github syntax: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax
- Big Thanks to @coinchimp for all the support: https://github.com/coinchimp/