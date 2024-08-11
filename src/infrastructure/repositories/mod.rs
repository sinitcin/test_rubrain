use std::path::PathBuf;

use crate::domains::calibration_documents::IRepository;

pub struct FileReaderRepository;

impl IRepository for FileReaderRepository {
    fn read_data_from_file(&self, data_file_path: PathBuf) -> Result<Vec<String>, std::io::Error> {
        Ok(std::fs::read_to_string(data_file_path)?
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
    }
}
