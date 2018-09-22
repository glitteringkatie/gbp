#[macro_use] extern crate tower_web;
extern crate redis;
extern crate tokio;
extern crate url;
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;
use regex::Regex;
use std::collections::HashMap;
use redis::{Commands, PipelineCommands};
use tokio::prelude::*;
use tower_web::ServiceBuilder;
use std::error::Error;

/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
struct HelloWorld {
    db: redis::Client,
}

#[derive(Debug)]
enum GbpCommand {
    Leaderboard,
    Give {
        to: String,
        to_id: String,
        from: String,
        from_id: String,
        reason: String
    },
    Take {
        to: String,
        to_id: String,
        from: String,
        from_id: String,
        reason: String
    },
    Invalid,
}

#[derive(Response)]
struct GbpResponse {
    text: String,
    response_type: String,
}

const HELP_TEXT: &'static str = r#"
Invalid command, dude. Try one of these:

/gbp leaderboard
/gbp give @jonah the goodest boi
/gbp take @cooper for being a snake
"#;

impl_web! {
    impl HelloWorld {
        #[post("/")]
        #[content_type("json")]
        fn hello_world(&self, body: Vec<u8>) -> Result<GbpResponse, ()> {
            // Get datastore connection
            let conn = self.db.get_connection().unwrap();

            // Parse query
             let query_params: HashMap<String, String> =
                url::form_urlencoded::parse(body.as_ref())
                    .into_owned()
                    .collect();
            println!("{:?}", query_params);

            // Parse intent
            let command = self.parse_intent(query_params).unwrap();
            println!("{:?}", command);

            // Process intent
            return self.process_intent(command, conn);
        }

        fn parse_intent(&self, query_params: HashMap<String, String>) -> Result<GbpCommand, ()> {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(.*) <@(.*)\|(.*)> (.*)").unwrap();
            }

            let user_name = query_params.get("user_name").unwrap();
            let user_id = query_params.get("user_id").unwrap();
            let text = query_params.get("text").unwrap().as_str();
            let command = text.split(" ").next().unwrap();

            match command {
                "give" => {
                    let capture = RE.captures_iter(text).next().unwrap();

                    Ok(GbpCommand::Give{
                        to: capture[3].to_string(),
                        to_id: capture[2].to_string(),
                        from: user_name.to_string(),
                        from_id: user_id.to_string(),
                        reason: capture[4].to_string()
                    })
                },
                "take" => {
                    let capture = RE.captures_iter(text).next().unwrap();

                    Ok(GbpCommand::Take{
                        to: capture[3].to_string(),
                        to_id: capture[2].to_string(),
                        from: user_name.to_string(),
                        from_id: user_id.to_string(),
                        reason: capture[4].to_string()
                    })
                },
                "leaderboard" => Ok(GbpCommand::Leaderboard),
                _ => Ok(GbpCommand::Invalid),
            }
        }

        fn process_intent(&self, command: GbpCommand, conn: redis::Connection) -> Result<GbpResponse, ()> {
            match command {
                GbpCommand::Leaderboard => {
                    Ok(GbpResponse {
                        text: "soz cop, we don't support that right now.".to_string(),
                        response_type: "ephemeral".to_string()
                    })
                },
                GbpCommand::Give { to, to_id, from, from_id, reason } => {
                    let key = to_id.as_str();
                    let (new_val,): (isize,) = redis::transaction(&conn, &[key], |pipe| {
                        let old_val: isize = conn.get(key).unwrap_or(0);
                        pipe
                            .set(key, old_val + 1).ignore()
                            .get(key).query(&conn)
                    }).unwrap();

                    Ok(GbpResponse {
                        text: format!("@{} gave a good boy point to @{}: {}.\nThey now have {} good boy points.", from, to, reason, new_val),
                        response_type: "in_channel".to_string()
                    })
                },
                GbpCommand::Take { to, to_id, from, from_id, reason } => {
                    let key = to_id.as_str();
                    let (new_val,): (isize,) = redis::transaction(&conn, &[key], |pipe| {
                        let old_val: isize = conn.get(key).unwrap_or(0);
                        pipe
                            .set(key, old_val - 1).ignore()
                            .get(key).query(&conn)
                    }).unwrap();

                    Ok(GbpResponse {
                        text: format!("@{} took a good boy point from @{}: {}.\nThey now have {} good boy points.", from, to, reason, new_val),
                        response_type: "in_channel".to_string()
                    })
                },
                GbpCommand::Invalid => {
                    Ok(GbpResponse {
                        text: HELP_TEXT.to_string(),
                        response_type: "ephemeral".to_string()
                    })
                }
            }
        }
    }
}

pub fn main() {
    let port = env::var("PORT").expect("PORT is not set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not set");

    let addr = format!("0.0.0.0:{}", port).parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let db = redis::Client::open(redis_url.as_str()).unwrap();
    let gbp = HelloWorld { db: db };

    ServiceBuilder::new().resource(gbp).run(&addr).unwrap();
}
