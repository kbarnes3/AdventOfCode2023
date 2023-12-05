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
    is_potential_gear: bool,
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

    for symbol in schematic.symbols {
        if symbol.is_potential_gear {
            let gear_part_numbers = get_gear_part_numbers(&symbol, &schematic.part_numbers, X, Y);
            if gear_part_numbers.is_some() {
                let (first_part_number, second_part_number) = gear_part_numbers.unwrap();
                result += first_part_number.part_number * second_part_number.part_number;
            }
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
                        is_potential_gear: *value == '*',
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

fn get_gear_part_numbers<'a>(
    symbol: &Symbol,
    parts: &'a Vec<PartNumber>,
    row_count: usize,
    col_count: usize,
) -> Option<(&'a PartNumber, &'a PartNumber)> {
    let mut first_part_number: Option<&'a PartNumber> = None;
    let mut second_part_number: Option<&'a PartNumber> = None;

    let min_row: usize = if symbol.row == 0 {
        symbol.row
    } else {
        symbol.row - 1
    };

    let max_row: usize = if symbol.row == row_count - 1 {
        symbol.row
    } else {
        symbol.row + 1
    };

    let min_col: usize = if symbol.col == 0 {
        symbol.col
    } else {
        symbol.col - 1
    };

    let max_col: usize = if symbol.col == col_count - 1 {
        symbol.col
    } else {
        symbol.col + 1
    };

    for part in parts {
        if part.row >= min_row
            && part.row <= max_row
            && ((part.start_col <= min_col && part.end_col >= min_col)
                || (part.start_col >= min_col && part.start_col <= max_col)
                || (part.start_col >= min_col && part.end_col <= max_col))
        {
            if first_part_number.is_none() {
                first_part_number = Some(part);
            } else if second_part_number.is_none() {
                second_part_number = Some(part);
            } else {
                return None;
            }
        }
    }

    if let Some(first_part_number) = first_part_number {
        if let Some(second_part_number) = second_part_number {
            return Some((first_part_number, second_part_number));
        }
    }

    None
}
