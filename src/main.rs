fn main() {
    day12::part2();
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
