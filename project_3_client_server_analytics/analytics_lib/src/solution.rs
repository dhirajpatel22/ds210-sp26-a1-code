use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

//helper function that checks if a row satifies a condition
//takes a row, dataset, and condition as arguments
//returns true if the row satifies the condtion, returns false if the row does not satisfy the condition
pub fn satisfies_condition(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    let mut bool = false; 
    match condition {
        Condition::Equal(column_name, value) => {
            let index = dataset.column_index(column_name);
            let item = row.get_value(index);
            if item == value {
                bool = true;
            }
        },
        Condition::Not(condition) => {
            bool = !satisfies_condition(row, dataset, condition);
        },
        Condition::And(condition_1, condition_2) => {
            bool = satisfies_condition(row, dataset, condition_1) && satisfies_condition(row, dataset, condition_2);
        }, 
        Condition::Or(condition_1, condition_2) => {
            bool = satisfies_condition(row, dataset, condition_1) || satisfies_condition(row, dataset, condition_2);
        }
    }

    return bool
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut filtered_dataset = Dataset::new(dataset.columns().clone());
    for row in dataset.iter() {
           if satisfies_condition(row, dataset, filter) == true {
            filtered_dataset.add_row(row.clone());
           }
    }
 return filtered_dataset;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let mut map: HashMap<Value, Dataset> = HashMap::new();
    let index = dataset.column_index(group_by_column);

    for row in dataset.iter(){
        let key = row.get_value(index);

        map.entry(key.clone())
            .or_insert_with(|| Dataset::new(dataset.columns().clone()))
            .add_row(row.clone());
    }
    

    return map;

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