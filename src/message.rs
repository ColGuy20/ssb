use crate::imports::*; //Imports needed for program
use crate::send_stats; //Function to fetch, store, and send stats

//Static variables
pub static mut TRACK: bool = true; //Used to stop tracking
pub static mut COUNT: i64 = 0; //Goes up every time stats sent successfully
pub static mut TRACKING: bool = false; //Shows if bot is tracking

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
    let mut cooldown:u64 = 6; //Default cooldown is 5 minutes

    let message = msg.content.as_str(); //Message to &str
    let mut message_id = true; //Message ID validation

    let mut message_parts = message.split(' '); //Break apart words in message
    let command = message_parts.next().unwrap(); //Set command to first word
    let player_id = message_parts.next().unwrap_or(""); //Set player_id to second word

    // Make sure message has right number of words
    let message_word_count = message.split_whitespace().count(); //Count the number of words in the message
    let mut stats_working = true; //If stats does not have the correct word count
    let mut track_working = true; //If track does not have the correct word count
    if message_word_count > 1 && command != "/track" && command != "/stats" { //If the message has more than one word
        if command == "/untrack"{ //If the command is untrack
            send_simple_format(&ctx, &msg, "Incorrect number of fields. (`/untrack`)").await; //Send error
        } else {
            send_simple_format(&ctx, &msg, "Incorrect number of fields.").await; //Send error
        }
    } else if command == "/stats" && message_word_count != 2 { //If command is stats and there is not two words in message
        send_simple_format(&ctx, &msg, "Incorrect number of fields. (`/stats` `player_id`)").await; //Send error
        stats_working = false; //Command stats is not working
    } else if command == "/track"{ //If command is track
        if message_word_count != 3 { //If there is not three words in message
            send_simple_format(&ctx, &msg, "Incorrect number of fields. (`/track` `player_id` `time (seconds)`)").await; //Send error
            track_working = false; //Command stats is not working
        } else {
            match message_parts.next().unwrap_or("").parse::<u64>() { //Try to parse cooldown from message
                Ok(num) => { //If able to parse
                    if num > 5{ //If number is greater than 5
                        cooldown = num //Set cooldown
                    }  else {
                        send_simple_format(&ctx, &msg, "Please use a number over 5 seconds. (`/track` `player_id` `time (seconds)`)").await; //Send error
                        track_working = false; //Command track is not working
                    }
                }
                Err(e) => { //If unable to parse
                    send_simple_format(&ctx, &msg, "Please use a number for *time*. Make sure it is positive. (`/track` `player_id` `time (seconds)`)").await; //Send error
                    println!("Error parsing cooldown: {:?}", e); //Print error
                    track_working = false; //Command track is not working
                }
            }
        }
    } else if player_id.is_empty() { //If there is not a message id provided
        message_id = false; //Message ID is not present
    }

    match command { //Ract to command
        "/stats" => { //If command is stats
            if !message_id{ //If there is no player ID provided
                send_simple_format(&ctx, &msg, "There is no player ID provided. (`/stats` `player_id`)").await; //Send error
            }
            if stats_working{ //If stats is working
                if !send_stats(player_id, ctx.clone(), msg.clone()).await{ //Function to send stats
                    send_simple_format(&ctx, &msg, "Invalid player ID provided.").await; //Send error
                }
                unsafe{ //Using pub static variables
                    COUNT += 1; // Increment the success count
                    println!("Stats sent successfully to Discord [Message Count: {}] (/stats)", COUNT); // Prints if successful
                }
            }
        }
        "/track" => { //If command is track
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
                if !message_id{ //If player id is not valid
                    send_simple_format(&ctx, &msg, "There is no player ID provided. (`/track` `player_id` `time (seconds)`)").await; //Send error
                    break; //Exit the loop
                }
                if !track_working{break} //If /track is not working break
                if loop_count == 0{send_simple_format(&ctx, &msg, format!("Began tracking every {} seconds!", cooldown).as_str()).await} //If it is looping for the same time then print
                if !send_stats(player_id, ctx.clone(), msg.clone()).await{ //Function to send stats
                    send_simple_format(&ctx, &msg, "**FAILED:** Invalid player ID provided.").await; //Send error
                    break; //Exit the loop
                }
                unsafe{ //Using pub static variables
                    COUNT += 1; // Increment the success count
                    println!("Stats sent successfully to Discord [Message Count: {}] (/track [Every {} seconds])", COUNT, cooldown); // Prints if successful
                }
                sleep(Duration::from_secs(cooldown)).await; //Waits before looping
            }
            unsafe{ //Using pub static variables
                TRACKING = false; //Bot has stopped tracking
            }
        }
        "/untrack" => { //If command is untrack
            unsafe{ //Using pub static variables
                if TRACKING{ //If bot is tracking
                    TRACK = false; //Stop looping in track
                    send_simple_format(&ctx, &msg, "Tracking has stopped!").await; //Send message
                    println!("Tracking has stopped (/untrack)"); // Prints if successful
                } else {
                    send_simple_format(&ctx, &msg, "Not currently tracking!").await; //Send error
                }
            }
        }
        _ => { //If command is unknown
            send_simple_format(&ctx, &msg, "Invalid Command").await; //Send error
        }
    }
}