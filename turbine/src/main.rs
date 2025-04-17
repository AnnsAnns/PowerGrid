mod turbine;
mod ftp_access;
mod data_parser;

fn main() {
    println!("Parsing example csv");
    let csv_path = "test_data/example_csv.csv".to_string();
    let parsed_data = data_parser::read_from_csv(csv_path)
        .expect("Failed to read CSV file");

    println!("Parsed data: {:?}", parsed_data[5].to_string());

}
