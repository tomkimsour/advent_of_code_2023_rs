advent_of_code::solution!(3);
use std::{cell::Cell, collections::btree_map::Range};
use derive_more::{Add, Display, From};


use itertools::Itertools;

fn check_neighbour(grid: &Vec<Vec<Cell<char>>>, row: usize, col: usize) -> bool{
    let row_len = grid.len();
    let col_len = grid[0].len();
    if 0 < row {
        if !grid[row -1][col].get().is_ascii_digit() && grid[row -1][col].get() != '.'{
            return true;
        }
    }
    if row_len > row + 1 {
        if !grid[row +1][col].get().is_ascii_digit() && grid[row+1][col].get() != '.'{
            return true;
        }
    }
    if col_len > col + 1 {
        if !grid[row][col+1].get().is_ascii_digit() && grid[row][col+1].get() != '.'{
            return true;
        }
    }
    if 0 < col {
        if !grid[row][col-1].get().is_ascii_digit() && grid[row][col-1].get() != '.'{
            return true;
        }
    }
    if row_len > row + 1 && col_len > col + 1 {
        if !grid[row+1][col+1].get().is_ascii_digit() && grid[row+1][col+1].get() != '.'{
            return true;
        }
    }
    if 0 < row && col_len > col + 1 {
        if !grid[row-1][col+1].get().is_ascii_digit() && grid[row-1][col+1].get() != '.'{
            return true;
        }
    }
    if 0 < row && 0 < col {
        if !grid[row-1][col-1].get().is_ascii_digit() && grid[row-1][col-1].get() != '.'{
            return true;
        }
    }
    if row_len > row + 1 && 0 < col {
        if !grid[row+1][col-1].get().is_ascii_digit() && grid[row+1][col-1].get() != '.'{
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let grid = input.lines().map(|line| line.chars().map(|x| Cell::new(x)).collect_vec()).collect_vec();
    // let mut start: usize = 0;
    for (row, line) in grid.iter().enumerate(){
        let mut number_buf = "".to_string();
        let mut has_neighbour = false;
        for (col,c) in line.iter().enumerate() {
            if c.get().is_numeric() {
                number_buf.push(c.get());
                if !has_neighbour {
                    has_neighbour = check_neighbour(&grid,row,col);
                }
            } else {
                if number_buf.is_empty() && has_neighbour{
                    panic!("An empty buffer should not have neighbours");
                }
                if !number_buf.is_empty() {
                    if has_neighbour {
                        sum += number_buf.parse::<u32>().unwrap();
                    }
                }
                number_buf.clear();
                has_neighbour = false;
            }
        }
        if number_buf.is_empty() && has_neighbour{
            panic!("An empty buffer should not have neighbours");
        }
        if !number_buf.is_empty() {
            if has_neighbour {
                sum += number_buf.parse::<u32>().unwrap();
            }
        }
        number_buf.clear();
    }
    Some(sum)
}

#[derive(PartialEq, From)]
struct Number2 {
    range: std::ops::Range<usize>,
    value: i32,
}

struct Numbers {
    numbers: Vec<Number2>,
}

impl Numbers {
    fn new()->Self{
        Numbers { numbers: Vec::<Number2>::new() }
    }
}
#[derive(PartialEq, From, Display)]
#[display("{:}, {:}",x,y)]
struct Gears {
    x: Vec<usize>,
    y: Vec<usize>,
}

impl Gears {
    fn new()->Self{
        Gears {
            x : Vec::<usize>::new(),
            y : Vec::<usize>::new(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
   let grid = input.lines().map(|x| {x.chars().collect_vec()}).collect_vec();
   let sum = {
        let mut gears:Gears = Gears::new();
        let mut numbers = Numbers::new();
        let mut reading_number = false;
        let mut number_buf = "".to_string();
        let mut beginin_index: usize = 0;
        for (row, line) in grid.into_iter().enumerate() {
            for (col, c) in line.into_iter().enumerate() {
                if c.is_ascii_digit() {
                    if reading_number == false{
                        beginin_index = col;
                        reading_number = true;
                    }
                    number_buf.push(c);
                }
                else if c == '*' {
                    gears.x.push( row);
                    gears.y.push( col);
                }
                if reading_number {
                    let n = Number2{
                        range: beginin_index..col,
                        value: number_buf.parse::<i32>().unwrap(),
                    };
                    numbers.numbers.push(n);
                    reading_number = false;
                }
            }
        }
        println!("{:}",gears);
        gears
    };

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result,Some(467835));
    }
}
