#![allow(dead_code)]

fn main() {
    day25::part1();
}

mod day1 {
    pub fn part1() {
        let input: Vec<u32> = include_str!("../inputs/day1.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        for (i, first) in input.iter().enumerate() {
            for second in input[i..].iter() {
                if first + second == 2020 {
                    println!("{}", first * second);
                }
            }
        }
    }
    pub fn part2() {
        let input: Vec<u32> = include_str!("../inputs/day1.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        for (i, first) in input.iter().enumerate() {
            for (j, second) in input[i..].iter().enumerate() {
                for third in input[j..].iter() {
                    if first + second + third == 2020 {
                        println!("{}", first * second * third);
                    }
                }
            }
        }
    }
}
mod day2 {
    struct Rule {
        min: u32,
        max: u32,
        chr: char,
    }
    struct Password {
        rules: Rule,
        password: String,
    }
    impl Password {
        fn is_valid_count(&self) -> bool {
            let count = self
                .password
                .chars()
                .filter(|&c| c == self.rules.chr)
                .count() as u32;
            self.rules.min <= count && count <= self.rules.max
        }
        fn is_valid_position(&self) -> bool {
            let start_valid = self
                .password
                .chars()
                .nth(self.rules.min as usize - 1)
                .unwrap()
                == self.rules.chr;
            let end_valid = self
                .password
                .chars()
                .nth(self.rules.max as usize - 1)
                .unwrap()
                == self.rules.chr;
            start_valid ^ end_valid
        }
    }
    impl std::str::FromStr for Password {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split(' ');
            let (min, max) = {
                let mut bounds = s.next().unwrap().split('-').map(|num| num.parse().unwrap());
                (bounds.next().unwrap(), bounds.next().unwrap())
            };
            let chr = s.next().unwrap().chars().next().unwrap();
            let rules = Rule { min, max, chr };
            let password = s.next().unwrap().to_string();
            Ok(Password { rules, password })
        }
    }
    pub fn part1() {
        let input: Vec<Password> = include_str!("../inputs/day2.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        println!(
            "{}",
            input
                .iter()
                .filter(|password| password.is_valid_count())
                .count()
        )
    }
    pub fn part2() {
        let input: Vec<Password> = include_str!("../inputs/day2.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

        println!(
            "{}",
            input
                .iter()
                .filter(|password| password.is_valid_position())
                .count()
        )
    }
}
mod day3 {
    fn slide(trees: &[Vec<bool>], delta_x: usize, delta_y: usize) -> usize {
        let (mut row, mut column) = (0, 0);
        let mut count = 0;
        while row < trees.len() {
            if trees[row][column] {
                count += 1
            }
            row += delta_y;
            column = (column + delta_x) % trees[0].len();
        }
        count
    }
    pub fn part1() {
        let input: Vec<Vec<bool>> = include_str!("../inputs/day3.txt")
            .lines()
            .map(|line| line.chars().map(|chr| chr == '#').collect())
            .collect();

        println!("{}", slide(&input, 3, 1));
    }
    pub fn part2() {
        let input: Vec<Vec<bool>> = include_str!("../inputs/day3.txt")
            .lines()
            .map(|line| line.chars().map(|chr| chr == '#').collect())
            .collect();
        println!(
            "{}",
            slide(&input, 1, 1)
                * slide(&input, 3, 1)
                * slide(&input, 5, 1)
                * slide(&input, 7, 1)
                * slide(&input, 1, 2)
        );
    }
}
mod day4 {
    pub fn part1() {
        let input: Vec<String> = include_str!("../inputs/day4.txt")
            .split("\r\n\r\n")
            .map(|string| string.replace("\r\n", " "))
            .filter(|string| !string.is_empty())
            .collect();
        println!(
            "{}",
            input
                .iter()
                .filter(|string| {
                    let mut birth_year = false;
                    let mut issue_year = false;
                    let mut exp_year = false;
                    let mut height = false;
                    let mut hair_colour = false;
                    let mut eye_colour = false;
                    let mut passport_id = false;
                    let mut country_id = false;

                    string
                        .trim()
                        .split(' ')
                        .for_each(|substr| match &substr[..3] {
                            "byr" => birth_year = true,
                            "iyr" => issue_year = true,
                            "eyr" => exp_year = true,
                            "hgt" => height = true,
                            "hcl" => hair_colour = true,
                            "ecl" => eye_colour = true,
                            "pid" => passport_id = true,
                            "cid" => country_id = true,
                            _ => unreachable!(),
                        });
                    birth_year
                        && issue_year
                        && exp_year
                        && height
                        && hair_colour
                        && eye_colour
                        && passport_id
                })
                .count()
        );
    }
    pub fn part2() {
        let input: Vec<String> = include_str!("../inputs/day4.txt")
            .split("\r\n\r\n")
            .map(|string| string.replace("\r\n", " "))
            .filter(|string| !string.is_empty())
            .collect();
        println!(
            "{}",
            input
                .iter()
                .filter(|string| {
                    let mut birth_year = false;
                    let mut issue_year = false;
                    let mut exp_year = false;
                    let mut height = false;
                    let mut hair_colour = false;
                    let mut eye_colour = false;
                    let mut passport_id = false;
                    let mut country_id = false;

                    string
                        .trim()
                        .split(' ')
                        .for_each(|substr| match &substr[..3] {
                            "byr" => {
                                if 1920 <= substr[4..].parse().unwrap()
                                    && 2002 >= substr[4..].parse().unwrap()
                                {
                                    birth_year = true
                                }
                            }
                            "iyr" => {
                                if 2010 <= substr[4..].parse().unwrap()
                                    && 2020 >= substr[4..].parse().unwrap()
                                {
                                    issue_year = true
                                }
                            }
                            "eyr" => {
                                if 2020 <= substr[4..].parse().unwrap()
                                    && 2030 >= substr[4..].parse().unwrap()
                                {
                                    exp_year = true
                                }
                            }
                            "hgt" => {
                                let mut height_value = 0;
                                let mut ptr = 4;

                                while ptr < substr.len() {
                                    match substr.as_bytes()[ptr] {
                                        b'0'..=b'9' => {
                                            height_value *= 10;
                                            height_value += substr.as_bytes()[ptr] - b'0'
                                        }
                                        _ => break,
                                    }
                                    ptr += 1
                                }
                                let units = &substr[ptr..];
                                match units {
                                    "cm" => {
                                        if (150..=193).contains(&height_value) {
                                            height = true
                                        }
                                    }
                                    "in" => {
                                        if (59..=76).contains(&height_value) {
                                            height = true
                                        }
                                    }
                                    _ => {} //invalid units
                                }
                            }
                            "hcl" => {
                                if substr.as_bytes()[4] == b'#'
                                    && substr.len() == 11
                                    && substr[5..]
                                        .as_bytes()
                                        .iter()
                                        .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
                                {
                                    hair_colour = true
                                };
                            }
                            "ecl" => match &substr[4..] {
                                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                                    eye_colour = true
                                }
                                _ => {}
                            },
                            "pid" => {
                                if substr[4..].len() == 9 {
                                    passport_id = true
                                }
                            }
                            "cid" => country_id = true,
                            _ => unreachable!(),
                        });

                    birth_year
                        && issue_year
                        && exp_year
                        && height
                        && hair_colour
                        && eye_colour
                        && passport_id
                })
                .count()
        );
    }
}
mod day5 {
    fn find(seat: &str) -> u32 {
        let mut lo = 0;
        let mut hi = 127;
        for partition in &seat.as_bytes()[..7] {
            match partition {
                b'F' => hi = (hi + lo) / 2,
                b'B' => lo = (hi + lo + 1) / 2,
                _ => panic!(),
            }
        }
        assert!(lo == hi);
        let mut left = 0;
        let mut right = 7;
        for side in &seat.as_bytes()[7..] {
            match side {
                b'L' => right = (right + left) / 2,
                b'R' => left = (right + left + 1) / 2,
                _ => panic!(),
            }
        }
        assert!(left == right);
        let row = lo;
        let column = left;
        row * 8 + column
    }
    pub fn part1() {
        let input = include_str!("../inputs/day5.txt");
        println!("{}", input.lines().map(|line| find(line)).max().unwrap());
    }
    pub fn part2() {
        let input = include_str!("../inputs/day5.txt");
        let mut seats = input.lines().map(|line| find(line)).collect::<Vec<_>>();
        seats.sort_unstable();
        let base = seats[0];
        for (i, &seat) in seats.iter().enumerate() {
            if seat != base + i as u32 {
                println!("{}", base + i as u32);
                return;
            }
        }
        panic!("seat not found")
    }
}
mod day6 {
    use std::collections::{HashMap, HashSet};
    pub fn part1() {
        let input: Vec<String> = include_str!("../inputs/day6.txt")
            .split("\r\n\r\n")
            .map(|string| string.to_string())
            .filter(|string| !string.is_empty())
            .collect();
        println!(
            "{:?}",
            input
                .iter()
                .map(|group| {
                    let mut seen = HashSet::new();
                    for chr in group.chars() {
                        if !chr.is_whitespace() {
                            seen.insert(chr);
                        }
                    }
                    seen.len()
                })
                .sum::<usize>()
        );
    }
    pub fn part2() {
        let input: Vec<String> = include_str!("../inputs/day6.txt")
            .split("\r\n\r\n")
            .map(|string| string.to_string())
            .filter(|string| !string.is_empty())
            .collect();
        println!(
            "{:?}",
            input
                .iter()
                .map(|group| {
                    let mut seen = HashMap::new();
                    for chr in group.chars() {
                        if !chr.is_whitespace() {
                            *seen.entry(chr).or_insert(0) += 1;
                        }
                    }
                    seen.iter()
                        .filter(|(_key, value)| **value == group.lines().count())
                        .count()
                })
                .sum::<usize>()
        );
    }
}
mod day7 {
    use std::collections::{HashMap, HashSet};

