use crate::PlayerData; //PlayerData struct
use crate::Changes; //Changes struct
use crate::imports::*; //Imports

//Comparing new-old data main function
pub fn compare_data(data: &PlayerData, data_from_db: &PlayerData) -> Result<Changes, Error>{
    //Serializing data for comparing
    let serialized_data: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data)).unwrap();
    let serialized_data_from_db: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data_from_db)).unwrap();
    let serialized_scoreStats: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data.scoreStats)).unwrap();
    let serialized_scoreStats_from_db: HashMap<String, serde_json::Value> = serde_json::from_value(json!(data_from_db.scoreStats)).unwrap();

    //Make instance variable based of Changes struct
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

    //Checks and responds if data changed
    for (key, value) in &serialized_data { //Loops to check differences in all data
        if let Some(value_from_db) = serialized_data_from_db.get(key){ //If the data exists in both
            //Make variable that excludes certain fields of PlayerData
            let exclude: bool = matches!(key.as_str(),"id"|"name"|"country"|"firstSeen"|"banned"|"inactive"|"profilePicture"|"histories");
            if value != value_from_db && !exclude { //If the old data does not match the new data and is not excluded
                if key == "scoreStats" { //If the key being checked is scoreStats
                    for (key, value) in &serialized_scoreStats { //Loops to check differences in scoreStats
                        if let Some(value_from_db) = serialized_scoreStats_from_db.get(key) { //If the data exists in both
                            if value != value_from_db { //If the data is different
                                if key == "averageRankedAccuracy" { //Since this is a f64, then it is seperate
                                    changes.rAccuracy = true; //There is a change
                                    changes.rAccuracy_change = value.as_f64().unwrap() - value_from_db.as_f64().unwrap(); //Puts change in the instance
                                } else { //If the data in scoreStats is not averageRankedAccuracy
                                    let change = value.as_i64().unwrap() - value_from_db.as_i64().unwrap();
                                    match key.as_str() { //Responds according to the variable (identified by the key)
                                        "totalScore" => { //If the key is "totalScore" and so on..
                                            changes.tScore = true; //There is a change
                                            changes.tScore_change = change; //Puts the change in the instance
                                        }
                                        "totalRankedScore" => {
                                            changes.rScore = true;
                                            changes.rScore_change = change;
                                        }
                                        "totalPlayCount" => {
                                            changes.tCount = true;
                                            changes.tCount_change = change;
                                        }
                                        "rankedPlayCount" => {
                                            changes.rCount = true;
                                            changes.rCount_change = change;
                                        }
                                        "replaysWatched" => {
                                            changes.replays = true;
                                            changes.replays_change = change;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                } else { //If the key being checked is not in score stats
                    match key.as_str() { //Responds according to the variable (identified by the key)
                        "pp" => { //If the key is "pp" and so on...
                            changes.pp = true; //There is a change
                            changes.pp_change = value.as_f64().unwrap() - value_from_db.as_f64().unwrap();//Puts the change in the instance
                        }
                        "rank" => {
                            changes.rank = true;
                            changes.rank_change = value.as_i64().unwrap() - value_from_db.as_i64().unwrap();
                        }
                        "countryRank" => {
                            changes.cRank = true;
                            changes.cRank_change = value.as_i64().unwrap() - value_from_db.as_i64().unwrap();
                        }
                        _ => {}
                    }
                }
            }
        } 
    }
    Ok(changes) //Return information about chnages
}