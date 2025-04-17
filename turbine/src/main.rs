use ftp_access::download_wind_date_for;

mod turbine;
mod ftp_access;
mod data_parser;

#[tokio::main]
async fn main() {
    println!("Parsing example csv");
    println!("{:?}", download_wind_date_for(11).await);

}
