use std::{path::PathBuf, time::Duration};

use log::debug;

use crate::meta_data::MetaDataType;

const FILE_EXTENSION: &str = "_akt.zip";

/// Download wind date from the DWD for the specific station id
pub async fn download_data_for(id: usize, data_type: MetaDataType) -> Result<String, String> {
    let url = format! {"{}{:05}{}", data_type.to_access_url(), id, FILE_EXTENSION};
    debug!(
        "Requested {:?} for {} - Requesting URL: {}",
        data_type, id, url
    );

    // Check if the file already exists
    let file_path = format!("{}/{}/data.csv", data_type.to_string(), id);
    if std::path::Path::new(&file_path).exists() {
        debug!("File already exists for station id: {}", id);
        return Ok(format!(
            "File already exists for station id: {}",
            id
        ));
    }

    let response = reqwest::get(&url).await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let data_type_str = data_type.to_string();
                // Create the directory if it doesn't exist
                if !std::path::Path::new(data_type_str.as_str()).exists() {
                    std::fs::create_dir(data_type_str.as_str())
                        .expect("Failed to create directory");
                }
                let mut content =
                    std::io::Cursor::new(resp.bytes().await.expect("Failed to read response"));
                let path = PathBuf::from(format!("{}/{}", data_type_str.as_str(), id));

                match zip_extract::extract(&mut content, &path, true) {
                    Ok(_) => {
                        debug!("File downloaded successfully for station id: {}", id);
                        tokio::time::sleep(Duration::from_secs(1)).await;

                        // Check if a file was created within the directory
                        for entry in std::fs::read_dir(path).expect("Failed to read directory") {
                            let entry = entry.expect("Failed to read entry");
                            if entry.path().is_file() {
                                // Rename the file to data.csv
                                let new_path = entry.path().clone().with_file_name("data.csv");
                                debug!("Renaming {}", entry.path().to_string_lossy());
                                std::fs::rename(entry.path(), &new_path)
                                    .expect("Failed to rename file");
                                return Ok(format!(
                                    "File downloaded successfully for station id: {}",
                                    id
                                ));
                            }
                        }
                        Err(format!(
                            "No file created in the directory for station id: {}",
                            id
                        ))
                    }
                    Err(e) => return Err(format!("Failed to extract zip file: {}", e)),
                }
            } else {
                Err(format!(
                    "Failed to download file for station id: {}. Status: {}",
                    id,
                    resp.status()
                ))
            }
        }
        Err(e) => Err(format!("Error occurred while making request: {}", e)),
    }
}

/// Reads text from a URL
pub async fn read_text_from_url(url: &str) -> Result<String, String> {
    let response = reqwest::get(url).await;
    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let text = resp.text().await.expect("Failed to read response");
                Ok(text)
            } else {
                Err(format!(
                    "Failed to download file. Status: {}",
                    resp.status()
                ))
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
        let file_path = format!("{}/{}.zip", MetaDataType::Wind.to_string(), 11);
        if std::path::Path::new(&file_path).exists() {
            std::fs::remove_file(&file_path).expect("Failed to delete test file");
        }

        let result = download_data_for(11, MetaDataType::Wind).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "File downloaded successfully for station id: 11"
        );

        // Check whether the file exists
        assert!(
            std::path::Path::new(&file_path).exists(),
            "File was not created"
        );

        // Clean up the created file after the test
        std::fs::remove_file(file_path).expect("Failed to delete test file");
    }
}
