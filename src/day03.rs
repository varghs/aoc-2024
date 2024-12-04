use std::fmt::Display;
use regex::Regex;

pub fn part1(s: &str) -> impl Display {
    let re_capture = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    s.lines()
    .map(|line| re_capture
        .captures_iter(line)
        .map(|c: regex::Captures<'_>| (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap())))
    .flatten()
    .map(|(x, y)| x * y).sum::<u32>()
}

pub fn part2(s: &str) -> impl Display {
    ""
}