    fn contains_shiny_gold_bag(
        bag: &str,
        capacities: &HashMap<String, Vec<(u32, String)>>,
        memo: &mut HashMap<String, bool>,
    ) -> bool {
        if memo.contains_key(bag) {
            return memo[bag];
        }
        let can_contain = capacities[bag].iter().any(|(_limit, bag)| {
            if bag == "shiny gold" {
                true
            } else {
                contains_shiny_gold_bag(bag, capacities, memo)
            }
        });
        memo.insert(bag.to_string(), can_contain);
        memo[bag]
    }
    pub fn part1() {
        let input: Vec<&str> = include_str!("../inputs/day7.txt").lines().collect();
        let mut bag_type = HashSet::with_capacity(input.len());
        let mut bag_capacities = HashMap::new();

        for string in input {
            let mut string = string.split(" contain ");
            let left = string.next().unwrap().replacen(" bags", "", 1);
            let right = string.next().unwrap().trim_end_matches('.').split(", ");

            if !bag_type.contains(&left) {
                bag_type.insert(left.clone());
            }
            let mut holding_capacity = Vec::new();
            for can_hold in right {
                if can_hold == "no other bags" {
                    break;
                }
                let mut can_hold = can_hold.split(' ');
                let count: u32 = can_hold.next().unwrap().parse().unwrap();

                let mut name = String::new();
                name += can_hold.next().unwrap();
                name += " ";
                name += can_hold.next().unwrap();

                if !bag_type.contains(&name) {
                    bag_type.insert(name.clone());
                }
                holding_capacity.push((count, name));
            }
            bag_capacities.insert(left, holding_capacity);
        }

        let mut can_contain_shiny_gold = HashMap::new();
        for bag in &bag_type {
            contains_shiny_gold_bag(bag, &bag_capacities, &mut can_contain_shiny_gold);
        }

        println!(
            "{}",
            can_contain_shiny_gold
                .iter()
                .filter(|(_key, value)| **value)
                .count()
        );
    }
    fn bag_count(
        bag: &str,
        capacities: &HashMap<String, Vec<(usize, String)>>,
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if memo.contains_key(bag) {
            return memo[bag];
        }

        let count = capacities[bag]
            .iter()
            .map(|(limit, bag)| {
                if bag == "shiny gold" {
                    panic!("cycle found")
                } else {
                    limit * bag_count(bag, capacities, memo) + limit
                }
            })
            .sum();
        memo.insert(bag.to_string(), count);
        memo[bag]
    }
    pub fn part2() {
        let input: Vec<&str> = include_str!("../inputs/day7.txt").lines().collect();
        let mut bag_type = HashMap::with_capacity(input.len());
        let mut bag_capacities = HashMap::new();

        for string in input {
            let mut string = string.split(" contain ");
            let left = string.next().unwrap().replacen(" bags", "", 1);
            let right = string.next().unwrap().trim_end_matches('.').split(", ");

            if !bag_type.contains_key(&left) {
                bag_type.insert(left.clone(), bag_type.len());
            }
            let mut holding_capacity = Vec::new();
            for can_hold in right {
                if can_hold == "no other bags" {
                    break;
                }
                let mut can_hold = can_hold.split(' ');
                let count: usize = can_hold.next().unwrap().parse().unwrap();

                let mut name = String::new();
                name += can_hold.next().unwrap();
                name += " ";
                name += can_hold.next().unwrap();

                if !bag_type.contains_key(&name) {
                    bag_type.insert(name.clone(), bag_type.len());
                }
                holding_capacity.push((count, name));
            }
            bag_capacities.insert(left, holding_capacity);
        }

        let mut can_contain_shiny_gold = HashMap::new();

        println!(
            "{}",
            bag_count("shiny gold", &bag_capacities, &mut can_contain_shiny_gold)
        );
    }
}
mod day8 {
    use std::vec;
    struct Machine {
        instructions: Vec<Ops>,
        acc: i32,
        current: usize,
    }
    impl Machine {
        fn run_until_looped(&mut self) -> TerminateReason {
            let mut visited = vec![false; self.instructions.len()];
            loop {
                if self.current >= self.instructions.len() {
                    return TerminateReason::EndOfProgram;
                }
                if visited[self.current] {
                    return TerminateReason::InfiniteLoop;
                }

                visited[self.current] = true;

                match self.instructions[self.current] {
                    Ops::Jmp(arg) => self.current = (self.current as i32 + arg) as usize,
                    Ops::Acc(arg) => {
                        self.acc += arg;
                        self.current += 1
                    }
                    Ops::Nop(_) => self.current += 1,
                }
            }
        }
    }
    #[derive(Debug, Clone)]
    enum Ops {
        Nop(i32),
        Acc(i32),
        Jmp(i32),
    }
    impl std::str::FromStr for Ops {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut s = s.split(' ');
            match s.next().unwrap() {
                "nop" => Ok(Ops::Nop(s.next().unwrap().parse().unwrap())),
                "acc" => Ok(Ops::Acc(s.next().unwrap().parse().unwrap())),
                "jmp" => Ok(Ops::Jmp(s.next().unwrap().parse().unwrap())),
                _ => Err("unknown instruction"),
            }
        }
    }
    pub fn part1() {
        let input: Vec<Ops> = include_str!("../inputs/day8.txt")
            .lines()
            .map(|ops| ops.parse().unwrap())
            .collect();
        let mut machine = Machine {
            instructions: input,
            acc: 0,
            current: 0,
        };
        machine.run_until_looped();
        println!("{}", machine.acc);
    }
    enum TerminateReason {
        InfiniteLoop,
        EndOfProgram,
    }
    pub fn part2() {
        let input: Vec<Ops> = include_str!("../inputs/day8.txt")
            .lines()
            .map(|ops| ops.parse().unwrap())
            .collect();
        for i in 0..input.len() {
            let mut input = input.clone();
            match input[i] {
                Ops::Nop(arg) => input[i] = Ops::Jmp(arg),
                Ops::Jmp(arg) => input[i] = Ops::Nop(arg),
                Ops::Acc(_) => {
                    continue;
                }
            }
            let mut machine = Machine {
                instructions: input,
                acc: 0,
                current: 0,
            };
            match machine.run_until_looped() {
                TerminateReason::EndOfProgram => {
                    println!("{}", machine.acc);
                    return;
                }
                TerminateReason::InfiniteLoop => {}
            }
        }
    }
}
mod day9 {
    use std::{cmp::Ordering, collections::VecDeque};

