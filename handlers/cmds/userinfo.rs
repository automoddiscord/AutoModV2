use twilight_embed_builder::{EmbedAuthorBuilder, EmbedBuilder, ImageSource};

use std::{error::Error};

use twilight_http::Client as HttpClient;

use twilight_model::{
    channel::message::Message,
    channel::Channel,
};

const USER_INFO_COLOR: u32 = 0x00_cea2;


pub async fn userinfo(http: HttpClient, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let user = msg.author;

    let mut c = "".to_string();

    let mut builder = EmbedBuilder::new();
    let mut author_builder = EmbedAuthorBuilder::new().name(format!("{}#{}", user.name, user.discriminator))?;

    if let Some(avatar) = user.avatar.as_ref() {
        let extension = if avatar.starts_with("a_") { "gif" } else { "png" };

        author_builder = author_builder.icon_url(ImageSource::url(format!(
            "https://cdn.discordapp.com/avatars/{}/{}.{}",
            user.id,
            user.avatar.as_ref().unwrap(),
            extension
        ))?);
    }
    
    

    let mut gid = None;

    let raw_channel = http.channel(msg.channel_id).await?.unwrap();
    match raw_channel {
        Channel::Guild(guild_channel) => {
            gid = guild_channel.guild_id()
        }
        _ => {}
    }


    let cached_member = http.guild_member(gid.unwrap(), user.id).await?;
    let guild = http.guild(gid.unwrap()).await?.unwrap();

    match cached_member {
        Some(member) => {
            let color = match member.roles.first() {
                Some(role) => guild.roles.iter().filter(|x| x.id == *role).collect::<Vec<_>>().first().unwrap().color,
                None => USER_INFO_COLOR,
            };
            builder = builder.color(color)?;

            let split1 = member.joined_at.unwrap();
            let split2 = split1.split("T").collect::<Vec<_>>();
            let split3 = split2.get(0).into_iter().collect::<Vec<_>>();
            let joined = split3.first().unwrap();

            println!("{:?}", joined);

            let mut roles = "".to_string();
            for (count, role) in member.roles.iter().enumerate() {
                if count > 0 {
                    roles += ", ";
                }

                roles += &format!("<@&{}>", role.0);

                if count == 3 {
                    roles += &format!(" & {} more", member.roles.len() - 3);
                    break;
                }
            }

            if member.roles.is_empty() {
                roles = "0 Roles".to_string()
            }

            c += &format!(
                "**Joined on:** {}\n\n**Roles:** {}",
                joined.replace("-", "/"), roles
            );

        }
        None => {
            builder = builder.color(USER_INFO_COLOR)?;
        }
    }

    builder = builder.author(author_builder.build());

    builder = builder.description(c)?;

    http.create_message(msg.channel_id).embed(builder.build()?)?.await?;

    Ok(())
}