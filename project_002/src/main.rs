extern crate csv;

use std::error::Error;
use std::fs::File;
use std::vec::Vec;
use std::io::Write;

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
		println!("{:?}", row[0]);
		println!("{:?}", row[1]);
    if let Some(number) = parse_f32(&row[1]) {
        println!("Parsed f32: {}", number);
    } else {
        println!("Failed to parse the string as an f32.");
    }
	}
	
	
	
    if let Err(err) = generate_histogram_plot(data_3) {
        eprintln!("Error: {:?}", err);
    } else {
        println!("Histogram plot generated successfully!");
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

fn generate_histogram_plot(data: Vec<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    // Separate the names and amounts into separate vectors
    let mut names: Vec<String> = Vec::new();
    let mut amounts: Vec<f32> = Vec::new();

    for row in &data {
        names.push(row[0].clone());
        if let Some(number) = parse_f32(&row[1]) {
            amounts.push(number);
            println!("howdy do {}", number);
        } else {
            println!("Failed to parse the string as an f32.");
        }
    }

    // Generate the JavaScript code for the chart using Chart.js
    let chart_js_code = format!(
        r#"
        <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
        <canvas id="chart"></canvas>
        <script>
            var ctx = document.getElementById('chart').getContext('2d');
            var data = {{
                labels: {:?},
                datasets: [{{
                    label: 'Amounts',
                    data: {:?},
                    backgroundColor: 'rgba(75, 192, 192, 0.6)',
                    borderColor: 'rgba(75, 192, 192, 1)',
                    borderWidth: 1
                }}]
            }};
            var options = {{
                responsive: true,
                scales: {{
                    x: {{
                        title: {{
                            display: true,
                            text: 'Names'
                        }}
                    }},
                    y: {{
                        title: {{
                            display: true,
                            text: 'Amounts'
                        }},
                        ticks: {{
                            beginAtZero: true,
                            suggestedMax: 1500
                        }}
                    }}
                }}
            }};
            new Chart(ctx, {{
                type: 'bar',
                data: data,
                options: options
            }});
        </script>
        "#,
        names, amounts
    );

    // Generate the HTML content
    let html_content = format!(
        r#"
        <html>
            <head>
                <title>Histogram of Names vs Amounts</title>
            </head>
            <body>
                {}
            </body>
        </html>
        "#,
        chart_js_code
    );

    // Create and write the HTML file
    let mut file = File::create("histogram.html")?;
    file.write_all(html_content.as_bytes())?;

    Ok(())
}

fn parse_f32(string: &str) -> Option<f32> {
    match string.parse::<f32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}