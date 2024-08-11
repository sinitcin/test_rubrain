use std::{collections::HashMap, path::PathBuf, vec};

use crate::domains::calibration_documents::interfaces::IRepository;

use super::CalibrationServiceErrors;

pub struct CalibrationService {
    repository: Box<dyn IRepository>,
}

impl CalibrationService {
    pub fn new(repository: Box<dyn IRepository>) -> Self {
        Self { repository }
    }

    pub fn calculate_total_value_from_file(
        &self,
        path: PathBuf,
    ) -> Result<u32, CalibrationServiceErrors> {
        let lines = self.repository.read_data_from_file(path)?;
        self.calculate_total_value_from_lines(lines)
    }

    pub fn calculate_total_value_from_lines(
        &self,
        lines: Vec<String>,
    ) -> Result<u32, CalibrationServiceErrors> {
        let text_digits = Self::get_text_digits_map();
        Ok(lines
            .iter()
            .filter_map(|line| Self::find_calibration_value(line, &text_digits))
            .sum())
    }

    fn get_text_digits_map() -> HashMap<&'static str, u8> {
        vec![
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
        .into_iter()
        .collect()
    }

    // Поиск "цифр" в строке используя Sliding Window Algorithm
    // Search for "digits" in the line using the Sliding Window Algorithm
    fn find_calibration_value(line: &str, text_digits: &HashMap<&str, u8>) -> Option<u32> {
        // Переменные для хранения первой и последней найденной "цифры"
        // Variables to store the first and last found "digits"
        let mut first_digit = None;
        let mut last_digit = None;

        let mut start = 0;
        let mut end;

        // Итерация по всем подстрокам в строке для поиска "цифр"
        // Iterating over all substrings in the line to search for "digits"
        while start < line.len() {
            end = start + 1;
            while end <= line.len() {
                let token = &line[start..end];

                // Проверяем, является ли подстрока ("цифрой" в текстовом виде)
                // Check if the substring is a textual representation of a "digit"
                if let Some(&digit) = text_digits.get(token) {
                    // Если это первая "цифра", сохраняем её
                    // If it's the first "digit", store it
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    }
                    // Обновляем последнюю "цифру" в любом случае
                    // Update the last "digit" in any case
                    last_digit = Some(digit as u32);
                } else
                // Проверяем, является ли подстрока числом
                // Check if the substring is a numeric digit
                if let Ok(digit) = token.parse::<u32>() {
                    // Если это первая "цифра", сохраняем её
                    // If it's the first "digit", store it
                    if first_digit.is_none() {
                        first_digit = Some(digit as u8);
                    }
                    // Обновляем последнюю "цифру" в любом случае
                    // Update the last "digit" in any case
                    last_digit = Some(digit);
                }
                end += 1;
            }
            start += 1;
        }
        // Если найдена хотя бы одна "цифра", возвращаем комбинацию первой и последней цифры
        // If we found at least one "digit", return the combination of the first and last digit
        first_digit.map(|fd| (fd as u32) * 10 + last_digit.unwrap_or(0))
    }
}
