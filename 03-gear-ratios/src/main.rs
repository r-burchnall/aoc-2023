fn main() {
    part_one();
    part_two();
}

fn part_two() {
    let data = std::fs::read_to_string("engine")
        .expect("there is no engine file");

    let mut found_numbers: Vec<NumPositions> = vec![];

    let total_lines = data.lines().count();

    data.lines().enumerate().for_each(|(y, line)| {
        let mut chars: String = String::new();
        let mut start: i32 = -1;
        line.chars().enumerate().for_each(|(idx, x)| {
            if x.is_digit(10) {
                if start == -1 {
                    start = idx as i32
                }
                chars.push(x);
            } else {
                if chars.len() > 0 {
                    found_numbers.push(NumPositions {
                        value: chars.parse().expect("discovered characters should be a number"),
                        start_pos: (start as usize, y),
                        end_pos: (idx - 1, y),
                    });
                    start = -1;
                    chars.clear()
                }
            }
        });

        if chars.len() > 0 {
            found_numbers.push(NumPositions {
                value: chars.parse().expect("discovered characters should be a number"),
                start_pos: (start as usize, y),
                end_pos: (line.len() - 1, y),
            });
            start = -1;
        }
    });

    let mut total = 0;
    data.lines().enumerate().for_each(|(y, line)| {
        let line_len = line.len();

        line.chars().enumerate().for_each(|(x, char)| {
            if char == '*' {
                let symbol = Symbol {
                    max_y_size: total_lines - 1,
                    max_x_size: line_len - 1,
                    pos: (x, y),
                };
                let adjacent = symbol.calculate_adjacency();
                let discovered_parts: Vec<&NumPositions> = found_numbers
                    .iter()
                    .filter(|&i| i.contains(&adjacent))
                    .collect();

                if discovered_parts.len() == 2 {
                    total += discovered_parts.get(0).expect("honestly mate, what the fuck are you playing at...").value * discovered_parts.get(1).expect("like seriously?").value;
                }
            }
        })
    });

    println!("Total of gear ratios: {}", total)
}

fn part_one() {
    let data = std::fs::read_to_string("engine")
        .expect("there is no engine file");

    let mut found_numbers: Vec<NumPositions> = vec![];

    let total_lines = data.lines().count();

    data.lines().enumerate().for_each(|(y, line)| {
        let mut chars: String = String::new();
        let mut start: i32 = -1;
        line.chars().enumerate().for_each(|(idx, x)| {
            if x.is_digit(10) {
                if start == -1 {
                    start = idx as i32
                }
                chars.push(x);
            } else {
                if chars.len() > 0 {
                    found_numbers.push(NumPositions {
                        value: chars.parse().expect("discovered characters should be a number"),
                        start_pos: (start as usize, y),
                        end_pos: (idx - 1, y),
                    });
                    start = -1;
                    chars.clear()
                }
            }
        });

        if chars.len() > 0 {
            found_numbers.push(NumPositions {
                value: chars.parse().expect("discovered characters should be a number"),
                start_pos: (start as usize, y),
                end_pos: (line.len() - 1, y),
            });
            start = -1;
        }
    });

    println!("{:?}", found_numbers);


    let mut total = 0;
    data.lines().enumerate().for_each(|(y, line)| {
        let line_len = line.len();

        line.chars().enumerate().for_each(|(x, char)| {
            if !char.is_digit(10) && char != '.' {
                let symbol = Symbol {
                    max_y_size: total_lines - 1,
                    max_x_size: line_len - 1,
                    pos: (x, y),
                };
                let adjacent = symbol.calculate_adjacency();
                let discovered_parts: Vec<&NumPositions> = found_numbers
                    .iter()
                    .filter(|&i| i.contains(&adjacent))
                    .collect();

                println!("adjacent: {:?}", adjacent);

                discovered_parts
                    .iter()
                    .for_each(|x| {
                        total += x.value;
                        println!("adding {}", x.value)
                        // found_numbers.remove(found_numbers.)
                    })
            }
        })
    });

    println!("Total of part numbers: {}", total)
}

#[derive(Debug)]
struct NumPositions {
    value: usize,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

impl NumPositions {
    fn contains(&self, coordinates: &Vec<(usize, usize)>) -> bool {
        let mut occupied_spaces: Vec<(usize, usize)> = vec![];
        let mut x = self.start_pos.0;
        while x <= self.end_pos.0 {
            occupied_spaces.push((x, self.start_pos.1));
            x += 1
        }

        let mut found = false;
        coordinates.iter().for_each(|pos| {
            if occupied_spaces.contains(pos) {
                found = true
            }
        });

        return found
    }
}

#[derive(Debug)]
struct Symbol {
    max_x_size: usize,
    max_y_size: usize,
    pos: (usize, usize)
}

impl Symbol {
    fn calculate_adjacency(&self) -> Vec<(usize, usize)> {
        let mut coordinates: Vec<(usize, usize)> = vec![];

        let x;
        let max_x;
        let y;
        let max_y;
        // Clamp X positions
        if self.pos.0 == 0 {
            x = 0;
        } else {
            x = self.pos.0 - 1
        }
        if self.pos.0 + 1 > self.max_x_size  {
            max_x = self.max_x_size
        } else {
            max_x = self.pos.0 + 1
        }

        // Clamp Y positions
        if self.pos.1 == 0 {
            y = 0;
        } else {
            y = self.pos.1 - 1
        }
        if self.pos.1 + 1 > self.max_y_size  {
            max_y = self.max_y_size
        } else {
            max_y = self.pos.1 + 1
        }


        for i in x..=max_x {
            for j in y..=max_y {
                coordinates.push((i, j))
            }
        }

        return coordinates
    }
}

