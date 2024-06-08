//Imports
pub use serde::{Deserialize, Serialize}; //Used for serializing (Convert for easy storage and use) data
pub use serde_json::json; //Used for creating JSon files
pub use reqwest::Error; //Used for handling errors related to HTTP requests
pub use tokio::time::{sleep, Duration}; //Used for async/await and time-based operations such as sleep
pub use std::env; //Used for interacting with environmental variables
pub use rusqlite::{params, Connection, Result}; //Used to integrate database (SQLite) functions
pub use std::collections::HashMap; //Used to create instances without needing to write the whole path each time

//Imports needed for discord bot
pub use serenity::{
    model::{
        channel::{Message}, //Message sent in channel 
        gateway::{
            GatewayIntents, //Events the bot is seeking
            Ready //Event for when bot is ready
        }
    },
    builder::{
        CreateEmbed, //Create embeds
        CreateEmbedAuthor, //Create embedded author
        CreateMessage //Create messaged (can be embed)
    },
    async_trait, // Provides support for async traits
    prelude::* // Commonly used traits and types from serenity
};