extern crate http;
extern crate serde;
extern crate serde_json;

// use http::{Request, Response};
use std::fs;
// use serde_json::Result;
use serde::{Deserialize, Serialize};

// enum GuildEventFrequency
// {
//     YEARLY = 0,
//     MONTHLY = 1,
//     WEEKLY = 2,
//     DAILY = 3,
// }
//
// struct RecurrenceRule
// {
//     start: Date,
//     end: Option<Date>,
//     frequency: GuildEventFrequency,
//     interval: i32
// }

#[derive(Serialize, Deserialize)]
struct GuildEvent
{
    guild_id: String,
    // description: Option<String>,
    // scheduled_start_time: String,
    // scheduled_end_time: Option<String>,
    // status:
    // image: Option<String>,
    // recurrence_rule: Option<RecurrenceRule>
}

fn main()
{
    const DGTC_GUILD_ID: &str = "938566693154861126";
    const TEST_GUILD_ID: &str = "1563199948915347539";

    let discord_token: String = fs::read_to_string("discord_token").expect("No discord token file found");

    // let mut request = Request::builder()
    //     .uri("https://discord.com/api/v8/guilds/" + TEST_GUILD_ID + "/scheduled-events")
    //     .header("Authorization", discord_token);
    //
    // let response = send(request.body(()).unwrap());
    // dbg!(response);

    let response: &str = r#"{ "guild_id": "hi", "wa": 14 }"#;

    let event: GuildEvent = serde_json::from_str(response).expect("A?");
    println!("{}", event.guild_id);
}


