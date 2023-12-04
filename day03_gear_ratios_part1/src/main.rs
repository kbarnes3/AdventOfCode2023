#![allow(unused_imports)]
use day03_gear_ratios_common::{REAL_DATA, SAMPLE_DATA};

fn main() {
    let result = do_work(REAL_DATA);
    println!("{}", result);
}

struct PartNumber {
    part_number: u64,
    row: usize,
    start_col: usize,
    end_col: usize,
}

struct Symbol {
    row: usize,
    col: usize,
}

struct Schematic {
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

fn do_work<const X: usize, const Y: usize>(data: [[char; Y]; X]) -> u64 {
    let schematic: Schematic = parse_schematic(data);
    let mut result: u64 = 0;
    for part_number in schematic.part_numbers {
        if is_part_number_valid(&part_number, &schematic.symbols, X, Y) {
            result += part_number.part_number;
        }
    }

    result
}

fn parse_schematic<const X: usize, const Y: usize>(data: [[char; Y]; X]) -> Schematic {
    let mut part_numbers = Vec::new();
    let mut symbols = Vec::new();

    for (row_number, row) in data.iter().enumerate() {
        let mut start_col: Option<usize> = None;
        let mut end_col: Option<usize> = None;
        let mut part_number = 0;
        let mut in_part_number = false;
        for (col_number, value) in row.iter().enumerate() {
            if value.is_ascii_digit() {
                if !in_part_number {
                    start_col = Some(col_number);
                    in_part_number = true;
                }
                end_col = Some(col_number);
                part_number *= 10;
                part_number += value.to_digit(10).unwrap() as u64;
            } else {
                if in_part_number {
                    part_numbers.push(PartNumber {
                        part_number,
                        row: row_number,
                        start_col: start_col.unwrap(),
                        end_col: end_col.unwrap(),
                    });
                    in_part_number = false;
                    start_col = None;
                    end_col = None;
                    part_number = 0;
                }

                if *value != '.' {
                    symbols.push(Symbol {
                        row: row_number,
                        col: col_number,
                    });
                }
            }
        }
        if in_part_number {
            part_numbers.push(PartNumber {
                part_number,
                row: row_number,
                start_col: start_col.unwrap(),
                end_col: end_col.unwrap(),
            });
        }
    }

    Schematic {
        part_numbers,
        symbols,
    }
}

fn is_part_number_valid(
    part_number: &PartNumber,
    symbols: &Vec<Symbol>,
    row_count: usize,
    col_count: usize,
) -> bool {
    let min_row: usize = if part_number.row == 0 {
        0
    } else {
        part_number.row - 1
    };

    let max_row: usize = if part_number.row == row_count - 1 {
        part_number.row
    } else {
        part_number.row + 1
    };
    let min_col: usize = if part_number.start_col == 0 {
        0
    } else {
        part_number.start_col - 1
    };

    let max_col: usize = if part_number.end_col == col_count - 1 {
        part_number.end_col
    } else {
        part_number.end_col + 1
    };

    for symbol in symbols {
        if symbol.row >= min_row
            && symbol.row <= max_row
            && symbol.col >= min_col
            && symbol.col <= max_col
        {
            return true;
        }
    }

    false
}
