use twilight_http::Client as HttpClient;
use std::boxed::Box;
use twilight_model::channel::message::Message;

use std::{error::Error};
use std::string::String;

use twilight_gateway::{cluster::Cluster};


#[path = "./cmds/ban.rs"]
mod ban;

#[path = "./cmds/userinfo.rs"]
mod userinfo;

#[path = "./cmds/ping.rs"]
mod ping;


pub async fn handle_command<'a>(http: HttpClient, content: String, msg: Message, cl: Cluster) -> Result<(), Box<dyn Error + Send + Sync>> {

    let no_prefix = content.replace("!", "");
    let actual_content = no_prefix.split(" ").collect::<Vec<_>>();

    match actual_content.get(0).unwrap() {
        &"ping" => {
            let _result = ping::ping(http, msg, cl).await?;
        },
        &"ban" => {
            let _result = ban::ban(http, msg).await?;
        },
        &"userinfo" => {
            let _result = userinfo::userinfo(http, msg).await?;
        }
        _ => {}
    }

    Ok(())
}