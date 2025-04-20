use metadata::MetaDataType;

mod turbine;
mod ftp_access;
mod data_parser_structs;
mod metadata;

#[tokio::main]
async fn main() {
    println!("Parsing example csv");
    println!("{:?}", ftp_access::download_data_for(44, MetaDataType::Wind).await);
}