    pub fn part1() {
        const PREAMBLE: usize = 25;
        let input: Vec<u64> = include_str!("../inputs/day9.txt")
            .lines()
            .map(|ops| ops.parse().unwrap())
            .collect();

        let mut queue = {
            let mut queue = VecDeque::with_capacity(PREAMBLE);
            for &i in input.iter().take(PREAMBLE) {
                queue.push_back(i);
            }
            queue
        };

        for &num in &input[PREAMBLE..input.len()] {
            let valid = queue.iter().any(|first| {
                queue
                    .iter()
                    .any(|second| first != second && first + second == num)
            });
            if valid {
                queue.pop_front();
                queue.push_back(num);
            } else {
                println!("{}", num);
                return;
            }
        }
    }
    pub fn part2() {
        const PREAMBLE: usize = 25;
        let input: Vec<u64> = include_str!("../inputs/day9.txt")
            .lines()
            .map(|ops| ops.parse().unwrap())
            .collect();

        let mut queue = {
            let mut queue = VecDeque::with_capacity(PREAMBLE);
            for &i in input.iter().take(PREAMBLE) {
                queue.push_back(i);
            }
            queue
        };
        let mut i = PREAMBLE;
        while i < input.len() {
            let num = input[i];
            let valid = queue.iter().any(|first| {
                queue
                    .iter()
                    .any(|second| first != second && first + second == num)
            });
            if valid {
                queue.pop_front();
                queue.push_back(num);
            } else {
                break;
            }
            i += 1;
        }

        for start in 0..i {
            for end in start..i {
                match input[start..end].iter().sum::<u64>().cmp(&input[i]) {
                    Ordering::Equal => {
                        println!(
                            "{}",
                            input[start..end].iter().max().unwrap()
                                + input[start..end].iter().min().unwrap()
                        );
                        return;
                    }
                    Ordering::Greater => break,
                    Ordering::Less => {}
                }
            }
        }
    }
}
mod day10 {
    pub fn part1() {
        let mut input: Vec<u32> = include_str!("../inputs/day10.txt")
            .lines()
            .map(|num| num.parse().unwrap())
            .collect();
        input.push(0);
        input.push(input.iter().max().unwrap() + 3);
        input.sort_unstable();
        let (ones, threes) = input.windows(2).fold((0, 0), |(ones, threes), window| {
            match window[1] - window[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            }
        });
        println!("{}", ones * threes);
    }
    pub fn part2() {
        let mut input: Vec<u32> = include_str!("../inputs/day10.txt")
            .lines()
            .map(|num| num.parse().unwrap())
            .collect();
        input.push(0);
        input.push(input.iter().max().unwrap() + 3);
        input.sort_unstable();

        let mut ways: Vec<u64> = vec![0; input.len()];
        ways[0] = 1;

        for i in 0..input.len() {
            for j in i + 1..input.len() {
                if input[j] - input[i] <= 3 {
                    ways[j] += ways[i];
                } else {
                    break;
                }
            }
        }

        println!("{}", ways.last().unwrap())
    }
}
mod day11 {
    #[derive(Debug, Clone)]
    enum Seat {
        Floor,
        Empty,
        Occupied,
    }
    impl Seat {
        fn from_bytes(byte: u8) -> Self {
            match byte {
                b'.' => Seat::Floor,
                b'L' => Seat::Empty,
                b'#' => Seat::Occupied,
                _ => panic!(),
            }
        }
    }
    pub fn part1() {
        let mut layout: Vec<Vec<Seat>> = include_str!("../inputs/day11.txt")
            .lines()
            .map(|row| row.bytes().map(Seat::from_bytes).collect())
            .collect();
        let mut changed = true;
        while changed {
            let mut new_layout = layout.clone();
            changed = false;
            for row in 0..new_layout.len() {
                for col in 0..new_layout[0].len() {
                    let prev_row_valid = row > 0;
                    let prev_col_valid = col > 0;
                    let next_row_valid = row + 1 < layout.len();
                    let next_col_valid = col + 1 < layout[0].len();
                    let occupied_neighbours = [
                        (prev_row_valid
                            && prev_col_valid
                            && matches!(layout[row - 1][col - 1], Seat::Occupied)),
                        (prev_row_valid && matches!(layout[row - 1][col], Seat::Occupied)),
                        (prev_row_valid
                            && next_col_valid
                            && matches!(layout[row - 1][col + 1], Seat::Occupied)),
                        (prev_col_valid && matches!(layout[row][col - 1], Seat::Occupied)),
                        (next_col_valid && matches!(layout[row][col + 1], Seat::Occupied)),
                        (next_row_valid
                            && prev_col_valid
                            && matches!(layout[row + 1][col - 1], Seat::Occupied)),
                        (next_row_valid && matches!(layout[row + 1][col], Seat::Occupied)),
                        (next_row_valid
                            && next_col_valid
                            && matches!(layout[row + 1][col + 1], Seat::Occupied)),
                    ]
                    .iter()
                    .filter(|neighbour| **neighbour)
                    .count();
                    match layout[row][col] {
                        Seat::Empty => {
                            if occupied_neighbours == 0 {
                                new_layout[row][col] = Seat::Occupied;
                                changed = true;
                            }
                        }
                        Seat::Occupied => {
                            if occupied_neighbours >= 4 {
                                new_layout[row][col] = Seat::Empty;
                                changed = true;
                            }
                        }
                        Seat::Floor => {}
                    }
                }
            }
            layout = new_layout;
        }
        println!(
            "{}",
            layout
                .iter()
                .map(|row| row
                    .iter()
                    .filter(|seat| matches!(seat, Seat::Occupied))
                    .count())
                .sum::<usize>()
        );
    }
    pub fn part2() {
        let mut layout: Vec<Vec<Seat>> = include_str!("../inputs/day11.txt")
            .lines()
            .map(|row| row.bytes().map(Seat::from_bytes).collect())
            .collect();
        let mut changed = true;
        while changed {
            let mut new_layout = layout.clone();
            changed = false;
            for row in 0..new_layout.len() {
                for col in 0..new_layout[0].len() {
                    let find_occupied = |(row, col): (usize, usize)| match layout[row][col] {
                        Seat::Empty => Some(false),
                        Seat::Occupied => Some(true),
                        Seat::Floor => None,
                    };
                    let top_left = (0..row).rev().zip((0..col).rev()).find_map(find_occupied);
                    let top = (0..row).rev().map(|row| (row, col)).find_map(find_occupied);
                    let top_right = (0..row)
                        .rev()
                        .zip(col + 1..layout[0].len())
                        .find_map(find_occupied);
                    let left = (0..col).rev().map(|col| (row, col)).find_map(find_occupied);
                    let right = (col + 1..layout[0].len())
                        .map(|col| (row, col))
                        .find_map(find_occupied);
                    let bot_left = (row + 1..layout.len())
                        .zip((0..col).rev())
                        .find_map(find_occupied);
                    let bot = (row + 1..layout.len())
                        .map(|row| (row, col))
                        .find_map(find_occupied);
                    let bot_right = (row + 1..layout.len())
                        .zip(col + 1..layout[0].len())
                        .find_map(find_occupied);
                    let occupied_neighbours = [
                        top_left, top, top_right, left, right, bot_left, bot, bot_right,
                    ]
                    .iter()
                    .filter(|neighbour| **neighbour == Some(true))
                    .count();

                    match layout[row][col] {
                        Seat::Empty => {
                            if occupied_neighbours == 0 {
                                new_layout[row][col] = Seat::Occupied;
                                changed = true;
                            }
                        }
                        Seat::Occupied => {
                            if occupied_neighbours >= 5 {
                                new_layout[row][col] = Seat::Empty;
                                changed = true;
                            }
                        }
                        Seat::Floor => {}
                    }
                }
            }
            layout = new_layout;
            // dbg!(&layout);
        }
        println!(
            "{}",
            layout
                .iter()
                .map(|row| row
                    .iter()
                    .filter(|seat| matches!(seat, Seat::Occupied))
                    .count())
                .sum::<usize>()
        );
    }
}
mod day12 {
    #[derive(Debug)]
    enum Instructions {
        North(u32),
        South(u32),
        East(u32),
        West(u32),
        Left(u32),
        Right(u32),
        Forward(u32),
    }
    impl std::str::FromStr for Instructions {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let magnitude = s[1..].parse().unwrap();
            Ok(match &s[..1] {
                "N" => Instructions::North(magnitude),
                "S" => Instructions::South(magnitude),
                "E" => Instructions::East(magnitude),
                "W" => Instructions::West(magnitude),
                "L" => Instructions::Left(magnitude),
                "R" => Instructions::Right(magnitude),
                "F" => Instructions::Forward(magnitude),
                _ => return Err("idk"),
            })
        }
    }
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }
    impl Direction {
        fn right(&mut self) {
            *self = match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }
        fn left(&mut self) {
            *self = match self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            }
        }
    }
    #[derive(Debug)]
    struct Ship {
        x: i32,
        y: i32,
        dir: Direction,
    }
    pub fn part1() {
        let input: Vec<Instructions> = include_str!("../inputs/day12.txt")
            .lines()
            .map(|ins| ins.parse().unwrap())
            .collect();
        let ship = input.iter().fold(
            Ship {
                x: 0,
                y: 0,
                dir: Direction::East,
            },
            |mut ship, ins| {
                match ins {
                    Instructions::North(dist) => ship.y += *dist as i32,
                    Instructions::South(dist) => ship.y -= *dist as i32,
                    Instructions::East(dist) => ship.x += *dist as i32,
                    Instructions::West(dist) => ship.x -= *dist as i32,
                    Instructions::Left(angle) => {
                        for _ in 0..angle / 90 {
                            ship.dir.left()
                        }
                    }
                    Instructions::Right(angle) => {
                        for _ in 0..angle / 90 {
                            ship.dir.right()
                        }
                    }
                    Instructions::Forward(dist) => match ship.dir {
                        Direction::North => ship.y += *dist as i32,
                        Direction::South => ship.y -= *dist as i32,
                        Direction::East => ship.x += *dist as i32,
                        Direction::West => ship.x -= *dist as i32,
                    },
                };
                ship
            },
        );

        println!("{}", ship.x.abs() + ship.y.abs());
    }
    struct Waypoint {
        x: i32,
        y: i32,
    }
    impl Waypoint {
        fn left(&mut self) {
            let temp = self.x;
            self.x = -self.y;
            self.y = temp;
        }
        fn right(&mut self) {
            for _ in 0..3 {
                self.left()
            }
        }
    }
    pub fn part2() {
        let input: Vec<Instructions> = include_str!("../inputs/day12.txt")
            .lines()
            .map(|ins| ins.parse().unwrap())
            .collect();

        let (ship, _waypoint) = input.iter().fold(
            (
                Ship {
                    x: 0,
                    y: 0,
                    dir: Direction::East,
                },
                Waypoint { x: 10, y: 1 },
            ),
            |(mut ship, mut waypoint), ins| {
                match ins {
                    Instructions::North(dist) => waypoint.y += *dist as i32,
                    Instructions::South(dist) => waypoint.y -= *dist as i32,
                    Instructions::East(dist) => waypoint.x += *dist as i32,
                    Instructions::West(dist) => waypoint.x -= *dist as i32,
                    Instructions::Left(angle) => {
                        for _ in 0..angle / 90 {
                            waypoint.left()
                        }
                    }
                    Instructions::Right(angle) => {
                        for _ in 0..angle / 90 {
                            waypoint.right()
                        }
                    }
                    Instructions::Forward(times) => {
                        let times = *times as i32;
                        ship.x += times * waypoint.x;
                        ship.y += times * waypoint.y
                    }
                };
                (ship, waypoint)
            },
        );

        println!("{}", ship.x.abs() + ship.y.abs());
    }
}
mod day13 {
    pub fn part1() {
        let mut input = include_str!("../inputs/day13.txt").lines();
        let current_time: u32 = input.next().unwrap().parse().unwrap();
        let busses: Vec<u32> = input
            .next()
            .unwrap()
            .split(',')
            .filter_map(|id| id.parse().ok())
            .collect();

        let (id, time) = busses
            .iter()
            .map(|&id| (id, id * (1 + current_time / id)))
            .min_by_key(|(_id, time)| *time)
            .unwrap();
        println!("{}", id * (time - current_time));
    }
    pub fn part2() {
        let mut input = include_str!("../inputs/day13.txt").lines();
        let _current_time: u64 = input.next().unwrap().parse().unwrap();
        let busses: Vec<(u64, u64)> = input
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(count, id)| {
                if let Ok(id) = id.parse() {
                    Some((count as u64, id))
                } else {
                    None
                }
            })
            .collect();

