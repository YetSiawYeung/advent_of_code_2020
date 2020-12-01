fn main() {
    part2()
}
fn part1() {
    let input: Vec<u32> = include_str!("input.txt")
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
fn part2() {
    let input: Vec<u32> = include_str!("input.txt")
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
