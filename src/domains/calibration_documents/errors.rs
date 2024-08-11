#[derive(Debug, thiserror::Error)]
pub enum CalibrationServiceErrors {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] std::io::Error),
}
