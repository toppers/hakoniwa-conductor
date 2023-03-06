extern crate lazy_static;
extern crate once_cell;
use once_cell::sync::Lazy;
use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt as mqtt;
use std::{process, time::Duration};
use async_std;
use std::{sync::Mutex};
use crate::hako::pdu::ASSET_SUB_PDU_CHANNELS;
use crate::hako::pdu::write_asset_pub_pdu;
static mut MQTT_SERVER_ACTIVATED: bool = false;

static MQTT_URL: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(vec![])
});
pub fn set_mqtt_url(ipaddr: String, port: i32)
{
    let mut v = MQTT_URL.lock().unwrap();
    if v.is_empty() {
        let mqtt_url = format!("mqtt://{}:{}", ipaddr, port);
        println!("mqtt_url={}", mqtt_url.clone());
        v.push(mqtt_url);
    }
}
pub fn is_enabled() -> bool
{
    let v = MQTT_URL.lock().unwrap();
    v.is_empty() == false
}
pub fn get_mqtt_url() -> String
{
    let v = MQTT_URL.lock().unwrap();
    if v.is_empty() {
        return String::from("None");
    }
    v.first().unwrap().clone()
}

fn create_topics(topics: &mut Vec<String>, qoss: &mut Vec<i32>)
{
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    for (channel_id, pdu) in map.iter_mut() {
    //for channel_id in 0..4 {
        if pdu.method_type == "MQTT" {
            let combined = format!("hako_mqtt_{}", channel_id);
            topics.push(combined);
            qoss.push(1);
        }
    }
}

fn get_channel_id(topic: String) -> i32
{
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    for (channel_id, pdu) in map.iter_mut() {
    //for channel_id in 0..4 {
        if pdu.method_type == "MQTT" {
            let combined = format!("hako_mqtt_{}", channel_id);
            if topic == combined {
                return channel_id.clone();
            }
        }
    }
    println!("ERROR: unknown channel_id... topic={}", topic);
    -1
}

pub fn activate_server()
{
    unsafe {
        if MQTT_SERVER_ACTIVATED == true {
            return;
        }
        MQTT_SERVER_ACTIVATED = true;
    }
    let ip_port = get_mqtt_url();
    let mut topics: Vec<String> = Vec::with_capacity(100);
    let mut qoss: Vec<i32> = Vec::with_capacity(100);
    create_topics(&mut topics, &mut qoss);
    let buffer_siz = 1024 * 1024;
    let create_opts = mqtt::CreateOptionsBuilder::new_v3()
        .server_uri(ip_port)
        .client_id("hako-mqtt_subscriber")
        .finalize();
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });
    std::thread::spawn(move || {
        if let Err(err) = block_on(async {
            // Get message stream before connecting.
            let mut strm = cli.get_stream(buffer_siz);
    
            // Define the set of options for the connection
            let lwt = mqtt::Message::new("topic", "Async subscriber lost connection", mqtt::QOS_1);
    
            // Create the connect options, explicitly requesting MQTT v3.x
            let conn_opts = mqtt::ConnectOptionsBuilder::new_v3()
                .keep_alive_interval(Duration::from_secs(30))
                .clean_session(false)
                .will_message(lwt)
                .finalize();
    
            // Make the connection to the broker
            println!("Connecting to the MQTT server...");
            //cli.connect(conn_opts).await?;
            while let Err(err) = cli.connect(conn_opts.clone()).await {
                println!("Error reconnecting: {}", err);
                // For tokio use: tokio::time::delay_for()
                async_std::task::sleep(Duration::from_millis(1000)).await;
            }
    
            println!("Subscribing to topics: {:?}", topics);
            cli.subscribe_many(topics.as_slice(), qoss.as_slice()).await?;
    
            // Just loop on incoming messages.
            println!("Waiting for messages...");
    
            // Note that we're not providing a way to cleanly shut down and
            // disconnect. Therefore, when you kill this app (with a ^C or
            // whatever) the server will get an unexpected drop and then
            // should emit the LWT message.
    
            while let Some(msg_opt) = strm.next().await {
                if let Some(msg) = msg_opt {
                    //println!("{}", msg);
                    let channel_id = get_channel_id(msg.topic().to_string());
                    assert!(channel_id >= 0);
                    let ret = write_asset_pub_pdu(channel_id, msg.payload(), msg.payload().len());
                    assert!(ret == true);
                }
                else {
                    // A "None" means we were disconnected. Try to reconnect...
                    println!("Lost connection. Attempting reconnect.");
                    while let Err(err) = cli.reconnect().await {
                        println!("Error reconnecting: {}", err);
                        // For tokio use: tokio::time::delay_for()
                        async_std::task::sleep(Duration::from_millis(1000)).await;
                    }
                }
            }
    
            // Explicit return type for the async block
            Ok::<(), mqtt::Error>(())
        }) {
            eprintln!("{}", err);
        }
    });
}


pub fn send_all_subscriber()
{
    //Not supported...
}
