pub struct Solution;

impl crate::Day for Solution {
    fn first_star(input: &str) -> String {
        let mut total = 0;

        let mut v_index = 0;

        let mut number = (0, String::new());

        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        numbers.push(Vec::new());
        symbols.push(Vec::new());

        for c in input.chars() {
            if c.is_ascii_digit() {
                if number.1.is_empty() {
                    number.0 = v_index;
                }

                number.1.push_str(&c.to_string());

                v_index += 1;
            } else if c == '.' {
                if !number.1.is_empty() {
                    numbers[0].push(number.clone());
                }
                number.1 = "".to_string();

                v_index += 1;
            } else if c.is_whitespace() {
                if !number.1.is_empty() {
                    numbers[0].push(number.clone());
                }
                number.1 = "".to_string();

                numbers.insert(0, Vec::new());
                symbols.insert(0, Vec::new());

                v_index = 0;
            } else {
                if !number.1.is_empty() {
                    numbers[0].push(number.clone());
                }
                number.1 = "".to_string();

                symbols[0].push(v_index);

                v_index += 1;
            }
        }

        if !number.1.is_empty() {
            numbers[0].push(number.clone());
        }

        for (h_index, numbers_in_line) in numbers.iter().enumerate() {
            let mut interesting_symbols = Vec::new();
            if h_index as i32 - 1 > -1 {
                interesting_symbols.append(&mut symbols[h_index - 1].clone());
            }

            interesting_symbols.append(&mut symbols[h_index].clone());

            if h_index + 1 < symbols.len() {
                interesting_symbols.append(&mut symbols[h_index + 1].clone());
            }

            for number in numbers_in_line {
                let min_index = number.0 - 1;
                let max_index = number.0 + number.1.len() as i32;

                for &symbol in &interesting_symbols {
                    if symbol >= min_index && symbol <= max_index {
                        total += number.1.parse::<i32>().expect("Should be a number");
                        break;
                    }
                }
            }
        }

        total.to_string()
    }

    fn second_star(input: &str) -> String {
        let mut total = 0;

        let mut v_index = 0;

        let mut number = (0, String::new());

        let mut numbers = Vec::new();
        let mut gears = Vec::new();

        numbers.push(Vec::new());
        gears.push(Vec::new());

        for c in input.chars() {
            if c.is_ascii_digit() {
                if number.1.is_empty() {
                    number.0 = v_index;
                }

                number.1.push_str(&c.to_string());

                v_index += 1;
            } else if c == '.' {
                if !number.1.is_empty() {
                    numbers[0].push(number.clone());
                }
                number.1 = "".to_string();

                v_index += 1;
            } else if c.is_whitespace() {
                if !number.1.is_empty() {
                    numbers[0].push(number.clone());
                }
                number.1 = "".to_string();

                numbers.insert(0, Vec::new());
                gears.insert(0, Vec::new());

                v_index = 0;
            } else {
                if !number.1.is_empty() {
                    numbers[0].push(number.clone());
                }
                number.1 = "".to_string();

                if c == '*' {
                    gears[0].push(v_index);
                }

                v_index += 1;
            }
        }

        if !number.1.is_empty() {
            numbers[0].push(number.clone());
        }

        for (h_index, gears_in_line) in gears.iter().enumerate() {
            let mut interesting_numbers = Vec::new();
            if h_index as i32 - 1 > -1 {
                interesting_numbers.append(&mut numbers[h_index - 1].clone());
            }

            interesting_numbers.append(&mut numbers[h_index].clone());

            if h_index + 1 < gears.len() {
                interesting_numbers.append(&mut numbers[h_index + 1].clone());
            }

            for &gear in gears_in_line {
                let mut specific_numbers = Vec::new();
                for number in &interesting_numbers {
                    let min_index = number.0 - 1;
                    let max_index = number.0 + number.1.len() as i32;

                    if gear >= min_index && gear <= max_index {
                        specific_numbers.push(number.1.parse::<i32>().expect("Should be a number"));
                    }
                }

                if specific_numbers.len() == 2 {
                    total += specific_numbers[0] * specific_numbers[1];
                }
            }
        }

        total.to_string()
    }
}

mod test {
    #[allow(dead_code)]
    const INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    #[test]
    fn first_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::first_star("1.$."), "0");
        assert_eq!(<Solution as crate::Day>::first_star(INPUT), "4361");
    }

    #[test]
    fn second_star() {
        use super::*;
        assert_eq!(<Solution as crate::Day>::second_star("2*3"), "6");
        assert_eq!(<Solution as crate::Day>::second_star(INPUT), "467835");
    }
}
