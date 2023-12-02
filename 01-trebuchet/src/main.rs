fn main() {
    // Puzzle part one
    calculate_total_using_only_nums();

    // Puzzle part two
    calculate_total();
}

fn calculate_total() {
    let data = std::fs::read_to_string("lines")
        .expect("no file found");
    let arr_of_nums: Vec<Vec<usize>> = data
        .lines()
        .into_iter()
        .map(|val| {
            let mut nums: Vec<usize> = vec![];
            let mut idx = 0;
            while idx < val.len() {
                let diff = val.len() - idx;
                if diff >= 3 {
                    let window = &val[idx..idx + 3];
                    match window {
                        "one" => nums.push(1),
                        "two" => nums.push(2),
                        "six" => nums.push(6),
                        _ => {}
                    }
                }

                if diff >= 4 {
                    let window = &val[idx..idx + 4];
                    match window {
                        "four" => nums.push(4),
                        "five" => nums.push(5),
                        "nine" => nums.push(9),
                        _ => {}
                    }
                }

                if diff >= 5 {
                    let window = &val[idx..idx + 5];
                    match window {
                        "three" => nums.push(3),
                        "seven" => nums.push(7),
                        "eight" => nums.push(8),
                        _ => {}
                    }
                }

                let maybe_char = val.chars().nth(idx);
                if let Some(char) = maybe_char {
                    match char {
                        '1' => nums.push(1),
                        '2' => nums.push(2),
                        '3' => nums.push(3),
                        '4' => nums.push(4),
                        '5' => nums.push(5),
                        '6' => nums.push(6),
                        '7' => nums.push(7),
                        '8' => nums.push(8),
                        '9' => nums.push(9),
                        _ => {}
                    }
                }
                idx += 1;
            }

            return nums
        })
        .collect();

    let result: usize = arr_of_nums.iter()
        .map(|x| {
            println!("{:?}", x);
            let y: usize = *x.first().unwrap();
            let z: usize = *x.last().unwrap();
            return format!("{}{}", y, z).parse::<usize>().unwrap();
        })
        .sum();

    println!("total with words is: {}", result);
}

fn calculate_total_using_only_nums() {
    let data = std::fs::read_to_string("lines")
        .expect("no file found");
    let arr_of_chars = data
        .lines()
        .map(|x| x.chars());

    let mut vals: Vec<String> = vec![];
    for arr in arr_of_chars {
        let mut first_num: char = '-';
        let mut last_num: char = '-';

        for c in arr {
            if c.is_numeric() {
                if first_num == '-' {
                    first_num = c
                }

                last_num = c
            }
        }

        vals.push(format!("{}{}", first_num, last_num))
    }

    let mut total: usize = 0;
    for val in vals {
        let num = str::parse::<usize>(&*val);
        if let Ok(n) = num {
            total += n
        }
    }

    println!("Final total was {}", total);
}
