use std::{error::Error};

use twilight_http::Client as HttpClient;

use twilight_model::{
    channel::message::Message,
    id::UserId,
    channel::Channel
};

#[path = "../../utils/matchers.rs"]
mod matchers;



pub async fn ban(http: HttpClient, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = msg.content.split(" ").collect::<Vec<_>>(); // remove the prefix from the actual message
    
    
    if args.len() <= 1 {
        http.create_message(msg.channel_id).content("Who should I ban?")?.await?;
    } else {
        let rest = args.get(1).into_iter().collect::<Vec<_>>();
        let rest2 = args.get(2..).into_iter().collect::<Vec<_>>();

        let reason_list = rest2.first().unwrap();
        let _reason = reason_list.join(" ");
        let uid = {
            let m = rest.first().unwrap();
            matchers::get_mention(&m).ok_or("You are missing a required command parameter!")?
        };

        let mut gid = None;
        let raw_channel = http.channel(msg.channel_id).await?.unwrap();
        match raw_channel {
            Channel::Guild(guild_channel) => {
                gid = guild_channel.guild_id()
            }
            _ => {}
        }
        if let Err(_e) = http.guild_member(gid.unwrap(), UserId(uid)).await {
            http.create_message(msg.channel_id).content("❌ Failed to ban this user")?.await?;
        } else {
            let cached_member = http.guild_member(gid.unwrap(), UserId(uid)).await?;
            let guild = http.guild(gid.unwrap()).await?.unwrap();

            match cached_member {
                Some(target) => {
                    if let Err(_e) = http.create_ban(guild.id, UserId(uid))
                                        .delete_message_days(7)?
                                        .await {
                        http.create_message(msg.channel_id).content("❌ Failed to ban this user")?.await?;
                    } else {
                        let _content = format!(
                            "👌 Banned {}#{} for ``{}``",
                            target.user.name, target.user.discriminator, _reason
                        );
                        http.create_message(msg.channel_id).content(_content)?.await?;
                    }
                }
                _ => {
                    http.create_message(msg.channel_id).content("❌ Failed to ban this user")?.await?;
                }
            }
        }
        
    }

    Ok(())
}