        // assumes first bus is not "x"
        let mut i = 0;
        let mut mult = 1;
        let mut time = mult;
        while i < busses.len() {
            while (time + busses[i].0) % busses[i].1 != 0 {
                time += mult
            }
            mult *= busses[i].1; // should be LCM, but all given numbers are prime
            i += 1;
        }
        println!("{}", time);
    }
}
mod day14 {
    use std::collections::HashMap;
    #[derive(Debug)]
    enum Mask {
        Override(bool),
        Unchanged,
    }
    impl Mask {
        fn from_char(s: char) -> Self {
            match s {
                '1' => Mask::Override(true),
                '0' => Mask::Override(false),
                'X' => Mask::Unchanged,
                _ => panic!("unknown mask"),
            }
        }
    }
    #[derive(Debug)]
    struct Write {
        address: u64,
        value: u64,
    }
    impl Write {
        fn mask_values(address: u64, mut value: u64, bitmask: &[Mask]) -> Self {
            for (i, mask) in bitmask.iter().rev().enumerate() {
                match mask {
                    Mask::Override(true) => {
                        // set bit at i to 1
                        let mask = 1 << i;
                        value |= mask;
                    }
                    Mask::Override(false) => {
                        // set bit at i to 0
                        let mask = u64::MAX ^ (1 << i);
                        value &= mask;
                    }
                    Mask::Unchanged => {}
                }
            }
            Self { address, value }
        }
        fn mask_address(address: u64, value: u64, bitmask: &[Mask2]) -> Vec<Self> {
            let mut addresses = vec![0];
            let mut i = 0;
            while i < bitmask.len() {
                match bitmask[i] {
                    Mask2::Override => addresses.iter_mut().for_each(|addr| {
                        *addr <<= 1;
                        *addr += 1
                    }),
                    Mask2::Unchanged => addresses.iter_mut().for_each(|addr| {
                        *addr <<= 1;
                        *addr |= (address >> (bitmask.len() - 1 - i)) & 1;
                    }),
                    Mask2::Floating => {
                        for addr in &mut addresses {
                            *addr <<= 1;
                        }
                        for i in 0..addresses.len() {
                            addresses.push(addresses[i] + 1);
                        }
                    }
                }
                i += 1;
            }

            addresses
                .into_iter()
                .map(|address| Write { address, value })
                .collect()
        }
    }
    pub fn part1() {
        let writes: Vec<Write> = include_str!("../inputs/day14.txt")
            .split("mask = ")
            .filter(|line| !line.is_empty())
            .fold(Vec::new(), |mut acc, group| {
                let mut lines = group.lines();
                let bitmask: Vec<Mask> =
                    lines.next().unwrap().chars().map(Mask::from_char).collect();

                acc.extend(lines.map(|line| {
                    let mut write = line.split(" = ");
                    let address: u64 = write
                        .next()
                        .unwrap()
                        .strip_prefix("mem[")
                        .unwrap()
                        .strip_suffix(']')
                        .unwrap()
                        .parse()
                        .unwrap();
                    let value: u64 = write.next().unwrap().parse().unwrap();
                    Write::mask_values(address, value, &bitmask)
                }));
                acc
            });

        let mut memory: HashMap<u64, u64> = HashMap::new();
        for write in writes {
            memory.insert(write.address, write.value);
        }

        println!("{}", memory.values().sum::<u64>());
    }
    #[derive(Debug)]
    enum Mask2 {
        Override,
        Unchanged,
        Floating,
    }
    impl Mask2 {
        fn from_char(s: char) -> Self {
            match s {
                '1' => Mask2::Override,
                '0' => Mask2::Unchanged,
                'X' => Mask2::Floating,
                _ => panic!("unknown mask"),
            }
        }
    }
    pub fn part2() {
        let writes: Vec<Write> = include_str!("../inputs/day14.txt")
            .split("mask = ")
            .filter(|line| !line.is_empty())
            .fold(Vec::new(), |mut acc, group| {
                let mut lines = group.lines();
                let bitmask: Vec<Mask2> = lines
                    .next()
                    .unwrap()
                    .chars()
                    .map(Mask2::from_char)
                    .collect();

                acc.extend(lines.flat_map(|line| {
                    let mut write = line.split(" = ");
                    let address: u64 = write
                        .next()
                        .unwrap()
                        .strip_prefix("mem[")
                        .unwrap()
                        .strip_suffix(']')
                        .unwrap()
                        .parse()
                        .unwrap();
                    let value: u64 = write.next().unwrap().parse().unwrap();
                    Write::mask_address(address, value, &bitmask)
                }));
                acc
            });

        let mut memory: HashMap<u64, u64> = HashMap::new();
        for write in &writes {
            memory.insert(write.address, write.value);
        }

        println!("{}", memory.values().sum::<u64>());
    }
}
mod day15 {
    use std::collections::HashMap;
    pub fn part1() {
        let input: Vec<usize> = include_str!("../inputs/day15.txt")
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let mut last_seen = HashMap::new();
        let mut i = 1;
        let mut prev = *input.first().unwrap();
        while i < input.len() {
            last_seen.insert(prev, i + 1);
            prev = input[i];
            i += 1;
        }
        while i < 2020 {
            let next = if last_seen.contains_key(&prev) {
                i + 1 - last_seen[&prev]
            } else {
                0
            };

            last_seen.insert(prev, i + 1);
            i += 1;
            prev = next;
        }
        println!("{}", prev);
    }
    pub fn part2() {
        let input: Vec<usize> = include_str!("../inputs/day15.txt")
            .trim()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let mut last_seen = HashMap::new();
        let mut i = 1;
        let mut prev = *input.first().unwrap();
        while i < input.len() {
            last_seen.insert(prev, i + 1);
            prev = input[i];
            i += 1;
        }
        while i < 30_000_000 {
            let next = if last_seen.contains_key(&prev) {
                i + 1 - last_seen[&prev]
            } else {
                0
            };

            last_seen.insert(prev, i + 1);
            i += 1;
            prev = next;
        }
        println!("{}", prev);
    }
}
mod day16 {
    use std::ops::RangeInclusive;
    pub fn part1() {
        let mut input = include_str!("../inputs/day16.txt").split("\r\n\r\n");
        let valid: Vec<Vec<RangeInclusive<u32>>> = input
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let ranges = line.split(": ").nth(1).unwrap();
                ranges
                    .split(" or ")
                    .map(|range| {
                        let mut range = range.split('-');
                        let lower = range.next().unwrap().parse().unwrap();
                        let upper = range.next().unwrap().parse().unwrap();
                        lower..=upper
                    })
                    .collect()
            })
            .collect();
        let _my_ticket: Vec<u32> = input
            .next()
            .unwrap()
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let nearby_tickets: Vec<Vec<u32>> = input
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|ticket| ticket.split(',').map(|n| n.parse().unwrap()).collect())
            .collect();
        let error_rate = nearby_tickets
            .iter()
            .flatten()
            .map(|ticket| {
                if valid
                    .iter()
                    .any(|ranges| ranges.iter().any(|range| range.contains(ticket)))
                {
                    0
                } else {
                    *ticket
                }
            })
            .sum::<u32>();
        println!("{}", error_rate);
    }
    pub fn part2() {
        let mut input = include_str!("../inputs/day16.txt").split("\r\n\r\n");
        let valid: Vec<Vec<RangeInclusive<u64>>> = input
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let ranges = line.split(": ").nth(1).unwrap();
                ranges
                    .split(" or ")
                    .map(|range| {
                        let mut range = range.split('-');
                        let lower = range.next().unwrap().parse().unwrap();
                        let upper = range.next().unwrap().parse().unwrap();
                        lower..=upper
                    })
                    .collect()
            })
            .collect();
        let my_ticket: Vec<u64> = input
            .next()
            .unwrap()
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        let mut nearby_tickets: Vec<Vec<u64>> = input
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|ticket| ticket.split(',').map(|n| n.parse().unwrap()).collect())
            .collect();
        let valid_field = |field: &u64, ranges: &[Vec<RangeInclusive<u64>>]| {
            ranges
                .iter()
                .any(|subrange| subrange.iter().any(|f| f.contains(field)))
        };
        nearby_tickets.retain(|ticket| ticket.iter().all(|field| valid_field(field, &valid)));

        let mut possibilities = vec![Vec::new(); nearby_tickets[0].len()];
        for i in 0..nearby_tickets[0].len() {
            for (field_no, field) in valid.iter().enumerate() {
                if nearby_tickets
                    .iter()
                    .map(|ticket| ticket[i])
                    .all(|ticket| field.iter().any(|f| f.contains(&ticket)))
                {
                    possibilities[i].push(field_no);
                }
            }
        }

        let mut finalized = vec![None; nearby_tickets[0].len()];
        while finalized.iter().any(Option::is_none) {
            let (idx, field) = possibilities
                .iter()
                .enumerate()
                .find(|(_i, options)| options.len() == 1)
                .unwrap();
            let f = field[0];
            finalized[idx] = Some(f);
            for options in &mut possibilities {
                *options = options
                    .iter_mut()
                    .filter_map(|num| if *num == f { None } else { Some(*num) })
                    .collect()
            }
        }

        println!(
            "{}",
            my_ticket
                .iter()
                .zip(finalized.iter())
                .filter_map(|(tix, field)| if field.unwrap() < 6 { Some(*tix) } else { None })
                .product::<u64>()
        );
    }
}
mod day17 {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, Clone)]
    enum State {
        Active,
        Inactive,
    }
    impl State {
        fn from_bytes(byte: u8) -> Self {
            match byte {
                b'#' => State::Active,
                _ => State::Inactive,
            }
        }
    }
    pub fn part1() {
        const CYCLES: usize = 6;
        let mut cubes = include_str!("../inputs/day17.txt")
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut hm, (y, line)| {
                line.as_bytes().iter().enumerate().for_each(|(x, byte)| {
                    hm.insert((x as i32, y as i32, 0i32), State::from_bytes(*byte));
                });
                hm
            });

        fn neighbours(x: i32, y: i32, z: i32) -> Vec<(i32, i32, i32)> {
            let mut neighs = Vec::new();
            for x_i in x - 1..=x + 1 {
                for y_i in y - 1..=y + 1 {
                    for z_i in z - 1..=z + 1 {
                        if (x, y, z) != (x_i, y_i, z_i) {
                            neighs.push((x_i, y_i, z_i));
                        }
                    }
                }
            }
            neighs
        }

        for _ in 0..CYCLES {
            let mut checkers = HashSet::new();
            for &(x, y, z) in cubes.keys() {
                (x - 1..=x + 1)
                    .flat_map(|x_i| {
                        (y - 1..=y + 1)
                            .flat_map(|y_i| {
                                (z - 1..=z + 1)
                                    .map(|z_i| (y_i, z_i))
                                    .collect::<Vec<(i32, i32)>>()
                            })
                            .map(|(y, z)| (x_i, y, z))
                            .collect::<Vec<(i32, i32, i32)>>()
                    })
                    .for_each(|s| {
                        checkers.insert(s);
                    });
            }

            let state_changes: Vec<((i32, i32, i32), State)> = checkers
                .into_iter()
                .filter_map(|(x, y, z)| {
                    let state = match cubes.get(&(x, y, z)) {
                        Some(State::Active) => State::Active,
                        _ => State::Inactive,
                    };
                    let active_neighbours = neighbours(x, y, z)
                        .iter()
                        .filter(|cube| matches!(cubes.get(cube), Some(State::Active)))
                        .count();
                    match state {
                        State::Active if !(2..=3).contains(&active_neighbours) => {
                            Some(((x, y, z), State::Inactive))
                        }
                        State::Inactive if active_neighbours == 3 => {
                            Some(((x, y, z), State::Active))
                        }
                        _ => None,
                    }
                })
                .collect();
            cubes.extend(state_changes.into_iter());
        }

        println!(
            "{}",
            cubes
                .values()
                .filter(|cube| matches!(cube, State::Active))
                .count()
        );
    }
    pub fn part2() {
        const CYCLES: usize = 6;
        let mut cubes = include_str!("../inputs/day17.txt")
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut hm, (y, line)| {
                line.as_bytes().iter().enumerate().for_each(|(x, byte)| {
                    hm.insert((x as i32, y as i32, 0i32, 0i32), State::from_bytes(*byte));
                });
                hm
            });

        fn neighbours(x: i32, y: i32, z: i32, w: i32) -> Vec<(i32, i32, i32, i32)> {
            let mut neighs = Vec::new();
            for x_i in x - 1..=x + 1 {
                for y_i in y - 1..=y + 1 {
                    for z_i in z - 1..=z + 1 {
                        for w_i in w - 1..=w + 1 {
                            if (x, y, z, w) != (x_i, y_i, z_i, w_i) {
                                neighs.push((x_i, y_i, z_i, w_i));
                            }
                        }
                    }
                }
            }
            neighs
        }

        for _ in 0..CYCLES {
            let mut checkers: HashSet<(i32, i32, i32, i32)> = HashSet::new();
            for &(x, y, z, w) in cubes.keys() {
                for s in &neighbours(x, y, z, w) {
                    checkers.insert(*s);
                }
            }
            let state_changes: Vec<((i32, i32, i32, i32), State)> = checkers
                .into_iter()
                .filter_map(|(x, y, z, w)| {
                    let state = match cubes.get(&(x, y, z, w)) {
                        Some(State::Active) => State::Active,
                        _ => State::Inactive,
                    };
                    let active_neighbours = neighbours(x, y, z, w)
                        .iter()
                        .filter(|cube| matches!(cubes.get(cube), Some(State::Active)))
                        .count();
                    match state {
                        State::Active if !(2..=3).contains(&active_neighbours) => {
                            Some(((x, y, z, w), State::Inactive))
                        }
                        State::Inactive if active_neighbours == 3 => {
                            Some(((x, y, z, w), State::Active))
                        }
                        _ => None,
                    }
                })
                .collect();
            cubes.extend(state_changes.into_iter());
        }

        println!(
            "{}",
            cubes
                .values()
                .filter(|cube| matches!(cube, State::Active))
                .count()
        );
    }
}
mod day18 {
    enum Value {
        Number(u64),
        Brackets(Eqn),
    }
    impl Value {
        fn calc(self) -> u64 {
            match self {
                Value::Number(i) => i,
                Value::Brackets(eqn) => eqn.eval(),
            }
        }
        fn from_str_plus_mult_equal(s: &str) -> Self {
            match s.as_bytes()[0] {
                b'(' => Value::Brackets(Eqn::from_str_plus_mult_equal(&s[1..s.len()])),
                _ => Value::Number(s.parse().unwrap()),
            }
        }
    }
    enum Eqn {
        Add(Box<Value>, Box<Value>),
        Mult(Box<Value>, Box<Value>),
    }
    impl Eqn {
        fn eval(self) -> u64 {
            match self {
                Eqn::Add(left, right) => left.calc() + right.calc(),
                Eqn::Mult(left, right) => left.calc() * right.calc(),
            }
        }
        fn from_str_plus_before_mult(s: &str) -> Self {
            // extract brackets, adds, then mults
            enum ValOrOps {
                Val(Value),
                Add,
                Multiply,
            }
            impl ValOrOps {
                fn take_val(self) -> Value {
                    match self {
                        ValOrOps::Val(value) => value,
                        _ => panic!(),
                    }
                }
            }
            let mut i = 0;
            let mut tokens = Vec::new();

            while i < s.len() {
                match s.as_bytes()[i] {
                    b' ' => i += 1,
                    b'+' => {
                        i += 1;
                        tokens.push(ValOrOps::Add);
                    }
                    b'*' => {
                        i += 1;
                        tokens.push(ValOrOps::Multiply)
                    }
                    b'0'..=b'9' => {
                        let start = i;
                        while i < s.len() && (b'0'..=b'9').contains(&s.as_bytes()[i]) {
                            i += 1;
                        }
                        let num = Value::Number(s[start..i].parse().unwrap());
                        tokens.push(ValOrOps::Val(num));
                    }
                    b'(' => {
                        let mut brackets = 1;
                        i += 1;
                        let start = i;
                        while brackets != 0 {
                            match s.as_bytes()[i] {
                                b'(' => brackets += 1,
                                b')' => brackets -= 1,
                                _ => {}
                            }
                            i += 1;
                        }
                        let brackets =
                            Value::Brackets(Eqn::from_str_plus_before_mult(&s[start..i - 1]));
                        tokens.push(ValOrOps::Val(brackets));
                    }
                    _ => {
                        unreachable!("{}", &s[i..=i])
                    }
                }
            }

            let mut i = 1;
            while i < tokens.len() {
                match tokens[i] {
                    ValOrOps::Add => {
                        let right = tokens.remove(i + 1).take_val();
                        tokens.remove(i);
                        let left = tokens.remove(i - 1).take_val();
                        let new = ValOrOps::Val(Value::Brackets(Eqn::Add(
                            Box::new(left),
                            Box::new(right),
                        )));

                        tokens.insert(i - 1, new);
                    }
                    ValOrOps::Multiply => i += 2,
                    _ => unreachable!(),
                }
            }

            let first = tokens
                .into_iter()
                .fold(None, |prev, curr| match curr {
                    ValOrOps::Val(v) => {
                        if let Some(prev) = prev {
                            Some(Value::Brackets(Eqn::Mult(Box::new(prev), Box::new(v))))
                        } else {
                            Some(v)
                        }
                    }
                    ValOrOps::Add | ValOrOps::Multiply => prev,
                })
                .unwrap();

            match first {
                Value::Number(_) => {
                    panic!()
                }
                Value::Brackets(eqn) => eqn,
            }
        }
        fn from_str_plus_mult_equal(s: &str) -> Self {
            let mut i = 0;
            let mut left: Option<Value> = None;
            let mut is_plus = false;

            while i < s.len() {
                match s.as_bytes()[i] {
                    b'(' => {
                        let mut brackets = 1;
                        i += 1;
                        let start = i;
                        while brackets != 0 {
                            match s.as_bytes()[i] {
                                b'(' => brackets += 1,
                                b')' => brackets -= 1,
                                _ => {}
                            }
                            i += 1;
                        }
                        let brackets =
                            Value::Brackets(Eqn::from_str_plus_mult_equal(&s[start..i - 1]));
                        if let Some(l) = left {
                            left = if is_plus {
                                Some(Value::Brackets(Eqn::Add(Box::new(l), Box::new(brackets))))
                            } else {
                                Some(Value::Brackets(Eqn::Mult(Box::new(l), Box::new(brackets))))
                            }
                        } else {
                            left = Some(brackets);
                        }
                    }
                    b' ' => {
                        i += 1;
                    }
                    b'+' => {
                        i += 1;
                        is_plus = true;
                    }
                    b'*' => {
                        i += 1;
                        is_plus = false;
                    }
                    b'0'..=b'9' => {
                        let start = i;
                        while i < s.len() && (b'0'..=b'9').contains(&s.as_bytes()[i]) {
                            i += 1;
                        }
                        let num = Value::Number(s[start..i].parse().unwrap());
                        if let Some(l) = left {
                            left = if is_plus {
                                Some(Value::Brackets(Eqn::Add(Box::new(l), Box::new(num))))
                            } else {
                                Some(Value::Brackets(Eqn::Mult(Box::new(l), Box::new(num))))
                            }
                        } else {
                            left = Some(num);
                        }
                    }
                    _ => {
                        unreachable!("{}", &s[i..=i])
                    }
                }
            }

            match left.unwrap() {
                Value::Number(_) => {
                    panic!("equation is just a number")
                }
                Value::Brackets(eqn) => eqn,
            }
        }
    }
    pub fn part1() {
        println!(
            "{}",
            include_str!("../inputs/day18.txt")
                .lines()
                .map(|line| { Eqn::from_str_plus_mult_equal(line).eval() })
                .sum::<u64>()
        );
    }
    pub fn part2() {
        println!(
            "{}",
            include_str!("../inputs/day18.txt")
                .lines()
                .map(|line| { Eqn::from_str_plus_before_mult(line).eval() })
                .sum::<u64>()
        );
    }
    #[test]
    fn test_eval_equal() {
        let eqns = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];
        for &(eqn, expected) in &eqns {
            assert_eq!(Eqn::from_str_plus_mult_equal(eqn).eval(), expected);
        }
    }
    #[test]
    fn test_eval_plus_first() {
        let eqns = [
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];
        for &(eqn, expected) in &eqns {
            assert_eq!(Eqn::from_str_plus_before_mult(eqn).eval(), expected);
        }
    }
}
mod day19 {
    use std::collections::{HashMap, VecDeque};
    enum Rule {
        Single(char),
        Array(Vec<u32>),
        Alt(Vec<u32>, Vec<u32>),
    }
    struct Rules {
        rules: HashMap<u32, Rule>,
    }
    impl Rules {
        fn matches(&self, s: &str) -> bool {
            let mut queue = VecDeque::new();
            queue.push_back((vec![self.rules.get(&0).unwrap()], s));
            while let Some((mut rules, substr)) = queue.pop_front() {
                if let Some(rule) = rules.pop() {
                    if substr.is_empty() {
                        continue;
                    }
                    match rule {
                        Rule::Single(chr) => {
                            if &substr.chars().next().unwrap() == chr {
                                queue.push_back((rules, &substr[1..]));
                            }
                        }
                        Rule::Array(arr) => {
                            rules.extend(
                                arr.iter()
                                    .map(|rule_id| self.rules.get(rule_id).unwrap())
                                    .rev(),
                            );
                            queue.push_back((rules, substr));
                        }
                        Rule::Alt(left, right) => {
                            let mut left_rules = rules.clone();
                            left_rules.extend(
                                left.iter()
                                    .map(|rule_id| self.rules.get(rule_id).unwrap())
                                    .rev(),
                            );
                            queue.push_back((left_rules, substr));
                            let mut right_rules = rules;
                            right_rules.extend(
                                right
                                    .iter()
                                    .map(|rule_id| self.rules.get(rule_id).unwrap())
                                    .rev(),
                            );
                            queue.push_back((right_rules, substr));
                        }
                    }
                } else if substr.is_empty() {
                    return true;
                }
            }

            false
        }
    }
    impl std::str::FromStr for Rules {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut rules = HashMap::new();
            for rule in s.lines() {
                let mut rule = rule.split(": ");
                let rule_id = rule.next().unwrap().parse().unwrap();
                let rule_matches = rule.next().unwrap();
                if &rule_matches[..1] == "\"" {
                    rules.insert(rule_id, Rule::Single(rule_matches.chars().nth(1).unwrap()));
                } else if rule_matches.contains(" | ") {
                    let mut rule = rule_matches.split(" | ");
                    let left = rule
                        .next()
                        .unwrap()
                        .split(' ')
                        .map(|n| n.parse().unwrap())
                        .collect();
                    let right = rule
                        .next()
                        .unwrap()
                        .split(' ')
                        .map(|n| n.parse().unwrap())
                        .collect();
                    rules.insert(rule_id, Rule::Alt(left, right));
                } else {
                    rules.insert(
                        rule_id,
                        Rule::Array(
                            rule_matches
                                .split(' ')
                                .map(|n| n.parse().unwrap())
                                .collect(),
                        ),
                    );
                }
            }
            Ok(Self { rules })
        }
    }
    pub fn part1() {
        let mut input = include_str!("../inputs/day19.txt").split("\r\n\r\n");
        let rules: Rules = input.next().unwrap().parse().unwrap();
        let messages: Vec<&str> = input.next().unwrap().lines().collect();
        println!(
            "{}",
            messages.iter().filter(|msg| rules.matches(msg)).count()
        );
    }
    pub fn part2() {
        let mut input = include_str!("../inputs/day19.txt").split("\r\n\r\n");
        let rules: Rules = {
            let mut rules: Rules = input.next().unwrap().parse().unwrap();
            rules.rules.insert(8, Rule::Alt(vec![42], vec![42, 8]));
            rules
                .rules
                .insert(11, Rule::Alt(vec![42, 31], vec![42, 11, 31]));
            rules
        };
        let messages: Vec<&str> = input.next().unwrap().lines().collect();
        println!(
            "{}",
            messages.iter().filter(|msg| rules.matches(msg)).count()
        );
    }
    #[test]
    fn parse_messages() {
        let rules: Rules = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"
"#
        .parse()
        .unwrap();
        let messages = [
            ("ababbb", true),
            ("bababa", false),
            ("abbbab", true),
            ("aaabbb", false),
            ("aaaabbb", false),
        ];
        for &(msg, valid) in &messages {
            assert_eq!(rules.matches(msg), valid);
        }
    }
}
mod day20 {
    use std::collections::{HashMap, HashSet};

