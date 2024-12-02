#![allow(unused_imports)]
#![allow(dead_code)]
use std::char::EscapeDebug;
use std::collections::*;
use std::fmt::{LowerHex, format};
use std::process::Child;
use std::{fs,env};
use std::error::Error;
//use reqwest;
use soup::prelude::*;
use std::time::Instant;
use regex::Regex;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::cmp::Reverse;
use std::fs::File;
use std::io::Write;
use rand::prelude::*;

fn get_text(day: i32,sample:bool,part:usize) -> Result<String, Box<dyn Error>> {
    let path = format!("data/day{day}.txt");
    let sample_path = format!("data/day{day}sample{part}.txt");
    let year = 2024;
    match sample {
        false => {
            if let Ok(text) = fs::read_to_string(path.clone()) { return Ok(text)}
            let url = format!("https://adventofcode.com/{year}/day/{day}/input");
            let text = reqwest::blocking::Client::new().get(url).header("cookie",format!("session={}",env::var("AOC_SESSION").unwrap())).send()?.text()?.trim().to_string();
            fs::write(path, text.clone())?;
            Ok(text)
        },
        true => {
            if let Ok(text) = fs::read_to_string(sample_path.clone()) { return Ok(text) }
            let url = format!("https://adventofcode.com/{year}/day/{day}");
            let html_text = reqwest::blocking::Client::new().get(url).header("cookie",format!("session={}",env::var("AOC_SESSION").unwrap())).send()?.text()?;
            let text = &Soup::new(html_text.as_str()).tag("pre").find_all().map(|tag| {tag.text().trim().to_string()}).nth(part - 1).unwrap();
            fs::write(sample_path, text.clone())?;
            Ok(text.clone())
        }  
    }
}
fn day1() -> Result<(), Box<dyn Error>> {
    let text = get_text(1,false,1)?;
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in text.split('\n') {
        let mut it = line.split_whitespace();
        left.push(it.next().unwrap().parse()?);
        right.push(it.next().unwrap().parse()?);
    }
    left.sort_unstable();
    right.sort_unstable();
    let ans1:i64 = left.iter().zip(right.iter()).map(|(a,b)| (a-b).abs()).sum();
    println!("part 1: {:?}", ans1);
    let mut counts = HashMap::new();
    for n in right {counts.entry(n).and_modify(|ct| *ct += 1).or_insert(1);}
    let ans2:i64 = left.iter().map(|n| if let Some(&ct) = counts.get(n) {*n * ct} else {0}).sum();
    println!("part 2: {:?}", ans2);
    Ok(())
}
fn day2() -> Result<(), Box<dyn Error>> {
    fn is_safe(v:&[i32]) -> bool {
        v.windows(2).map(|x| x[0]-x[1]).all(|y| y==1 || y==2 || y==3) ||
        v.windows(2).map(|x| x[0]-x[1]).all(|y| y==-1 || y==-2 || y==-3)
    }
    let text = get_text(2,false,1)?;
    let ans1:i32 = text.split('\n').map(|line| if is_safe(&line.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>()) {1} else {0}).sum();
    println!("part1: {:?}", ans1);
    fn is_safe2(v:&[i32]) -> bool {
        is_safe(v) || (0..v.len()).any(|i| is_safe(&v[..i].iter().chain(v[i+1..].iter()).map(|x| *x).collect::<Vec<i32>>()))
    }
    let ans2:i32 = text.split('\n').map(|line| if is_safe2(&line.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>()) {1} else {0}).sum();
    println!("part2: {:?}", ans2);
    Ok(())
}
fn main() {
    let now = Instant::now();
    let _ = day2();
    println!("Elapsed: {:.2?}", now.elapsed());
}
