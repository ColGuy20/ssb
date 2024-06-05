//Imports
pub use serde::{Deserialize, Serialize}; //Used for serializing (Convert for easy storage and use) data
pub use serde_json::json; //Used for creating JSon files
pub use reqwest::Error; //Used for handling errors related to HTTP requests
pub use tokio::time::{sleep, Duration}; //Used for async/await and time-based operations such as sleep
pub use std::env; //Used for environmental variables
pub use rusqlite::{params, Connection, Result}; //Used to integrate database (SQLite) functions
pub use std::collections::HashMap; //Used to create instances without needing to write the whole path each time