    fn sqrt(n: usize) -> usize {
        let mut i = 0;
        while i * i <= n {
            if i * i == n {
                return i;
            }
            i += 1
        }
        panic!("not a square")
    }
    #[derive(Debug, Clone)]
    struct Tile {
        id: u64,
        grid: Vec<Vec<bool>>,
    }
    struct Sides {
        top: usize,
        bot: usize,
        left: usize,
        right: usize,
        top_inv: usize,
        bot_inv: usize,
        left_inv: usize,
        right_inv: usize,
    }
    impl Sides {
        fn contains(&self, val: usize) -> bool {
            let Sides {
                top,
                bot,
                left,
                right,
                top_inv,
                bot_inv,
                left_inv,
                right_inv,
            } = self;
            let val = &val;
            top == val
                || bot == val
                || left == val
                || right == val
                || top_inv == val
                || bot_inv == val
                || left_inv == val
                || right_inv == val
        }
    }
    enum TileSide {
        Corner,
        Edge,
        Centre,
    }
    impl Tile {
        fn len(&self) -> usize {
            self.grid.len()
        }
        fn allsides(&self) -> Sides {
            let top = self.top();
            let left = self.left();
            let right = self.right();
            let bot = self.bottom();
            let top_inv = count(&self.grid[0].iter().copied().rev().collect::<Vec<_>>());
            let left_inv = count(&self.grid.iter().map(|row| row[0]).rev().collect::<Vec<_>>());
            let right_inv = count(
                &self
                    .grid
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .rev()
                    .collect::<Vec<_>>(),
            );
            let bot_inv = count(
                &self.grid[self.len() - 1]
                    .iter()
                    .copied()
                    .rev()
                    .collect::<Vec<_>>(),
            );

            Sides {
                top,
                bot,
                left,
                right,
                top_inv,
                bot_inv,
                left_inv,
                right_inv,
            }
        }
        fn top(&self) -> usize {
            count(&self.grid[0])
        }
        fn left(&self) -> usize {
            count(&self.grid.iter().map(|row| row[0]).collect::<Vec<_>>())
        }
        fn right(&self) -> usize {
            count(
                &self
                    .grid
                    .iter()
                    .map(|row| row[row.len() - 1])
                    .collect::<Vec<_>>(),
            )
        }
        fn bottom(&self) -> usize {
            count(&self.grid[self.len() - 1])
        }
        fn rotate_cw(&mut self) {
            self.grid = (0..self.len())
                .map(|i| self.grid.iter().rev().map(|row| row[i]).collect())
                .collect();
        }
        fn flip_right(&mut self) {
            self.grid.iter_mut().for_each(|row| row.reverse());
        }
        fn flip_down(&mut self) {
            self.grid.reverse();
        }
        fn strip_borders(&mut self) {
            self.grid = self.grid[1..self.grid.len() - 1]
                .iter()
                .map(|row| row[1..row.len() - 1].to_vec())
                .collect();
        }
        fn position(&self, map: &HashMap<usize, i32>) -> TileSide {
            match [self.top(), self.bottom(), self.left(), self.right()]
                .iter()
                .filter(|edge| map[edge] == 1)
                .count()
            {
                0 => TileSide::Centre,
                1 => TileSide::Edge,
                2 => TileSide::Corner,
                _ => panic!(),
            }
        }
        fn rotate_until<T: Fn(&Tile) -> bool>(mut self, cond: T) -> Self {
            for _ in 0..4 {
                if cond(&self) {
                    return self;
                } else {
                    self.rotate_cw();
                }
            }
            self.flip_down();
            for _ in 0..4 {
                if cond(&self) {
                    return self;
                } else {
                    self.rotate_cw();
                }
            }
            panic!("condition unfufillable")
        }
    }
    fn count(nums: &[bool]) -> usize {
        nums.iter()
            .fold(0, |acc, i| if *i { (acc << 1) + 1 } else { acc << 1 })
    }
    impl std::str::FromStr for Tile {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut grid = s.lines();
            let id = grid
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse()
                .unwrap();
            let grid = grid
                .map(|row| row.as_bytes().iter().map(|tile| *tile == b'#').collect())
                .collect();

