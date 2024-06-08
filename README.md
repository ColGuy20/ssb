# Main
The purpose of this rust program is to provide player statistics from the [Score Saber API](https://scoresaber.com/api).
- Last updated: 2024-06-08
- Last commit version: **_(V2)_**
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
serenity = "0.12"
```
### Imports 1 (Excluding Serenity)
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
### Imports 2 (Serenity)
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
### Modules
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
### Message Static
The module `message.rs` has the following static variables:
```
pub static mut TRACK: bool = true; //Used to stop tracking
pub static mut COUNT: i64 = 0; //Goes up every time stats sent successfully
pub static mut TRACKING: bool = false; //Shows if bot is tracking
```
- Track is used by `/track` and `/untrack` to figure out when to exit the tracking loop
- Count is used by `/stats` and `/track` to increment and show the amount of times the stats have successfully been sent
- Tracking is used by `/track` and `/untrack` to show the status of tracking. If the bot is tracking then untrack will send an error to the discord.
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
### Insert Player Data
This function is made up of two parts but ultimately serves the purpose of storing the fetched data into a database. The first part creates a table if none exist yet. This table is inside the connection and named, you guessed it, `player_data`. The second part either inserts or replaces the data inside the databased with the new, updated data fetched from the API. It finishes off by returning `OK(())` to show that everything went well.
### Comparing Data
This function's purpose is to compare the new-old data and store the changes (or lack of changes) for later use. It begins by serializing the data, data from the database, and both the scoreStats structures inside each so they can be easily compared. Then, it makes an instance variable of the `Changes` struct to store any changes made. After it begins a loop for each field in the data, if the data is the same, excluded, or not comparable then it loop again. If none of these conditions are true, then it sets the first field as true to show that there is a change and the second field as the value of the change. Also, if the difference is inside of scoreStats, then it enters another loop and checks all the data in score stats to single out and store the difference. After all the changes are stored in the instance variable, it is then returned as `Ok(changes)` signaling that everything went right and sending back the now-filled struct.
### Send Stats
This async function begins by setting up the connection to the data base (`player_data.db`) and creating three default variables used when formatting. It then calls and checks the `fetch_player_data`, `fetch_player_data_from_db`, `compare_data`, and `insert_player_data` functions. After fetching and storing the data, it formats the data (by using `format_data` to create the embedded message) and send it to discord. Throughout, it responds accordingly depending on if they functioned or if they failed.'
### Format Data
This function serves to format all the data into an embed. It takes in the data/changes and returns the embedded message (`CreateMessage`). It starts by formatting all the fields that will be included in the message. Then, it applies any changes changes made. This was purely optional, but if the player name is "ColGuy20" then it will set the color to dark red. Lastly, it creates an embed with all the formatted content and returns it (`payload`).
### React to Message
This async function responds to the command received. It takes in the context and message but returns nothing. It starts by initiating the `cooldown` variable. Then, it takes the message, splits it into words, and gets the word count. The first two are stored as `command` and `player_id`. If the command does not require over one word (such as `/stats` and `/track`) then it sends a message if the word count is exceeded. Then, it compares the word count to those that do require over one word and sends a message if there is an incorrect number of fields. Also, the `/track` command checks if the `time` value provided is a valid, positive number or it sends a message. If the player_id is found to be empty then it sets the value of `message_id` as false for all the commands that require it. Next, it matches the command and if all the conditions are correct (otherwise it sends and error), then it performs the function required by the command (such as `send_stats` or `send_simple_format`). Unless, the command does not exist which leads to the bot sending an error instead of running a function.
### Start Client
This function creates and starts the client for the discord bot using the token. It first retrieves the token from the environment. Then, it sets the intents. Next, it actually makes the client using the token and intents. Lastly, it starts the client (`client.start().await`).
### Main Function
This is an async function given the main property by tokio (#[tokio::main]). It only serves to run the client (`start_client().await`).
## Additional Functions 
This category are function that aren't necessarily needed (cosmetic, formatting, extra, etc.).
#### Add Comma
This function serves one purpose, to add commas to the number taken in. It takes in `(i64,bool)` and returns a String. It begins by making an integer for the count (`count`) and the answer (`result`). Following, it checks if the number is negative (`num < 0`) and multiplies by -1 if it is in order to proceed with the code correctly. Also, if the number is zero then it proceeds to change the answer to zero in order to avoid any errors. Then, as long as there is more to the number, it begins to loop (`while num > 0`). If the count is divisible by three, then a comma is added. Next, no matter the digit, it is added to the `result`. After being the digit is added to the answer, the next number is divided by 10 to check the next digit and the count is incremented. When the loop ends, if the number is negative then a "-" symbol is added to the beginning. If the number is not negative and the bool entered in through the parameter (which gives the option to include a positive symbol) is true, then a "+" symbol is added to the beginning. The function finishes by returning the answer (`result`).
### Send Simple Format
This async function serves to send a simple embedded message. It takes in the context, message sent from the player (for the message id), and message the bot is seeking to send. It then creates the embed and actually sends it (If it does not work then an error is printed).
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
## Credits
- Website used for ScoreSaber API generation and guide: https://docs.scoresaber.com
- Website used for formatting JSon to send to the discord webhook: https://birdie0.github.io/discord-webhooks-guide/discord_webhook.html
- Website used for converting RGB and HEX into decimal: https://www.mathsisfun.com/hexadecimal-decimal-colors.html
- Serenity official website: https://docs.rs/serenity/latest/serenity/
- ScoreSaber official website: https://scoresaber.com
- ScoreSaber discord: https://discord.com/invite/scoresaber
- Official GitHub Guide for github syntax: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax
- Big Thanks to @coinchimp for all the support: https://github.com/coinchimp/