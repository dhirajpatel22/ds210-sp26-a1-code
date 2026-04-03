use analytics_lib::{dataset::Dataset, query::Query};
use analytics_lib::{solution::compute_query_on_dataset}; //my addition - mistake?

pub fn hello() -> String {
    println!("hello called");
    return String::from("hello");
}

pub fn slow_rpc(input_dataset: &Dataset) -> Dataset {
    println!("slow_rpc called");
    return input_dataset.clone(); //should it stay this way????
    
}

pub fn fast_rpc(input_dataset: &Dataset, query: Query) -> Dataset {
    let queried_dataset = compute_query_on_dataset(input_dataset, &query);
    return queried_dataset;
}