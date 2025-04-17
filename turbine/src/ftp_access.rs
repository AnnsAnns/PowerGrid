const REQUEST_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/wind/now/10minutenwerte_wind_";
const FILE_EXTENSION: &str = "_now.zip";

/// Download wind date from the DWD for the specific station id
pub async fn download_wind_date_for(id: usize) -> Result<String, String> {
    let url = format!{"{}{:05}{}", REQUEST_URL, id, FILE_EXTENSION};
    println!("Requesting URL: {}", url);
    let response = reqwest::get(&url).await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // Create the directory if it doesn't exist
                if !std::path::Path::new("wind_data").exists() {
                    std::fs::create_dir("wind_data").expect("Failed to create directory");
                }
                let mut file = std::fs::File::create(format!("wind_data/{}.zip", id)).expect("Failed to create file");
                let mut content = std::io::Cursor::new(resp.bytes().await.expect("Failed to read response"));
                std::io::copy(&mut content, &mut file).expect("Failed to write to file");
                println!("File downloaded successfully for station id: {}", id);
                Ok(format!("File downloaded successfully for station id: {}", id))
            } else {
                Err(format!("Failed to download file for station id: {}. Status: {}", id, resp.status()))
            }
        }
        Err(e) => Err(format!("Error occurred while making request: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_wind_date_for() {
        // Make sure the file does not exist before the test
        let file_path = format!("wind_data/{}.zip", 11);
        if std::path::Path::new(&file_path).exists() {
            std::fs::remove_file(&file_path).expect("Failed to delete test file");
        }

        let result = download_wind_date_for(11).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "File downloaded successfully for station id: 11");
        
        // Check whether the file exists
        let file_path = format!("wind_data/{}.zip", 11);
        assert!(std::path::Path::new(&file_path).exists(), "File was not created");

        // Clean up the created file after the test
        std::fs::remove_file(file_path).expect("Failed to delete test file");
    }
}