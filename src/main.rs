mod rule;
mod queue;

extern crate clap;
use std::collections::HashMap;
use clap::{Arg, App, SubCommand};
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use futures_lite::stream::StreamExt;
use subprocess::Exec;

#[macro_use] extern crate log;
extern crate simplelog;

use simplelog::*;

use std::fs::File;


fn create_connection(addr: &str, consumers: HashMap<String, Vec<queue::Consumer>>) {
    async_global_executor::block_on(async {
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");


        let channel = conn.create_channel().await.expect("create_channel");
     
        for queue in consumers.keys() {
            let mut consumer = channel.basic_consume(
                queue,
                queue,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            ).await
            .expect("basic_consume");

            let queue_consumers: Vec<queue::Consumer> = consumers.get(queue).unwrap().to_vec();


            async_global_executor::spawn(async move {
                info!("will consume");
                while let Some(delivery) = consumer.next().await {
                    let (_, delivery) = delivery.expect("error in consumer");

                    //assuming UTF8
                    if let Ok(message) = std::str::from_utf8(&delivery.data) {
                        exec_if_match(message, &queue_consumers);
                    } else {
                        info!("Could not transform data into UTF8 string, dropping message");
                    }
                    
                    delivery
                        .ack(BasicAckOptions::default())
                        .await
                        .expect("ack");
                }
            }).detach();

        }

    })
}

fn exec_if_match(message: &str, consumers: &Vec<queue::Consumer>) {
    for consumer in consumers {
        if consumer.rule.should_exec(message) {
            info!("Found a consumer for the event");
            // this probably should not be here
            let exec = str::replace(&consumer.exec, "${event}", message);
            // We should probably log the result, but for now running and getting exit code
            if let Ok(result) = Exec::shell(exec).join() {
                info!("{:?}", result);
            } else {
                info!("Command failed");
            }
        } 
    }
}


fn main() {
    let matches = App::new("EventLauncher")
                          .version("0.0.1")
                          .author("Elias Granja <me@elias.sh>")
                          .about("Launches scripts based on RabbitMQ events")
                          .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets a custom config file")
                               .required(true)
                               .index(1)
                               .takes_value(true))
                           .arg(Arg::with_name("rabbitmq")
                               .short("r")
                               .long("rabbitmq")
                               .value_name("URL")
                               .help("connection url for rabbitmq")
                               .index(2)
                               .required(true)
                               .takes_value(true))
                               
                        .get_matches();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("event_launcher.log").unwrap()),
        ]
    ).unwrap();

    let config = matches.value_of("config").unwrap();

    if let Some(consumers) = queue::from_file(config) {
        create_connection(matches.value_of("rabbitmq").unwrap(), consumers);
        loop {}
    } else {
        panic!("Could not create consumers");
    }
}