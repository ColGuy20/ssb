use crate::PlayerData; //PlayerData struct
use crate::Changes; //Changes struct
use crate::imports::*; //Imports

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

//Function to format data
pub fn formatdata(data: &PlayerData, changes: &Changes, new_user: bool) -> CreateMessage{ // Name of function and stating return type
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
    else{ //If new use
        println!("\nNew User `{}` created! <ID:{}>\n", data.name, data.id); //Print out basic user data and lets know user is new
    }

    //If-statement to make my own embed message look dark red
    let mut my_color = 0; //Set default color to black
    if data.name == "ColGuy20"{ //If it is me
        my_color = 5505024; //Change color to dark red
    }
    
    //Making the payload (Embed message into CreateEmbed)
    let payload = CreateMessage::new().embed(CreateEmbed::new() //Create message
        .color(my_color) //Set color
        .field("Description", format!("Rank: **#{}**\nCountry Rank ({}): **#{}**\nFirst Seen: {}", rank_formatted, data.country, countryRank_formatted, firstSeen_formatted), false) //Rank, Country/Rank, and date first seen
        .field("", "", false) //These are just for cosmetic purposes, they ensure that only two fields are in each row
        .field("Total Score", format!("{}", totalScore_formatted), true) //Total Score
        .field("Total Ranked Score", format!("{}", totalRankedScore_formatted), true) //Total Ranked Score
        .field("", "", false) //These are just for cosmetic purposes, they ensure that only two fields are in each row
        .field("Average Ranked Accuracy", format!("%{}", averageRankedAccuracy_formatted), true) //Average Ranked Accuracy
        .field("Performance Point (PP)", format!("{}", pp_formatted), true) //Performance Points
        .field("", "", false) //These are just for cosmetic purposes, they ensure that only two fields are in each row
        .field("Ranked Play Count", format!("{} ", rankedPlayCount_formatted), true) //Ranked Play Count
        .field("Total Play Count", format!("{} ", totalPlayCount_formatted), true) //Total Play Count
        .author(
            CreateEmbedAuthor::new("Temp") //Add embedded author
                .name(format!("{} #{}", data.name, data.rank)) //Username and global rank
                .icon_url(format!("{}",data.profilePicture)) //Profile Picture
        ));
    payload //Return embedded payload
}