use std::fmt::Display;

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace().map(|num| num.parse().unwrap()).collect()
}

fn is_safe(v: &Vec<u32>) -> bool {
    let strictly_monotone = v.windows(2).all(|w| w[0] < w[1]) || v.windows(2).all(|w| w[0] > w[1]);
    let gradual = v.windows(2).all(|w| (w[0] as i32 - w[1] as i32).abs() >= 1 && (w[0] as i32 - w[1] as i32).abs() <= 3);

    strictly_monotone && gradual
}

fn one_removed(v: &Vec<u32>) -> Vec<Vec<u32>> {
    (0..v.len()).map(|i| {
            let mut v = v.clone();
            v.remove(i);
            v
        }).collect()
}

fn is_safe_2(v: &Vec<u32>) -> bool {
    is_safe(v) || one_removed(v).iter().any(|vec| is_safe(vec))
}

fn check_safety(s: &str, f: fn(&Vec<u32>) -> bool) -> usize {
    s.lines()
        .map(parse_line)
        .filter(f)
        .count()
}

pub fn part1(s: &str) -> impl Display {
    check_safety(s, is_safe)
}

pub fn part2(s: &str) -> impl Display {
    check_safety(s, is_safe_2)
}