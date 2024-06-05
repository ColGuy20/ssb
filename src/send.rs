use crate::PlayerData;
use crate::Changes;
use crate::imports::*;

//Function to add commas
pub fn add_commas(mut num: i64, include_pos: bool) -> String {
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
pub async fn send_stats_to_discord(data: &PlayerData, changes: &Changes, success_count: &mut i64, new_user: bool) -> Result<(), Error> { // Name of function and stating return type
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
    let mut averageRankedAccuracy_formatted = format!("{}", (data.scoreStats.averageRankedAccuracy * 10_000.0).round() / 10_000.0); //Make formatted var (round four decimal places)
    let firstSeen_formatted = &data.firstSeen[0..10]; //Condencing the firstSeen variable to exclude time

    //Implementing changes to payload if it is not a new user 
    if !new_user{ //If not new user
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
            averageRankedAccuracy_formatted += &format!("\n`{}`", (changes.rAccuracy_change * 10_000.0).round() / 10_000.0); //rounds 4 decimal places
        }
        //println!("\nExisting User `{}` found! <ID:{}>\n", data.name, data.id); //Print out basic user data
    }
    /*else{ //If new use
        println!("\nNew User `{}` created! <ID:{}>\n", data.name, data.id); //Print out basic user data and lets know user is new
    }*/

    //If-statement to make my own embed message look dark red
    let mut color = 0; //Set default color to black
    if data.name == "ColGuy20"{ //If it is me
        color = 5505024; //Change color to dark red
    }

    //Making the payload (Formatted JSon that program gives to webhook to send)
    let payload = json!({ //The data is put in a JSon file for the discord webhook
        "embeds": [{
            "author": {
                "name": format!("{} #{}", data.name, data.rank), //Username and global rank
                "icon_url": format!("{}",data.profilePicture) //Profile Picture
            },
            "color": color,
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