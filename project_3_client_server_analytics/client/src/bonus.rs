extern crate tarpc;
extern crate querystring; //delete? i also edited cargo toml

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::Query;
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    
    let ref_to_input = &input;
    let parts: Vec<&str> = ref_to_input.split_whitespace().collect();
    .iter()
    .filter(|word_or_sign| matches!(word_or_sign, &"FILTER" | 
    &"GROUP" | &"BY" | &"COUNT" | &"AVERAGE" | &"AND" | &"OR" | &"!" ))
    .collect::<Vec<_>>();

    



    
    //change the input into 3  parts - filter, group by, aggregation
    //I am using the query from query file

    //divide the string where spaces appear
    //iterate through the input, if element = filter make it a filter part, if group by than group by
    //if count or average then aggregation
    //from filter to group by (or other condition, it is filter's scope)

    // FILTER section == "A1" GROUP BY grade COUNT name
    // FILTER (section == "A1" OR section == "B1") GROUP BY section AVERAGE grade
    // FILTER (!(band == "Meshuggah") AND !(band == "Vildhjarta")) GROUP BY album AVERAGE rating

    let query = analytics_lib::query::Query::new(...);
    return query;

//  --------------------------------------------------------------IDEAS GREAVEYARD--------------------------------------------------------------
// let query = Query::from(querified);
//let parameters = querystring::querify(ref_to_input);
//return query; good
//
//     let params = vec![("id", "123"), ("type", "admin")];
    
//     let query: String = params
//         .iter()
//         .map(|(k, v)| format!("{}={}", k, v))
//         .collect::<Vec<_>>()
//         .join("&");

//     println!("{}", query);

// //
//let results = sql_query("FILTER section == A1 GROUP BY grade COUNT name");


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