            Ok(Self { id, grid })
        }
    }
    pub fn part1() {
        let input: Vec<Tile> = include_str!("../inputs/day20.txt")
            .split("\r\n\r\n")
            .filter(|tile| !tile.is_empty())
            .map(|grid| grid.parse().unwrap())
            .collect();

        let mut seen = HashMap::new();
        for tile in &input {
            let Sides {
                top,
                bot,
                left,
                right,
                top_inv,
                bot_inv,
                left_inv,
                right_inv,
            } = tile.allsides();
            let sides = [top, bot, left, right, top_inv, bot_inv, left_inv, right_inv];
            for &i in &sides {
                *seen.entry(i).or_insert(0) += 1;
            }
        }

        println!(
            "{}",
            input
                .iter()
                .filter_map(|tile| {
                    let Sides {
                        top,
                        bot,
                        left,
                        right,
                        top_inv,
                        bot_inv,
                        left_inv,
                        right_inv,
                    } = tile.allsides();
                    if [top, bot, left, right, top_inv, bot_inv, left_inv, right_inv]
                        .iter()
                        .filter(|side| seen.get(side) == Some(&2))
                        .count()
                        == 2 * 2
                    // each edge is counted twice, once forward and once backward (flipped)
                    {
                        Some(tile.id)
                    } else {
                        None
                    }
                })
                .product::<u64>()
        );
    }
    struct Image {
        tiles: Vec<Vec<bool>>,
    }
    impl Image {
        fn new(mut tiles: Vec<Tile>) -> Self {
            let side_length = sqrt(tiles.len());
            let mut grid: Vec<Vec<Tile>> = vec![vec![]; side_length];
            let mut seen = HashMap::new();
            for tile in &tiles {
                let Sides {
                    top,
                    bot,
                    left,
                    right,
                    top_inv,
                    bot_inv,
                    left_inv,
                    right_inv,
                } = tile.allsides();
                let sides = [top, bot, left, right, top_inv, bot_inv, left_inv, right_inv];
                for &i in &sides {
                    *seen.entry(i).or_insert(0) += 1;
                }
            }
            let queue: Vec<(usize, usize)> = (0..side_length)
                .flat_map(|x| (0..side_length).map(|y| (x, y)).collect::<Vec<_>>())
                .collect();

            for (x, y) in queue {
                match (x, y) {
                    (0, 0) => {
                        //top left corner
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Corner) {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let corner = tiles.swap_remove(pos).rotate_until(|tile| {
                            seen.get(&tile.right()) == Some(&2)
                                && seen.get(&tile.bottom()) == Some(&2)
                        }); // rotate until sides are facing outwards

                        grid[0].push(corner);
                    }
                    (0, y) if y == side_length - 1 => {
                        // bottom left corner
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Corner)
                                    && tile.allsides().contains(grid[y - 1][0].bottom())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let corner = tiles.swap_remove(pos).rotate_until(|tile| {
                            seen.get(&tile.right()) == Some(&2)
                                && tile.top() == grid[y - 1][0].bottom()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(corner);
                    }
                    (x, y @ 0) if x == side_length - 1 => {
                        // top right corner
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Corner)
                                    && tile.allsides().contains(grid[y][x - 1].right())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let corner = tiles.swap_remove(pos).rotate_until(|tile| {
                            seen.get(&tile.bottom()) == Some(&2)
                                && tile.left() == grid[0][x - 1].right()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(corner);
                    }
                    (x, y) if x == side_length - 1 && y == side_length - 1 => {
                        // bottom right corner
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Corner)
                                    && tile.allsides().contains(grid[y][x - 1].right())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let corner = tiles.swap_remove(pos).rotate_until(|tile| {
                            tile.left() == grid[y][x - 1].right()
                                && tile.top() == grid[y - 1][x].bottom()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(corner);
                    }
                    (x, y) if x >= side_length || y >= side_length => {
                        // outside Image
                        unreachable!();
                    }
                    (0, y) => {
                        //left edge
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Edge)
                                    && tile.allsides().contains(grid[y - 1][0].bottom())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let edge = tiles.swap_remove(pos).rotate_until(|tile| {
                            seen.get(&tile.right()) == Some(&2)
                                && tile.top() == grid[y - 1][0].bottom()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(edge);
                    }
                    (x, 0) => {
                        //top edge
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Edge)
                                    && tile.allsides().contains(grid[0][x - 1].right())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let edge = tiles.swap_remove(pos).rotate_until(|tile| {
                            seen.get(&tile.right()) == Some(&2)
                                && seen.get(&tile.bottom()) == Some(&2)
                                && tile.left() == grid[0][x - 1].right()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(edge);
                    }
                    (x, y) if x == side_length - 1 => {
                        // right edge
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Edge)
                                    && tile.allsides().contains(grid[y][x - 1].right())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let edge = tiles.swap_remove(pos).rotate_until(|tile| {
                            seen.get(&tile.bottom()) == Some(&2)
                                && tile.left() == grid[y][x - 1].right()
                                && tile.top() == grid[y - 1][x].bottom()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(edge);
                    }
                    (x, y) if y == side_length - 1 => {
                        // bottom edge
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Edge)
                                    && tile.allsides().contains(grid[y][x - 1].right())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let edge = tiles.swap_remove(pos).rotate_until(|tile| {
                            seen.get(&tile.right()) == Some(&2)
                                && tile.left() == grid[y][x - 1].right()
                                && tile.top() == grid[y - 1][x].bottom()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(edge);
                    }
                    (x, y) => {
                        //middle tiles
                        let pos = tiles
                            .iter()
                            .enumerate()
                            .find_map(|(i, tile)| {
                                if matches!(tile.position(&seen), TileSide::Centre)
                                    && tile.allsides().contains(grid[y][x - 1].right())
                                {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        let tile = tiles.swap_remove(pos).rotate_until(|tile| {
                            tile.left() == grid[y][x - 1].right()
                                && tile.top() == grid[y - 1][x].bottom()
                        }); // rotate until sides are facing correct directions

                        grid[y].push(tile);
                    }
                };
            }

            grid.iter_mut()
                .for_each(|row| row.iter_mut().for_each(Tile::strip_borders));

            let mut tiles = Vec::new();
            for row in grid {
                for i in 0..row[0].grid.len() {
                    tiles.push(row.iter().flat_map(|tile| &tile.grid[i]).copied().collect());
                }
            }
            Self { tiles }
        }
        fn count_monster(&self, monster: &[Vec<u8>]) -> usize {
            let (monster_pixels, width, height): (Vec<(usize, usize)>, usize, usize) = {
                (
                    monster
                        .iter()
                        .enumerate()
                        .flat_map(|(y, row)| {
                            row.iter()
                                .enumerate()
                                .filter_map(
                                    |(x, pixel)| {
                                        if pixel == &b'#' {
                                            Some((x, y))
                                        } else {
                                            None
                                        }
                                    },
                                )
                                .collect::<Vec<_>>()
                        })
                        .collect(),
                    monster[0].len(),
                    monster.len(),
                )
            };

            let mut part_of_monster = HashSet::new();

            for x in 0..(self.tiles[0].len() - width) {
                for y in 0..(self.tiles.len() - height) {
                    if monster_pixels
                        .iter()
                        .all(|&(delta_x, delta_y)| self.tiles[y + delta_y][x + delta_x])
                    {
                        for &(delta_x, delta_y) in &monster_pixels {
                            part_of_monster.insert((x + delta_x, y + delta_y));
                        }
                    }
                }
            }
            part_of_monster.len()
        }
        fn roughness(&self) -> usize {
            // rotate image of monster instead
            let mut monster = vec![
                b"                  # ".to_vec(),
                b"#    ##    ##    ###".to_vec(),
                b" #  #  #  #  #  #   ".to_vec(),
            ];
            let rotate = |monster: &mut Vec<Vec<u8>>| {
                let new_monster = (0..monster[0].len())
                    .map(|i| (0..monster.len()).rev().map(|j| monster[j][i]).collect())
                    .collect();
                *monster = new_monster;
            };
            let rough_tiles: usize = self
                .tiles
                .iter()
                .map(|row| row.iter().filter(|tile| **tile).count())
                .sum();

            for _ in 0..4 {
                let count = self.count_monster(&monster);
                if count > 0 {
                    return rough_tiles - count;
                } else {
                    rotate(&mut monster);
                }
            }
            // flip the grid
            monster.reverse();
            for _ in 0..4 {
                let count = self.count_monster(&monster);
                if count > 0 {
                    return rough_tiles - count;
                } else {
                    rotate(&mut monster);
                }
            }
            panic!("no monsters found")
        }
    }
    pub fn part2() {
        let input: Vec<Tile> = include_str!("../inputs/day20.txt")
            .split("\r\n\r\n")
            .filter(|tile| !tile.is_empty())
            .map(|grid| grid.parse().unwrap())
            .collect();

        let image = Image::new(input);
        println!("{}", image.roughness())
    }
}
mod day21 {
    use std::collections::{HashMap, HashSet};
    struct Food<'a> {
        ingredients: Vec<&'a str>,
        allergerns: Vec<&'a str>,
    }
    impl<'a> Food<'a> {
        fn parse(s: &'a str) -> Self {
            let mut s = s.split(" (contains ");
            let ingredients = s.next().unwrap().split(' ').collect();
            let allergerns = s
                .next()
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ")
                .collect();

            Food {
                ingredients,
                allergerns,
            }
        }
    }
    fn intersection(set1: &mut Vec<&str>, set2: &[&str]) {
        set1.retain(|x| set2.contains(x));
    }
    pub fn part1() {
        let input: Vec<Food> = include_str!("../inputs/day21.txt")
            .lines()
            .map(|line| Food::parse(line))
            .collect();
        let mut possible = HashMap::new();
        for food in &input {
            for allergen in &food.allergerns {
                let pos = possible
                    .entry(allergen)
                    .or_insert_with(|| food.ingredients.clone());
                intersection(pos, &food.ingredients);
            }
        }
        let mut confirmed_allergens = HashSet::new();
        while !possible.is_empty() {
            possible.retain(|_key, value| {
                value.retain(|ingredient| !confirmed_allergens.contains(ingredient));
                if value.len() == 1 {
                    confirmed_allergens.insert(value[0]);
                    false
                } else {
                    true
                }
            });
        }

        println!(
            "{}",
            input
                .iter()
                .map(|food| food
                    .ingredients
                    .iter()
                    .filter(|ingredient| !confirmed_allergens.contains(*ingredient))
                    .count())
                .sum::<usize>()
        )
    }
    pub fn part2() {
        let input: Vec<Food> = include_str!("../inputs/day21.txt")
            .lines()
            .map(|line| Food::parse(line))
            .collect();
        let mut possible = HashMap::new();
        for food in &input {
            for allergen in &food.allergerns {
                let pos = possible
                    .entry(allergen)
                    .or_insert_with(|| food.ingredients.clone());
                intersection(pos, &food.ingredients);
            }
        }
        let mut confirmed_allergens = possible
            .values()
            .filter_map(|possibles| {
                if possibles.len() == 1 {
                    Some(possibles[0])
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();
        while possible.values().any(|possibles| possibles.len() > 1) {
            possible
                .values_mut()
                .filter(|possibles| possibles.len() > 1)
                .for_each(|possibles| {
                    possibles.retain(|pos| !confirmed_allergens.contains(pos));
                    if possibles.len() == 1 {
                        confirmed_allergens.insert(possibles[0]);
                    }
                });
        }
        let mut dangerous_ingredient_list: Vec<&str> = possible.keys().map(|k| **k).collect();
        dangerous_ingredient_list.sort_unstable();
        for allergen in &mut dangerous_ingredient_list {
            *allergen = possible[allergen][0];
        }

        println!("{}", dangerous_ingredient_list.join(","));
    }
}
mod day22 {
    use std::collections::{HashMap, HashSet, VecDeque};
    fn score(deck: &VecDeque<usize>) -> usize {
        (1..=deck.len())
            .zip(deck.iter().rev())
            .map(|(i, &j)| i * j)
            .sum::<usize>()
    }
    pub fn part1() {
        let mut input = include_str!("../inputs/day22.txt")
            .split("\r\n\r\n")
            .filter(|string| !string.is_empty());

        let mut player1: VecDeque<usize> = input
            .next()
            .unwrap()
            .strip_prefix("Player 1:")
            .unwrap()
            .trim()
            .lines()
            .map(|n| n.parse().unwrap())
            .collect();
        let mut player2: VecDeque<usize> = input
            .next()
            .unwrap()
            .strip_prefix("Player 2:")
            .unwrap()
            .trim()
            .lines()
            .map(|n| n.parse().unwrap())
            .collect();

        while !(player1.is_empty() || player2.is_empty()) {
            let card1 = player1.pop_front().unwrap();
            let card2 = player2.pop_front().unwrap();
            if card1 > card2 {
                player1.push_back(card1);
                player1.push_back(card2);
            } else {
                player2.push_back(card2);
                player2.push_back(card1);
            }
        }

        let winner = if player1.is_empty() {
            &player2
        } else {
            &player1
        };
        println!("{}", score(winner));
    }
    #[derive(Debug, Clone)]
    enum Winner {
        Player1(VecDeque<usize>),
        Player2(VecDeque<usize>),
    }
    impl Winner {
        fn take_cards(self) -> VecDeque<usize> {
            match self {
                Winner::Player1(cards) | Winner::Player2(cards) => cards,
            }
        }
    }
    fn recursive_combat(
        p1: &VecDeque<usize>,
        p2: &VecDeque<usize>,
        memo: &mut HashMap<(VecDeque<usize>, VecDeque<usize>), Winner>,
    ) -> Winner {
        if memo.contains_key(&(p1.clone(), p2.clone())) {
            return memo[&(p1.clone(), p2.clone())].clone();
        }
        // true if player1 won, false otherwise
        let mut player1 = p1.clone();
        let mut player2 = p2.clone();
        let mut seen = HashSet::new();

        while !(player1.is_empty() || player2.is_empty()) {
            if seen.contains(&(player1.clone(), player2.clone())) {
                return Winner::Player1(player1);
            } else {
                seen.insert((player1.clone(), player2.clone()));
            }
            let card1 = player1.pop_front().unwrap();
            let card2 = player2.pop_front().unwrap();
            let recursive_winner = if card1 <= player1.len() && card2 <= player2.len() {
                Some(matches!(
                    recursive_combat(
                        &player1.iter().take(card1).copied().collect(),
                        &player2.iter().take(card2).copied().collect(),
                        memo
                    ),
                    Winner::Player1(_)
                ))
            } else {
                None
            };

            if recursive_winner.unwrap_or_else(|| card1 > card2) {
                player1.push_back(card1);
                player1.push_back(card2);
            } else {
                player2.push_back(card2);
                player2.push_back(card1);
            };
        }

        let winner = if player2.is_empty() {
            Winner::Player1(player1)
        } else {
            Winner::Player2(player2)
        };
        memo.insert((p1.clone(), p2.clone()), winner.clone());

        winner
    }
    pub fn part2() {
        let mut input = include_str!("../inputs/day22.txt")
            .split("\r\n\r\n")
            .filter(|string| !string.is_empty());

        let player1: VecDeque<usize> = input
            .next()
            .unwrap()
            .strip_prefix("Player 1:")
            .unwrap()
            .trim()
            .lines()
            .map(|n| n.parse().unwrap())
            .collect();
        let player2: VecDeque<usize> = input
            .next()
            .unwrap()
            .strip_prefix("Player 2:")
            .unwrap()
            .trim()
            .lines()
            .map(|n| n.parse().unwrap())
            .collect();

        let winner = recursive_combat(&player1, &player2, &mut HashMap::new()).take_cards();
        println!("{}", score(&winner));
    }
}
mod day23 {
    struct CrabCups {
        current: usize,
        next_cup: Vec<usize>,
    }
    impl CrabCups {
        fn new(vec: &[u32]) -> Self {
            let mut next_cup = vec![0; vec.len() + 1];
            next_cup[0] = vec[0] as usize;
            next_cup[*vec.last().unwrap() as usize] = vec[0] as usize;
            for i in vec.windows(2) {
                if let [first, second] = i {
                    next_cup[*first as usize] = *second as usize;
                }
            }
            Self {
                current: 0,
                next_cup,
            }
        }
        fn play(&mut self, moves: usize) {
            let min_label = 1;
            let max_label = self.next_cup.len() - 1;

            for _ in 0..moves {
                let current_cup = self.next_cup[self.current];

                let first = self.next_cup[current_cup];
                let second = self.next_cup[first];
                let third = self.next_cup[second];

                let after_third = self.next_cup[third];
                let mut destination_label = current_cup - 1;
                loop {
                    if destination_label < min_label {
                        destination_label = max_label;
                    } else if [first, second, third].contains(&destination_label) {
                        destination_label -= 1
                    } else {
                        break;
                    }
                }
                let after_destination = self.next_cup[destination_label];
                self.next_cup[destination_label] = first;
                self.next_cup[third] = after_destination;

                self.next_cup[current_cup] = after_third;
                self.current = current_cup;
            }
        }
        fn part1(&self) -> u32 {
            let mut acc = 0;
            let mut current = self.next_cup[1];
            while current != 1 {
                acc = acc * 10 + current;
                current = self.next_cup[current];
            }
            acc as u32
        }
        fn part2(&self) -> u64 {
            (self.next_cup[1] * self.next_cup[self.next_cup[1]]) as u64
        }
    }
    pub fn part1() {
        const LOOPS: usize = 100;
        let mut cups = CrabCups::new(
            &include_str!("../inputs/day23.txt")
                .trim()
                .bytes()
                .map(|chr| u32::from(chr - b'0'))
                .collect::<Vec<_>>(),
        );

        cups.play(LOOPS);

        println!("{}", cups.part1());
    }
    pub fn part2() {
        const LOOPS: usize = 10_000_000;
        let mut cups = CrabCups::new(
            &include_str!("../inputs/day23.txt")
                .trim()
                .bytes()
                .map(|chr| u32::from(chr - b'0'))
                .chain((include_str!("../inputs/day23.txt").trim().len() + 1) as u32..=1_000_000)
                .collect::<Vec<_>>(),
        );

        cups.play(LOOPS);

        println!("{}", cups.part2());
    }
}
mod day24 {
    use std::collections::HashMap;

    #[derive(Debug)]
    enum Direction {
        East,
        SouthEast,
        SouthWest,
        West,
        NorthWest,
        NorthEast,
    }
    #[derive(Debug)]
    struct Path(Vec<Direction>);
    impl std::str::FromStr for Path {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut path = Vec::new();
            let mut s = s.bytes();
            while let Some(dir) = s.next() {
                match dir {
                    b'e' => path.push(Direction::East),
                    b'w' => path.push(Direction::West),
                    b's' => match s.next() {
                        Some(b'e') => path.push(Direction::SouthEast),
                        Some(b'w') => path.push(Direction::SouthWest),
                        _ => panic!("Expected 'e' or 'w'"),
                    },
                    b'n' => match s.next() {
                        Some(b'e') => path.push(Direction::NorthEast),
                        Some(b'w') => path.push(Direction::NorthWest),
                        _ => panic!("Expected 'e' or 'w'"),
                    },
                    _ => panic!("Expected 'e' or 'w' or 's' or 'n'"),
                }
            }

            Ok(Self(path))
        }
    }
    #[derive(Debug, Clone)]
    enum Colour {
        Black,
        White,
    }
    impl Colour {
        fn flip(&mut self) {
            *self = match self {
                Colour::Black => Colour::White,
                Colour::White => Colour::Black,
            }
        }
    }
    struct Grid {
        mid: usize,
        grid: Vec<Vec<Colour>>,
    }
    impl Grid {
        fn new(tiles: HashMap<(i32, i32), bool>, len: usize) -> Self {
            let mut grid = vec![vec![Colour::White; len]; len];
            let mid = (len / 2) as i32;
            for ((x, y), is_black) in tiles {
                grid[(mid + y) as usize][(mid + x) as usize] = if is_black {
                    Colour::Black
                } else {
                    Colour::White
                };
            }
            Self {
                mid: mid as usize,
                grid,
            }
        }
        fn flip(&mut self, times: usize) {
            for _ in 0..times {
                let mut flip = Vec::new();
                for x in 0..self.grid[0].len() {
                    for y in 0..self.grid.len() {
                        if (x + y) % 2 != 0 {
                            // if x+y is not divisible by 2 -> not on hexagon grid
                            continue;
                        }
                        let adjacent = self
                            .neighbours((x, y))
                            .iter()
                            .filter(|(x, y)| matches!(self.grid[*y][*x], Colour::Black))
                            .count();

                        match self.grid[y][x] {
                            Colour::Black if !((1..=2).contains(&adjacent)) => flip.push((x, y)),
                            Colour::White if adjacent == 2 => flip.push((x, y)),
                            _ => {}
                        }
                    }
                }
                for (x, y) in flip {
                    self.grid[y][x].flip();
                }
            }
        }
        fn black_tiles(&self) -> Vec<(usize, usize)> {
            let mut tiles = Vec::new();
            for y in 0..self.grid.len() {
                for x in 0..self.grid[0].len() {
                    if matches!(self.grid[y][x], Colour::Black) {
                        tiles.push((x, y))
                    }
                }
            }
            tiles
        }
        fn neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
            let mut neighbours = Vec::with_capacity(6);
            if x >= 2 {
                neighbours.push((x - 2, y))
            }
            if x + 2 < self.grid[0].len() {
                neighbours.push((x + 2, y))
            }
            if x >= 1 {
                if y >= 1 {
                    neighbours.push((x - 1, y - 1))
                }
                if y + 1 < self.grid.len() {
                    neighbours.push((x - 1, y + 1))
                }
            }
            if x + 1 < self.grid[0].len() {
                if y >= 1 {
                    neighbours.push((x + 1, y - 1))
                }
                if y + 1 < self.grid.len() {
                    neighbours.push((x + 1, y + 1))
                }
            }
            neighbours
        }
    }
    pub fn part1() {
        let paths: Vec<Path> = include_str!("../inputs/day24.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let mut tiles: HashMap<(i32, i32), bool> = HashMap::new(); // true if black

        for steps in paths {
            let (mut x, mut y) = (0, 0);
            for dir in steps.0 {
                match dir {
                    Direction::East => x += 2,
                    Direction::SouthEast => {
                        x += 1;
                        y -= 1;
                    }
                    Direction::SouthWest => {
                        x -= 1;
                        y -= 1;
                    }
                    Direction::West => x -= 2,
                    Direction::NorthWest => {
                        x -= 1;
                        y += 1;
                    }
                    Direction::NorthEast => {
                        x += 1;
                        y += 1;
                    }
                }
            }
            let colour = tiles.entry((x, y)).or_insert(false);
            *colour = !*colour;
        }

        println!("{}", tiles.values().filter(|tile| **tile).count());
    }
    pub fn part2() {
        const DAYS: usize = 100;
        let paths: Vec<Path> = include_str!("../inputs/day24.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let mut tiles: HashMap<(i32, i32), bool> = HashMap::new(); // true if black

        for steps in paths {
            let (mut x, mut y) = (0, 0);
            for dir in steps.0 {
                match dir {
                    Direction::East => x += 2,
                    Direction::SouthEast => {
                        x += 1;
                        y -= 1;
                    }
                    Direction::SouthWest => {
                        x -= 1;
                        y -= 1;
                    }
                    Direction::West => x -= 2,
                    Direction::NorthWest => {
                        x -= 1;
                        y += 1;
                    }
                    Direction::NorthEast => {
                        x += 1;
                        y += 1;
                    }
                }
            }
            let colour = tiles.entry((x, y)).or_insert(false);
            *colour = !*colour;
        }
        let max_dist = tiles
            .keys()
            .map(|&(x, y)| {
                let x = x.abs();
                let y = y.abs();
                if x > y {
                    x
                } else {
                    y
                }
            })
            .max()
            .unwrap() as usize;
        let length = 2 * DAYS + 2 * max_dist;

        let mut grid = Grid::new(tiles, length);
        grid.flip(DAYS);

        println!("{}", grid.black_tiles().len());
    }
}
mod day25 {
    fn pub_key_to_loop_size(key: usize) -> usize {
        let mut loops = 0;
        let subject_number = 7;
        let mut current = 1;
        while key != current {
            current *= subject_number;
            current %= 20_201_227;
            loops += 1;
        }
        loops
    }
    fn form_encryption_key(key: usize, loop_size: usize) -> usize {
        let subject_number = key;
        let mut current = 1;
        for _ in 0..loop_size {
            current *= subject_number;
            current %= 20_201_227;
        }
        current
    }
    pub fn part1() {
        let input: Vec<usize> = include_str!("../inputs/day25.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        let key1 = input[0];
        let key2 = input[1];

        let loop_1 = pub_key_to_loop_size(key1);
        let loop_2 = pub_key_to_loop_size(key2);

        println!("{}", form_encryption_key(key2, loop_1));
        println!("{}", form_encryption_key(key1, loop_2));
    }
}
