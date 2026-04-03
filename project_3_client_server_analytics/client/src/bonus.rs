extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::dataset::Value;
use analytics_lib::query::{Aggregation, Condition, Query};
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    let group_split = input.split_once(" GROUP BY ").unwrap();
    let left = group_split.0;
    let right = group_split.1;

    //maybe add .expect for wrong format or some unwraps
    //DEAR DHIRAJ IF I FORGET TO HANDLE WRONG FORMAT OF QUERY, ADD EXPECT OR HANDLE UNWRAPS HERE
    // THIS MIGHT NOT BE NECESSARY, LETS ASK KINAN
    // DO NOT COMMIT YET, MAYBE STASH
    
    let filter_raw = left.trim_start_matches("FILTER ").trim();
    let eq_split = filter_raw.split_once("==").unwrap();
    let col = eq_split.0.trim();
    let raw = eq_split.1.trim();

    let value = if raw.starts_with('"') && raw.ends_with('"') {
        Value::String(raw.trim_matches('"').to_string())
    } 
    else 
    {
        Value::Integer(raw.parse().unwrap())
    };
    let filter = Condition::Equal(col.to_string(), value);

    let count_split = right.split_once(" COUNT ");
    let group_by;
    let aggregate;

    if let Some((g, c)) = count_split {
        group_by = g.trim().to_string();
        aggregate = Aggregation::Count(c.trim().to_string());
    } 
    else 
    {
        let avg_split = right.split_once(" AVERAGE ").unwrap();
        group_by = avg_split.0.trim().to_string();
        aggregate = Aggregation::Average(avg_split.1.trim().to_string());
    }

    Query::new(filter, group_by, aggregate)

}

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}