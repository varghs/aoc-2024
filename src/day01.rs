use std::{collections::{HashMap, HashSet}, fmt::Display};
pub fn part1(s: &str) -> impl Display {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = s.lines().map(|line| line.split_whitespace())
        .map(|mut split_iter| (
            split_iter.next().unwrap().parse::<i32>().unwrap(), 
            split_iter.next().unwrap().parse::<i32>().unwrap()
        )).unzip();
    left.sort();
    right.sort();
    
    left.into_iter().zip(right.into_iter()).map(|(l, r)| (l - r).abs()).sum::<i32>()
}

pub fn part2(s: &str) -> impl Display {
    let (left, right): (HashSet<i32>, HashMap<i32, u32>) = s.lines().map(|line| line.split_whitespace())
        .map(|mut split_iter| (
            split_iter.next().unwrap().parse::<i32>().unwrap(), 
            split_iter.next().unwrap().parse::<i32>().unwrap()
        ))
        .fold(
            (HashSet::new(), HashMap::new()),
            |(mut set, mut counter), (l, r)| {
                set.insert(l);
                *counter.entry(r).or_insert(0) += 1;
                (set, counter)
            }
        );
    
    left.into_iter().map(|num| num * *right.get(&num).unwrap_or(&0) as i32).sum::<i32>()
}