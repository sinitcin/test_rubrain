pub mod application;
pub mod domains;
pub mod infrastructure;

use application::use_cases::uc_calculate_total_value;
use clap::Parser;
use domains::calibration_documents::CalibrationService;
use infrastructure::errors::AppErrors;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> Result<(), AppErrors> {
    let args = Cli::parse();

    let repository = Box::new(infrastructure::repositories::FileReaderRepository {});
    let calibration_service = CalibrationService::new(repository);

    println!(
        "{}",
        uc_calculate_total_value(&calibration_service, args.path)?
    );
    Ok(())
}
