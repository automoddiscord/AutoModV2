use std::time::{Duration, Instant};
use std::{thread};
use std::{error::Error};

use twilight_http::Client as HttpClient;
use twilight_gateway::{cluster::Cluster};

use twilight_model::{
    channel::message::Message
};

pub async fn ping(http: HttpClient, msg: Message, cl: Cluster) -> Result<(), Box<dyn Error + Send + Sync>> {
    let start_time = Instant::now();

    let sent_msg = http.create_message(msg.channel_id).content("⏳ Pinging...")?.await?;

    let rest = start_time.elapsed().as_millis();

    let cluster_info = cl.info();

    let ws_avg_time = cluster_info
        .into_iter()
        .filter_map(|(_, info)| info.latency().average())
        .sum::<Duration>()
        .as_millis();

    let o = Duration::from_millis(20);
    thread::sleep(o); // we have to wait 15 ms, otherwise we can't receive a heartbeat from the websocket (wtf?)
    
    let edited_msg = format!("🏓 Pong! Client Latency: {} ms | REST API ping: {} ms", rest, ws_avg_time);

    http.update_message(sent_msg.channel_id, sent_msg.id).content(edited_msg)?.await?;

    Ok(())
}