extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::dataset::Value;
use analytics_lib::query::{Aggregation, Condition, Query};
use client::{start_client, solution};

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    fn split_top_level<'a>(text: &'a str, delimiter: &str) -> Option<(&'a str, &'a str)> {
        let mut depth = 0;
        for (i, ch) in text.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                }
                _ => {}
            }

            if depth == 0 && text[i..].starts_with(delimiter) {
                return Some((&text[..i], &text[i + delimiter.len()..]));
            }
        }
        None
    }

    fn is_wrapped_in_parentheses(text: &str) -> bool {
        if !(text.starts_with('(') && text.ends_with(')')) {
            return false;
        }

        let mut depth = 0;
        for (i, c) in text.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    if depth == 0 {
                        return false;
                    }
                    depth -= 1;
                    if depth == 0 && i + c.len_utf8() != text.len() {
                        return false;
                    }
                }
                _ => {}
            }
        }

        depth == 0
    }

    fn parse_value(raw: &str) -> Value {
        if raw.starts_with('"') && raw.ends_with('"') && raw.len() >= 2 {
            Value::String(raw[1..raw.len() - 1].to_string())
        } else {
            Value::Integer(raw.parse().expect("Invalid filter value"))
        }
    }

    fn parse_condition(condition: &str) -> Condition {
        let condition = condition.trim();

        if let Some((left, right)) = split_top_level(condition, " OR ") {
            return Condition::Or(
                Box::new(parse_condition(left)),
                Box::new(parse_condition(right)),
            );
        }

        if let Some((left, right)) = split_top_level(condition, " AND ") {
            return Condition::And(
                Box::new(parse_condition(left)),
                Box::new(parse_condition(right)),
            );
        }

        if let Some(rest) = condition.strip_prefix('!') {
            return Condition::Not(Box::new(parse_condition(rest)));
        }

        if is_wrapped_in_parentheses(condition) {
            return parse_condition(&condition[1..condition.len() - 1]);
        }

        let (column, raw_value) = condition
            .split_once("==")
            .expect("Invalid condition: expected `column == value`");
        Condition::Equal(column.trim().to_string(), parse_value(raw_value.trim()))
    }

    let trimmed = input.trim();
    let (filter_prefix, right) = trimmed
        .split_once(" GROUP BY ")
        .expect("Invalid query: missing ` GROUP BY `");

    let filter_expression = filter_prefix
        .strip_prefix("FILTER ")
        .expect("Invalid query: missing `FILTER ` prefix");
    let filter = parse_condition(filter_expression);

    let parts: Vec<&str> = right.split_whitespace().collect();
    if parts.len() != 3 {
        panic!("Invalid query: expected `GROUP BY <column> <aggregation> <column>`");
    }

    let group_by = parts[0].to_string();
    let aggregate = match parts[1] {
        "COUNT" => Aggregation::Count(parts[2].to_string()),
        "SUM" => Aggregation::Sum(parts[2].to_string()),
        "AVERAGE" => Aggregation::Average(parts[2].to_string()),
        _ => panic!("Invalid query: aggregation must be COUNT, SUM, or AVERAGE"),
    };

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