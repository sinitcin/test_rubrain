use crate::domains::calibration_documents::CalibrationService;

use super::IRepository;

pub struct MocRepository {}

impl IRepository for MocRepository {
    fn read_data_from_file(
        &self,
        _path: std::path::PathBuf,
    ) -> Result<Vec<String>, std::io::Error> {
        todo!()
    }
}

#[test]
fn test_calculate_total_value_part_one() {
    let repository = Box::new(MocRepository {});
    let calibration_service = CalibrationService::new(repository);
    let lines = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
        .into_iter()
        .map(String::from)
        .collect();
    let result = calibration_service
        .calculate_total_value_from_lines(lines)
        .unwrap();
    assert_eq!(result, 142);
}

#[test]
fn test_calculate_total_value_part_two() {
    let repository = Box::new(MocRepository {});
    let calibration_service = CalibrationService::new(repository);
    let lines = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    let result = calibration_service
        .calculate_total_value_from_lines(lines)
        .unwrap();
    assert_eq!(result, 281);
}
