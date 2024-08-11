use crate::domains::calibration_documents::CalibrationServiceErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppErrors {
    #[error(transparent)]
    CalibrationServiceErrors(#[from] CalibrationServiceErrors),
}
