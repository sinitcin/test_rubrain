# Тестовое задание RuBrain от 11.08.2024

## Описание решения

Основной алгоритм работы был построен на основе: Sliding Window Algorithm.
Временная сложность - O(n), сложность по памяти - O(1).

Детально описано в src/domains/calibration_documents/service.rs
См. функцию: `find_calibration_value`

Так же сознательно решение было переусложнено, чтобы показать возможность использования DDD и Clean Architecture.

## Тестирование и запуск

*Разработка и тестирование производились в ОС Linux, не гарантирована возможность работы на других ОС!*

Запустить модульные тесты можно командой: `cargo test`

Для запуска функциональных тестов: `./functional_test.sh`

Для работы с произвольным файлом достаточно выполнить в терминале:

```bash
cargo build --release
./target/release/test_for_job functional_tests/test_data/test_1.txt 
```

[В случае возникновения вопросов](mailto:antonsinitsyn@outlook.de).
