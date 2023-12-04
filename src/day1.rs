use regex::Regex;

fn get_value_for_line(line: String) -> u32 {
    let re = Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut line = line;
    let mut numbers = Vec::<u32>::new();
    while !line.is_empty() {
        if let Some(mat) = re.find(&line) {
            let local_mat: &str = mat.as_str();
            match local_mat {
                "zero" => numbers.push(0),
                "one" => numbers.push(1),
                "two" => numbers.push(2),
                "three" => numbers.push(3),
                "four" => numbers.push(4),
                "five" => numbers.push(5),
                "six" => numbers.push(6),
                "seven" => numbers.push(7),
                "eight" => numbers.push(8),
                "nine" => numbers.push(9),
                "0" => numbers.push(0),
                "1" => numbers.push(1),
                "2" => numbers.push(2),
                "3" => numbers.push(3),
                "4" => numbers.push(4),
                "5" => numbers.push(5),
                "6" => numbers.push(6),
                "7" => numbers.push(7),
                "8" => numbers.push(8),
                "9" => numbers.push(9),
                _ => ()
            }
        }
        line.drain(..1);
    }
    return (numbers.first().unwrap()*10) + numbers.last().unwrap();
}

pub fn get_calibration_value(contents: Vec<String>) -> u32 {
    let mut calibration = 0;
    for line in contents.iter() {
        calibration += get_value_for_line(line.to_string());
    }
    return calibration;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1_part1_example() {
        let example = vec!["1abc2".to_string(),"pqr3stu8vwx".to_string(),"a1b2c3d4e5f".to_string(),"treb7uchet".to_string()];
        let calibration_num = get_calibration_value(example);
        assert_eq!(calibration_num, 142)
    }

    #[test]
    fn day1_part2_example() {
        let example = vec!["two1nine".to_string(),"eightwothree".to_string(),"abcone2threexyz".to_string(),"xtwone3four".to_string(), "4nineeightseven2".to_string(), "zoneight234".to_string(), "7pqrstsixteen".to_string()];
        let calibration_num = get_calibration_value(example);
        assert_eq!(calibration_num, 281)
    }

    #[test]
    fn test_get_value_for_line() {
        assert_eq!(get_value_for_line("zero2one".to_string()), 1);
        assert_eq!(get_value_for_line("one2zero".to_string()), 10);
        assert_eq!(get_value_for_line("two1three".to_string()), 23);
        assert_eq!(get_value_for_line("three1two".to_string()), 32);
        assert_eq!(get_value_for_line("four1five".to_string()), 45);
        assert_eq!(get_value_for_line("five1four".to_string()), 54);
        assert_eq!(get_value_for_line("six1seven".to_string()), 67);
        assert_eq!(get_value_for_line("seven1six".to_string()), 76);
        assert_eq!(get_value_for_line("eight1nine".to_string()), 89);
        assert_eq!(get_value_for_line("nine1eight".to_string()), 98);
        assert_eq!(get_value_for_line("zero".to_string()), 0);
        assert_eq!(get_value_for_line("one".to_string()), 11);
        assert_eq!(get_value_for_line("two".to_string()), 22);
        assert_eq!(get_value_for_line("three".to_string()), 33);
        assert_eq!(get_value_for_line("four".to_string()), 44);
        assert_eq!(get_value_for_line("five".to_string()), 55);
        assert_eq!(get_value_for_line("six".to_string()), 66);
        assert_eq!(get_value_for_line("seven".to_string()), 77);
        assert_eq!(get_value_for_line("eight".to_string()), 88);
        assert_eq!(get_value_for_line("nine".to_string()), 99);
        assert_eq!(get_value_for_line("0two1".to_string()), 1);
        assert_eq!(get_value_for_line("1two0".to_string()), 10);
        assert_eq!(get_value_for_line("2one3".to_string()), 23);
        assert_eq!(get_value_for_line("3one2".to_string()), 32);
        assert_eq!(get_value_for_line("4one5".to_string()), 45);
        assert_eq!(get_value_for_line("5one4".to_string()), 54);
        assert_eq!(get_value_for_line("6one7".to_string()), 67);
        assert_eq!(get_value_for_line("7one6".to_string()), 76);
        assert_eq!(get_value_for_line("8one9".to_string()), 89);
        assert_eq!(get_value_for_line("9one8".to_string()), 98);
        assert_eq!(get_value_for_line("0".to_string()), 0);
        assert_eq!(get_value_for_line("1".to_string()), 11);
        assert_eq!(get_value_for_line("2".to_string()), 22);
        assert_eq!(get_value_for_line("3".to_string()), 33);
        assert_eq!(get_value_for_line("4".to_string()), 44);
        assert_eq!(get_value_for_line("5".to_string()), 55);
        assert_eq!(get_value_for_line("6".to_string()), 66);
        assert_eq!(get_value_for_line("7".to_string()), 77);
        assert_eq!(get_value_for_line("8".to_string()), 88);
        assert_eq!(get_value_for_line("9".to_string()), 99);
    }
}