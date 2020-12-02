fn main() {
    day2::part2()
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
