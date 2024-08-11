use std::path::PathBuf;

pub trait IRepository {
    fn read_data_from_file(&self, data_file_path: PathBuf) -> Result<Vec<String>, std::io::Error>;
}
