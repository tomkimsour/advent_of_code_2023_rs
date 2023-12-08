advent_of_code::solution!(2);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let map = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    let mut sum : u32 = 0;

    for (i,line) in input.lines().enumerate() {
        // splits sets
        let mut is_valid = true;
        for set in  line.split_terminator(&[';',':'][..]).filter(|&c| !c.starts_with("Game")).collect::<Vec<&str>>(){
            println!("{}", set);
            for splited_line in set.split(",").collect::<Vec<&str>>(){
                let color = splited_line.split_whitespace().collect::<Vec<&str>>();
                println!("{} < {}",*map.get(color[1]).as_deref().unwrap(), splited_line);
                if *map.get(color[1]).as_deref().unwrap() < (color[0].parse::<i32>().unwrap()){
                    is_valid = false;
                }
            }
        }
        if is_valid {
            sum += u32::try_from(i + 1).unwrap();
            println!("{}",sum);
        }

    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum : u32 = 0;

    for line in input.lines() {
        let mut min_map: HashMap<&str, u32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);
        // splits sets
        for set in  line.split_terminator(&[';',':'][..]).filter(|&c| !c.starts_with("Game")).collect::<Vec<&str>>(){
            println!("{}", set);
            for splited_line in set.split(",").collect::<Vec<&str>>(){
                let color = splited_line.split_whitespace().collect::<Vec<&str>>();
                let text_value = color[0].parse::<u32>().unwrap();
                if let Some(x) = min_map.get_mut(&color[1]) {
                    *x = std::cmp::max(*x, text_value);
                }
            }
        }
        println!("{},{},{}",min_map.get("red").unwrap() , min_map.get("green").unwrap() , min_map.get("blue").unwrap());
        sum += min_map.get("red").unwrap() * min_map.get("green").unwrap() * min_map.get("blue").unwrap();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
