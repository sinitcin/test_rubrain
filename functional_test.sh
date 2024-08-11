#!/bin/bash

cargo build --release

# Путь к Rust-приложению
RUST_APP="./target/release/test_for_job"
# Аргументы Path
PATH_ARG1="./functional_tests/test_data/test_1.txt"
PATH_ARG2="./functional_tests/test_data/test_2.txt"
# Ожидаемое значение
EXPECTED_OUTPUT1=142
EXPECTED_OUTPUT2=281
# Определить цвета
RED='\033[0;31m'
NC='\033[0m' # Без цвета
UGreen='\033[4;32m'

# Запуск Rust-приложения с первым файлом
output1=$($RUST_APP "$PATH_ARG1")

# Проверка успешности выполнения приложения
if [ $? -ne 0 ]; then
  echo "Rust application failed on $PATH_ARG1"
  exit -1
fi

# Проверка, является ли вывод числом
if ! [[ "$output1" =~ ^-?[0-9]+([.][0-9]+)?$ ]]; then
  echo -e "${RED}[ERROR]${NC} Output from $PATH_ARG1 is not a valid number: $output1"
  exit -1
fi

# Сравнение вывода с ожидаемым значением
if [[ $(bc <<< "$output1 != $EXPECTED_OUTPUT1") -eq 1 ]]; then
  echo -e "${RED}[ERROR]${NC} Output from $PATH_ARG1 ($output1) does not match expected $EXPECTED_OUTPUT1"
  exit -1
fi

# Запуск Rust-приложения со вторым файлом
output2=$($RUST_APP "$PATH_ARG2")

# Проверка успешности выполнения приложения
if [ $? -ne 0 ]; then
  echo "Rust application failed on $PATH_ARG2"
  exit -1
fi

# Проверка, является ли вывод числом
if ! [[ "$output2" =~ ^-?[0-9]+([.][0-9]+)?$ ]]; then
  echo -e "${RED}[ERROR]${NC} Output from $PATH_ARG2 is not a valid number: $output2"
  exit -1
fi

# Сравнение вывода с ожидаемым значением
if [[ $(bc <<< "$output2 != $EXPECTED_OUTPUT2") -eq 1 ]]; then
  echo -e "${RED}[ERROR]${NC} Output from $PATH_ARG2 ($output2) does not match expected $EXPECTED_OUTPUT2"
  exit -1
fi

# Если все проверки пройдены
echo -e "${UGreen}[CONGRATULATIONS]${NC} Outputs $output1 from $PATH_ARG1 and $output2 from $PATH_ARG2 match expected $EXPECTED_OUTPUT1 and $EXPECTED_OUTPUT2"
exit 0
