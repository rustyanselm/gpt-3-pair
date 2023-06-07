extern crate csv;

use std::error::Error;
use std::fs::File;
use std::vec::Vec;

fn main() {
    // Specify the path to your CSV file
    let file_path_1 = "C:/Users/ansel/code/gpt-3-pair/project_002/file_1.csv";
	let file_path_2 = "C:/Users/ansel/code/gpt-3-pair/project_002/file_2.csv";

    // Call the function to load the CSV file into a vector
    let data_1 = match load_csv(file_path_1) {
        Ok(data_1) => data_1,
        Err(err) => {
            eprintln!("Error loading CSV file: {}", err);
            return;
        }
    };

    // Print the loaded data
    for row in &data_1 {
        println!("{:?}", row);
    }
	
    let data_2 = match load_csv(file_path_2) {
        Ok(data_2) => data_2,
        Err(err) => {
            eprintln!("Error loading CSV file: {}", err);
            return;
        }
    };

    // Print the loaded data
    for row in &data_2 {
        println!("{:?}", row);
    }

let data_3 = aggregate_data(&data_1, &data_2);

    // Print the aggregated data
    for row in &data_3 {
        println!("{:?}", row);
	}
}

fn aggregate_data(data_1: &Vec<Vec<String>>, data_2: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut aggregated_data: Vec<Vec<String>> = Vec::new();

    // Aggregate data from data_1
    for row in data_1 {
        let name = &row[0];
        let amount = &row[2].parse::<f64>().unwrap();

        let existing_row = aggregated_data.iter_mut().find(|r| &r[0] == name);
        if let Some(row) = existing_row {
            let current_amount = &row[1].parse::<f64>().unwrap();
            row[1] = (current_amount + amount).to_string();
        } else {
            aggregated_data.push(vec![name.to_string(), amount.to_string()]);
        }
    }

    // Aggregate data from data_2
    for row in data_2 {
        let name = &row[0];
        let amount = &row[2].parse::<f64>().unwrap();

        let existing_row = aggregated_data.iter_mut().find(|r| &r[0] == name);
        if let Some(row) = existing_row {
            let current_amount = &row[1].parse::<f64>().unwrap();
            row[1] = (current_amount + amount).to_string();
        } else {
            aggregated_data.push(vec![name.to_string(), amount.to_string()]);
        }
    }

    aggregated_data
}

fn load_csv(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(file_path)?;

    // Create a CSV reader
    let mut reader = csv::Reader::from_reader(file);

    // Read the CSV records into a vector
    let mut data = Vec::new();
    for result in reader.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        data.push(row);
    }

    Ok(data)
}

fn print_data(data: &Vec<Vec<String>>) {
    // Print the loaded data
    for row in data {
        println!("{:?}", row);
    }
}
