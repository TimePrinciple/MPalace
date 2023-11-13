use anyhow::Result;
use futures::stream::StreamExt;
use mongodb::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Record {
    Q: String,
    A: String,
}

enum Command {
    // Triggered by 'q'
    Quit,
    // Triggered by 'enter key strock'
    Insert,
    // Triggered by 'p'
    Print,
}

pub async fn app() {
    // Initializing process
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = client.database("MPalace");
    let records = db.collection::<Record>("records");

    // Read input
    loop {
        println!("MPalace");
        println!("'enter' to insert Q&A, 'p' to print records, 'q' to quit");
        println!("-------------------------------------------------");
        let input = get_input().await.unwrap();
        match parse_command(&input).await {
            Command::Insert => {
                // Form a record to be inserted to mongodb
                println!("Inserting Q&A records:");
                println!("-------------------------------------------------");
                println!("Q:");
                let Q = get_input().await.unwrap();
                println!("-------------------------------------------------");
                println!("A:");
                let A = get_input().await.unwrap();
                println!("-------------------------------------------------");
                println!("Record to be inserted: ");
                println!("Q: {}", Q);
                println!("A: {}", A);
                println!("-------------------------------------------------");
                println!("Type 'Enter' to confirm, anything else to cancel:");
                let input = get_input().await.unwrap();
                match input.as_str() {
                    "" => {
                        let record = Record { Q, A };
                        records.insert_one(record, None).await.unwrap();
                        println!("Insertion completed");
                        println!("-------------------------------------------------");
                    }
                    _ => {
                        println!("Aborting insertion...");
                        println!("-------------------------------------------------");
                    }
                }
            }
            Command::Print => {
                println!("Displaying records in collection");
                println!("-------------------------------------------------");
                let mut cursor = records.find(None, None).await.unwrap();
                while let Some(record) = cursor.next().await {
                    let record = record.unwrap();
                    println!("Q: ");
                    println!("{}", record.Q);
                    println!("A: ");
                    println!("{}", record.A);
                    println!("-------------------------------------------------");
                }
            }
            Command::Quit => {
                println!("Quiting MPalace...");
                break;
            }
        }
    }
}

async fn parse_command(input: &str) -> Command {
    match input {
        "q" => Command::Quit,
        "p" => Command::Print,
        "" => Command::Insert,
        _ => Command::Insert,
    }
}

async fn get_input() -> Result<String, ()> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err(()),
    }
}