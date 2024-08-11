use std::path::PathBuf;

use crate::{
    domains::calibration_documents::CalibrationService, infrastructure::errors::AppErrors,
};

pub fn uc_calculate_total_value(
    calibration_service: &CalibrationService,
    data_file_path: PathBuf,
) -> Result<u32, AppErrors> {
    Ok(calibration_service.calculate_total_value_from_file(data_file_path)?)
}
