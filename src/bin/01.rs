use advent_of_code::DisplayExt;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum: u32 = 0;
    for line in lines {
        let mut first_digit: Option<u32> = None;
        let mut second_digit: Option<u32> = None;
        for c in line.chars() {
            if c.is_numeric() {
                if first_digit.is_none() {
                    first_digit = c.to_digit(10);
                    second_digit = c.to_digit(10);
                } else {
                    second_digit = c.to_digit(10);
                }
            }
        }
        sum += first_digit.unwrap() * 10 + second_digit.unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let nums = vec![ "zero","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for mut line in input.lines(){
        let digits = line.digits();
        while !nums.iter().any(|&x| line.starts_with(x)) && !line.chars().next().unwrap().is_numeric(){
            line = &line[1..];
        }
        while !nums.iter().any(|&x| line.ends_with(x)) && !line.chars().last().unwrap().is_numeric(){
            line = &line[..line.len()-1];
        }
        let mut first_digit: Option<u32> = None;
        let mut second_digit: Option<u32> = None;
        for (i,&n) in nums.iter().enumerate() {
            if line.starts_with(n) {
                first_digit = Some(i as u32) ;
            }
            if line.ends_with(n) {
                second_digit = Some(i as u32) ;
            }
        }
        if line.chars().next().unwrap().is_numeric() {
            first_digit = Some(digits[0]);
        }
        if line.chars().last().unwrap().is_numeric() {
            second_digit = Some(digits[digits.len()-1])
        }
        println!("{}",line);
        sum += first_digit.unwrap() * 10 + second_digit.unwrap();
        println!("{}",sum);
    }
    
    // println!("{}", first_digit.unwrap() * 10 + second_digit.unwrap());
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let folder = "examples";
        let cwd = std::env::current_dir().unwrap();
        let filepath = cwd.join("data").join(folder).join(format!("01_1.txt"));
        let f = std::fs::read_to_string(filepath);
        let result = part_one(&f.expect("could not open input file"));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
