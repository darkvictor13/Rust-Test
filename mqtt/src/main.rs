use paho_mqtt as mqtt;
use std::{process, time::Duration};

fn main() {
    let create_options = mqtt::CreateOptionsBuilder::new()
        .client_id("client_in_rust")
        .server_uri("tcp://localhost:1883")
        .finalize();

    let mut cli = mqtt::Client::new(create_options)
    .unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    cli.set_timeout(Duration::from_secs(5));

    cli.connect(None)
    .unwrap_or_else(|e| {
        println!("Error connecting: {:?}", e);
        process::exit(1);
    });

    let msg = mqtt::MessageBuilder::new()
        .topic("topico")
        .payload("valor")
        .qos(0)
        .retained(false)
        .finalize();

    cli.publish(msg)
    .unwrap_or_else(|e| {
        println!("Error sending message: {:?}", e);
        process::exit(1);
    });

    cli.disconnect(None).unwrap();
}
