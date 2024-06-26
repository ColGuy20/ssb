use crate::imports::*; //Imports needed for program
use crate::send_stats; //Function to fetch, store, and send stats
use crate::datatweaks::link_discord;
use crate::datatweaks::delete_discord_if_exists;
use crate::datatweaks::discord_linked;
use crate::datatweaks::search_player;

//Static variables
pub static mut TRACK: bool = true; //Used to stop tracking
pub static mut COUNT: i64 = 0; //Goes up every time stats sent successfully
pub static mut TRACKING: bool = false; //Shows if bot is tracking
pub static mut LINKED: bool = false; //Shows if discord is linked to a ScoreSaber account

//Function to send a simple embedded message
pub async fn send_simple_format(ctx: &Context, msg: &Message, message: &str) {
    let embed = CreateMessage::new().embed(CreateEmbed::new() //Make Embedf
        .color(0) //Set color to black
        .title("ScoreSaber Stats") //Set title
        .description(message) //Set description to message taken in
    );
    if let Err(why) = msg.channel_id.send_message(&ctx.http, embed).await { // If sending message has error
        println!("Error sending message: {:?}", why); // Print error
    }
}

//Function to act on message recieved
pub async fn react_to_msg(ctx: Context, msg: Message){
    //Making variables
    let message = msg.content.as_str(); //Message to &str
    let mut message_parts = message.split(' '); //Break apart words in message
    let command = message_parts.next().unwrap(); //Set command to first word
    let first_param = message_parts.next().unwrap_or(""); //Save first parameter
    let message_word_count = message.split_whitespace().count(); //Count the number of words in the message
    //
    let mut cooldown:u64 = 300; //Default cooldown is 5 minutes
    let mut player_id = ""; //Initiate player_id
    let mut working = true; //If stats does not have the correct word count
    //
    let mut linking = true;
    let mut linked_id = String::new(); // Initialize linked_id as an empty string
    unsafe {
        LINKED = false;
    }

    //Check if player linked
    match discord_linked(&msg.author.name).await {
        Ok(Some(temp_linked_id)) => { //If player linked
            unsafe {
                LINKED = true; //Player is linked
            }
            linked_id = temp_linked_id; //ID that is linked to discord
        },
        Ok(None) | Err(_) => { //If player not linked
            unsafe {
                LINKED = false;
            }
        },
    }
    

    // Make sure message has right number of words
    if message_word_count > 1 && command != "!track" && command != "!stats" && command != "!link" && command != "!help" && command != "!id" { //If the message has more than one word
        if command == "!untrack"{ //If the command is untrack
            send_simple_format(&ctx, &msg, "Incorrect number of fields. (`!untrack`)").await; //Send error
        } else if command == "!unlink"{ //If the command is unlink
            send_simple_format(&ctx, &msg, "Incorrect number of fields. (`!unlink`)").await; //Send error
        } else {
            send_simple_format(&ctx, &msg, "Incorrect number of fields.").await; //Send error
        }
        working = false; //Command link is not working
    } else if command == "!stats" {
        if message_word_count != 2 { //If command is stats and there is not two words in message
            unsafe{
                if message_word_count != 1 || !LINKED {
                    send_simple_format(&ctx, &msg, "Incorrect number of fields. (`!stats` `player_id`) (If linked use: `!stats`)").await; //Send error
                    working = false; //Command stats is not working
                }
            }
        } else {
            player_id = first_param;
            linking = false;
        }
    } else if command == "!id" {
        if message_word_count != 2 { //If command is id and there is not two words in message
            send_simple_format(&ctx, &msg, "Incorrect number of fields. (`!id` `player_name`)").await; //Send error
            working = false; //Command id is not working
        } else {
            player_id = first_param;
        }
    } else if command == "!link" && message_word_count !=2 {
        send_simple_format(&ctx, &msg, "Incorrect number of fields. (`!link` `player_id`)").await; //Send error
        working = false; //Command link is not working
    } else if command == "!track"{ //If command is track
        if message_word_count != 3 { //If there is not three words in message
            unsafe {
                if message_word_count !=2 || !LINKED { //If command is not linked
                    send_simple_format(&ctx, &msg, "Incorrect number of fields. (`!track` `player_id` `time (seconds)`) (If linked use: `!track` `time`)").await; //Send error
                    working = false; //Command stats is not working
                } else {
                    match first_param.parse::<u64>() { //Try to parse cooldown from message
                        Ok(num) => { //If able to parse
                            if num > 5{ //If number is greater than 5
                                cooldown = num; //Set cooldown
                            }  else {
                                send_simple_format(&ctx, &msg, "Please use a number over 5 seconds. (`!track` `player_id` `time (seconds)`)").await; //Send error
                                working = false; //Command track is not working
                            }
                        }
                        Err(e) => { //If unable to parse
                            send_simple_format(&ctx, &msg, "Please use a number for **time**. Make sure it is positive. (`!track` `player_id` `time (seconds)`)").await; //Send error
                            println!("Error parsing cooldown: {:?}", e); //Print error
                            working = false; //Command track is not working
                        }
                    }
                }
            }
        } else {
            match message_parts.next().unwrap_or("").parse::<u64>() { //Try to parse cooldown from message
                Ok(num) => { //If able to parse
                    if num > 5{ //If number is greater than 5
                        cooldown = num; //Set cooldown
                        player_id = first_param;
                        linking = false;
                    }  else {
                        send_simple_format(&ctx, &msg, "Please use a number over 5 seconds. (`!track` `player_id` `time (seconds)`)").await; //Send error
                        working = false; //Command track is not working
                    }
                }
                Err(e) => { //If unable to parse
                    send_simple_format(&ctx, &msg, "Please use a number for **time**. Make sure it is positive. (`!track` `player_id` `time (seconds)`)").await; //Send error
                    println!("Error parsing cooldown: {:?}", e); //Print error
                    working = false; //Command track is not working
                }
            }
        }
    }

    match command { //Ract to command
        "!stats" => { //If command is stats
            if working{ //If stats is working
                unsafe {
                    if LINKED && linking{
                        player_id = linked_id.as_str();
                    }
                }
                if !send_stats(player_id, ctx.clone(), msg.clone()).await{ //Function to send stats
                    send_simple_format(&ctx, &msg, "Invalid player ID provided.").await; //Send error
                }
                unsafe{ //Using pub static variables
                    COUNT += 1; // Increment the success count
                    println!("Stats message sent successfully to Discord [Message Count: {}] (!stats)", COUNT); // Prints if successful
                }
            }
        }
        "!track" => { //If command is track
            unsafe{
                TRACK = true; //Keep looping because it is tracking
                TRACKING = true; //Bot is currently tracking
            }
            for loop_count in 0.. { //Loop
                unsafe{ //Using pub static variables
                    if TRACK == false { //If the function will only run once
                        break; //Exit the loop
                    }
                }
                if !working{break} //If /track is not working break
                unsafe {
                    if LINKED && linking{
                        player_id = linked_id.as_str();
                    }
                }
                if loop_count == 0{send_simple_format(&ctx, &msg, format!("Began tracking every {} seconds!", cooldown).as_str()).await} //If it is looping for the same time then print
                if !send_stats(player_id, ctx.clone(), msg.clone()).await{ //Function to send stats
                    send_simple_format(&ctx, &msg, "****FAILED:**** Invalid player ID provided.").await; //Send error
                    break; //Exit the loop
                }
                unsafe{ //Using pub static variables
                    COUNT += 1; // Increment the success count
                    println!("Stats message sent successfully to Discord [Message Count: {}] (!track [Every {} seconds])", COUNT, cooldown); // Prints if successful
                }
                sleep(Duration::from_secs(cooldown)).await; //Waits before looping
            }
            unsafe{ //Using pub static variables
                TRACKING = false; //Bot has stopped tracking
            }
        }
        "!untrack" => { //If command is untrack
            if working {
                unsafe{ //Using pub static variables
                    if TRACKING{ //If bot is tracking
                        TRACK = false; //Stop looping in track
                        send_simple_format(&ctx, &msg, "Tracking has stopped!").await; //Send message
                        println!("Tracking has stopped (!untrack)"); // Prints if successful
                    } else {
                        send_simple_format(&ctx, &msg, "Not currently tracking!").await; //Send error
                    }
                }
            }
        }
        "!link" => {
            if working {
                if let Ok(deleted) = delete_discord_if_exists(&msg.author.name).await { // Call Function to delete old discord
                    if deleted { //If deleted
                        println!("\nDiscord `{}` has been removed from its previous location (!link)", msg.author.name); //Print success
                    }
                    player_id = first_param;
                    if let Ok(linked) = link_discord(&msg.author.name, &player_id).await { //Function to create database if not exists
                        if linked {
                            println!("\nAccount: `{}` has been linked with the discord: `{}` (!link)\n", player_id, msg.author.name); //Print success
                            send_simple_format(&ctx, &msg, format!("Account: `{}` has been linked with `{}` (discord)", player_id, msg.author.name).as_str()).await; //If worked
                        } else { //If not linked
                            send_simple_format(&ctx, &msg, "Invalid ID provided! (`!link` `player_id`)").await; //Send error
                        }
                    } else {
                        send_simple_format(&ctx, &msg, format!("Please generate an account in the database for `{}` by running a stats command! (`!stats` `player_id`)", player_id).as_str()).await; //If invalid
                    }
                } else { //If failed to remove previous discord
                    send_simple_format(&ctx, &msg, "Failed to remove previous Discord link. Please try again.").await; //Send error
                }
            }
        }
        "!unlink" => {
            if working {
                if let Ok(_) = delete_discord_if_exists(&msg.author.name).await { //If deleted discord link
                    send_simple_format(&ctx, &msg, format!("\nDiscord `{}` has been removed if it existed (!link)", msg.author.name).as_str()).await; //Send success
                } else { //If failed to remove discord link
                    send_simple_format(&ctx, &msg, "Failed to remove previous Discord link. Please try again.").await; //Send error
                }
            }
        }
        "!id" => {
            if working {
                //In this case please not player_id is actually the search term (player name)
                match search_player(&player_id.to_lowercase()).await { //Checks if fetching data from ScoreSabere API goes successfully
                    Ok(data) => { //If searched for player correctly
                        let mut exit_loop = false; //Used to exit loop
                        let mut player_counter = 0; //Used to increment through players matching search term
                        while !exit_loop { //Loop as long as exit_loop is false
                            if let Some(player) = data.players.get(player_counter) { //If player exists
                                if player.name.to_lowercase() == player_id.to_lowercase() { //If player name found matches player name requested
                                    let found_player_id = &player.id; //Find id of player
                                    send_simple_format(&ctx, &msg, format!("The player id of {} is `{}`", player_id, found_player_id).as_str()).await; //Send player_id
                                    println!("The player id of {} is `{}` (!id)", player_id, found_player_id); //Print result
                                    exit_loop = true; //Exit loop
                                }
                            } else { //If player is not found
                                send_simple_format(&ctx, &msg, format!("No players matched the name `{}`!\nDid you spell it right? (Caps don't affect search)", player_id).as_str()).await; //Send error
                                println!("No players matched the name `{}`! (!id)", player_id); //Print error
                                exit_loop = true; //Exit loop
                            }
                            player_counter += 1; //Increment
                        }
                    }
                    Err(e) => { //If failed to search player
                        send_simple_format(&ctx, &msg, format!("Error searching for and fetching player data from API: {}", e).as_str()).await; //Send error
                        println!("Error fetching data from API: {}", e); //Print error
                    }
                }
            }
        }
        "!help" => {
            if working {
                let embed = CreateMessage::new().embed(CreateEmbed::new() //Make Embed
                    .color(0) //Set color to black
                    .title("ScoreSaber Stats - Help") //Set title
                    .description("
                        **HELP**: `!help` - Provides list of commands\n
                        **STATS**: `!stats + player_id [NOTLINKED]` - Provides player statistics\n
                        **TRACK**: `!track + player_id [NOTLINKED] + time` - Provides player statistics repeatedly based on time (seconds)\n
                        **UNTRACK**: `!untrack` - Stops tracking\n
                        **LINK**: `!link + player_id` - Links player_id to messenger's discord\n
                        **UNLINK**: `!unlink` - Deletes link to messenger's discord\n
                        **ID**: `!id + player_name` - Find a player_id by name\n
                    ") //Set description to message taken in
                );
                if let Err(why) = msg.channel_id.send_message(&ctx.http, embed).await { // If sending message has error
                    println!("Error sending message: {:?}", why); // Print error
                }
            }
        }
        _ => { //If command is unknown
            send_simple_format(&ctx, &msg, "Invalid Command").await; //Send error
        }
    }
}