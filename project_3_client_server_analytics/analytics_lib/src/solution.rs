use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

fn row_matches_filter(dataset: &Dataset, row: &Row, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(column_name, value) => {
            let index = dataset.column_index(column_name);
            row.get_value(index) == value
        }
        Condition::Not(inner) => !row_matches_filter(dataset, row, inner),
        Condition::And(left, right) => {
            row_matches_filter(dataset, row, left) && row_matches_filter(dataset, row, right)
        }
        Condition::Or(left, right) => {
            row_matches_filter(dataset, row, left) || row_matches_filter(dataset, row, right)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut result = Dataset::new(dataset.columns().clone());

    for row in dataset.iter() {
        if row_matches_filter(dataset, row, filter) {
            result.add_row(row.clone());
        }
    }

    return result;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();

    for (group_value, grouped_dataset) in dataset {
        let value = match aggregation {
            Aggregation::Count(_column_name) => Value::Integer(grouped_dataset.len() as i32),
            Aggregation::Sum(column_name) => {
                // Add up all values in the chosen column.
                let index = grouped_dataset.column_index(column_name);
                let mut sum = 0;

                for row in grouped_dataset.iter() {
                    match row.get_value(index) {
                        Value::Integer(number) => sum += *number,
                        Value::String(_) => panic!("sum can only be used on integer columns"),
                    }
                }

                Value::Integer(sum)
            }
            Aggregation::Average(column_name) => {
                // Average uses integer division in the tests.
                let index = grouped_dataset.column_index(column_name);
                let mut sum = 0;

                for row in grouped_dataset.iter() {
                    match row.get_value(index) {
                        Value::Integer(number) => sum += *number,
                        Value::String(_) => panic!("average can only be used on integer columns"),
                    }
                }

                Value::Integer(sum / grouped_dataset.len() as i32)
            }
        };

        result.insert(group_value, value);
    }

    return result;
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}