use ftp_access::download_wind_date_for;

mod turbine;
mod ftp_access;
mod data_parser_structs;
mod metadata;

#[tokio::main]
async fn main() {
    println!("Parsing example csv");
    println!("{:?}", download_wind_date_for(44).await);

}
