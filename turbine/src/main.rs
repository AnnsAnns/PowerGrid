mod turbine;
mod ftp_access;
mod meta_data;
mod parsing;

#[tokio::main]
async fn main() {
    println!("Parsing example csv");
    println!("{:?}", ftp_access::download_data_for(44, meta_data::MetaDataType::Wind).await);
}
