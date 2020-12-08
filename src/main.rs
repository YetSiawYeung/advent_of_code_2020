fn main() {
    day8::part2()
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
