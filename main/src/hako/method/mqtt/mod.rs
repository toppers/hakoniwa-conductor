extern crate lazy_static;
extern crate once_cell;
use once_cell::sync::Lazy;
use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt as mqtt;
use std::{process, time::Duration};
use async_std;
use std::thread::sleep;
use std::{sync::Mutex};
use crate::hako::pdu::ASSET_PUB_PDU_CHANNELS;
use crate::hako::pdu::ASSET_SUB_PDU_CHANNELS;
use crate::hako::pdu::write_asset_pub_pdu;
use crate::hako::pdu::get_asset_pub_pdu_channel_robo_name;
static mut MQTT_SERVER_ACTIVATED: bool = false;
use crate::hako::api;

struct MqttOptions {
    pub url: String,
    pub portno: i32
}
static MQTT_URL: Lazy<Mutex<Vec<MqttOptions>>> = Lazy::new(|| {
    Mutex::new(vec![])
});
pub fn set_mqtt_url(ipaddr: String, port: i32)
{
    let mut v = MQTT_URL.lock().unwrap();
    if v.is_empty() {
        let mqtt_options = MqttOptions {
            url: format!("mqtt://{}:{}", ipaddr.clone(), port),
            portno: port
        };
        println!("mqtt_url={}", mqtt_options.url.clone());
        v.push(mqtt_options);
    }
}
pub fn get_mqtt_port() -> i32
{
    let v = MQTT_URL.lock().unwrap();
    if v.is_empty() {
        return -1;
    }
    v.first().unwrap().portno
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
    v.first().unwrap().url.clone()
}

fn create_topics(topics: &mut Vec<String>, qoss: &mut Vec<i32>)
{
    let mut map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    for (real_id, pdu) in map.iter_mut() {
    //for channel_id in 0..4 {
        println!("create topic: real_id={} method_type={}", real_id, pdu.method_type);
        if pdu.method_type == "MQTT" {
            let combined = format!("hako_mqtt_{}_{}", pdu.robo_name, pdu.channel_id);
            println!("create topic: {}", combined.clone());
            topics.push(combined);
            qoss.push(1);
        }
    }
}

fn get_channel_id(topic: String) -> i32
{
    let mut map = ASSET_PUB_PDU_CHANNELS.lock().unwrap();
    for (real_id, pdu) in map.iter_mut() {
    //for channel_id in 0..4 {
        if pdu.method_type == "MQTT" {
            let combined = format!("hako_mqtt_{}_{}", pdu.robo_name, pdu.channel_id);
            if topic == combined {
                return real_id.clone();
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
            println!("SUBSCRIBER Connecting to the MQTT server...");
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
                    //println!("recv topic={}", msg.topic());
                    let real_id = get_channel_id(msg.topic().to_string());
                    assert!(real_id >= 0);
                    //println!("write_asset_pub_pdu: real_id={} size={}", real_id, msg.payload().len());
                    let (channel_id, robo_name) = get_asset_pub_pdu_channel_robo_name(real_id);
                    let ret = write_asset_pub_pdu(robo_name, channel_id, msg.payload(), msg.payload().len());
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

pub fn create_mqtt_publisher() -> Option<mqtt::Client>
{
    let ip_port = get_mqtt_url();
    let create_opts = mqtt::CreateOptionsBuilder::new_v3()
        .server_uri(ip_port)
        .client_id("hako-mqtt_publisher")
        .finalize();
    let mut cli = mqtt::Client::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });
    cli.set_timeout(Duration::from_secs(5));
    println!("PUBLISHER Connecting to the MQTT server...");
    while let Err(err) = cli.connect(None) {
        println!("Error pub reconnecting: {}", err);
        // For tokio use: tokio::time::delay_for()
        sleep(Duration::from_millis(1000));
    }
    println!("PUBLISHER CONNECTED to the MQTT server...");
    Some(cli)
}

pub fn publish_mqtt_topics(cli: &mqtt::Client)
{
    //println!("publish_mqtt_topics(): start");
    let mut map = ASSET_SUB_PDU_CHANNELS.lock().unwrap();
    for (_real_id, pdu) in map.iter_mut() {
        if pdu.method_type == "MQTT" {
            assert!(pdu.pdu_size < 4096);
            //println!("publish_mqtt_topics(): send channel_id={}", channel_id);
            let mut buf : Box<[u8]> = Box::new([0; 4096]);
            let result = api::asset_read_pdu(
                pdu.asset_name.clone(), 
                pdu.robo_name.clone(), 
                pdu.channel_id, 
                buf.as_mut_ptr() as *mut i8, 
                pdu.pdu_size as i32);
            if result {
                let topic = format!("hako_mqtt_{}_{}", pdu.robo_name.clone(), pdu.channel_id);
                let msg = mqtt::Message::new(topic, buf, mqtt::QOS_1);
                if let Err(e) = cli.publish(msg) {
                    println!("Error sending message: {:?}", e);
                }
            }
        }
    }
    //println!("publish_mqtt_topics(): end");
}
