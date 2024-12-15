#![allow(unused_imports)]
#![allow(dead_code)]
use std::char::EscapeDebug;
use std::collections::*;
use std::fmt::{LowerHex, format};
use std::fmt;
use std::process::Child;
use std::{fs,env,iter};
use std::error::Error;
//use reqwest;
use soup::prelude::*;
use std::time::Instant;
use regex::Regex;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{stdin, Write};
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
fn day3() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(3,false,1)?;
    let re = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    fn sum_up(text:String, re: Regex) -> i64 {
        re.captures_iter(&text).map(|caps| {
            let a: i64 = caps["a"].parse().unwrap();
            let b: i64 = caps["b"].parse().unwrap();
            a*b
        }).sum()
    }
    let ans1:i64 = sum_up(text, re.clone());
    println!("part1: {:?}", ans1);
    let mut text: String = get_text(3,false,1)?;
    let mut ans2:i64 = 0;
    while !text.is_empty() {
        match (text.rfind(r"do()"),text.rfind(r"don't()")) {
            (Some(x),Some(y)) if x < y => {
                text = text[..y].to_string();
            },
            (Some(x),Some(_)) => {
                ans2 += sum_up(text[x..].to_string(),re.clone());
                text = text[..x].to_string();
            },
            (_,Some(y)) => {
                text = text[..y].to_string();
            },
            _ => {
                ans2 += sum_up(text.clone(),re.clone());
                text.clear();
            }
        }
        //println!("{ans2} {text:?}");
    }
    println!("part2: {:?}", ans2);
    Ok(())
}
fn day4() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(4,false,1)?;
    let mut grid = text.split('\n').map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    fn look(i:usize, j: usize, word: &[char], grid: &mut [Vec<char>], tot: &mut i64) {
        let len = word.len() - 1;
        if i+len < grid.len() && j.wrapping_sub(len) < grid[0].len() && (0..word.len()).all(|d| word[d] == grid[i+d][j.wrapping_sub(d)]) {*tot += 1}
        if i+len < grid.len() && (0..word.len()).all(|d| word[d] == grid[i+d][j]) {*tot += 1}
        if i+len < grid.len() && j + len < grid[0].len() && (0..word.len()).all(|d| word[d] == grid[i+d][j+d]) {*tot += 1}
        if j.wrapping_sub(len) < grid[0].len() && (0..word.len()).all(|d| word[d] == grid[i][j.wrapping_sub(d)]) {*tot += 1}
        if j + len < grid[0].len() && (0..word.len()).all(|d| word[d] == grid[i][j+d]) {*tot += 1}
        if i.wrapping_sub(len) < grid.len() && j.wrapping_sub(len) < grid[0].len() && (0..word.len()).all(|d| word[d] == grid[i.wrapping_sub(d)][j.wrapping_sub(d)]) {*tot += 1}
        if i.wrapping_sub(len) < grid.len() && (0..word.len()).all(|d| word[d] == grid[i.wrapping_sub(d)][j]) {*tot += 1}
        if i.wrapping_sub(len) < grid.len() && j + len < grid[0].len() && (0..word.len()).all(|d| word[d] == grid[i.wrapping_sub(d)][j+d]) {*tot += 1}
    }
    let mut tot = 0;
    let word: Vec<char> = "XMAS".chars().collect();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                look(i,j, &word, &mut grid, &mut tot);
            }
        }
    }
    println!("part 1: {tot}"); 
    let mut tot2 = 0;
    for i in 1..grid.len()-1 {
        for j in 1..grid[0].len()-1 {
            if grid[i][j] == 'A' {
                match (grid[i-1][j-1],grid[i+1][j+1],grid[i+1][j-1],grid[i-1][j+1]) {
                    ('M','S','M','S') => tot2 += 1,
                    ('M','S','S','M') => tot2 += 1,
                    ('S','M','M','S') => tot2 += 1,
                    ('S','M','S','M') => tot2 += 1,
                    _ => ()
                }
            }
        }
    }
    println!("part 2: {tot2}");
    Ok(())
}
fn day5() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(5,false,1)?;
    let mut lines = text.split('\n');
    let mut rules = HashMap::new();
    while let Some(line) = lines.next() {
        let nums:Vec<i64> = line.split('|').filter_map(|x| x.parse::<i64>().ok()).collect();
        match &nums[..] {
            &[a,b] => {rules.entry(a).and_modify(|set: &mut HashSet<i64>| {set.insert(b);}).or_insert(HashSet::from([b]));},
            _ => break
        }
    }
    let mut pages = Vec::new();
    while let Some(line) = lines.next() {
        let nums:Vec<i64> = line.split(',').filter_map(|x| x.parse::<i64>().ok()).collect();
        pages.push(nums);
    }
    fn follows_rules(s: &[i64], rules: &HashMap<i64,HashSet<i64>>) -> bool {
        let mut seen = HashSet::new();
        for x in s {
            if let Some(set) = rules.get(x) {
                if seen.iter().any(|y| set.contains(y)) {
                    return false
                }
            }
            seen.insert(*x);
        }
        true
    }
    let ans1:i64 = pages.iter().filter_map(|s| if follows_rules(s,&rules) {Some(s[s.len()/2])} else {None}).sum();
    println!("part 1: {ans1}");
    fn sort_and_mid(s: &[i64], rules: &HashMap<i64, HashSet<i64>>) -> Option<i64> {
        let mut rel = HashMap::new();
        for &x in s {
            rel.insert(x, rules.get(&x).unwrap_or(&HashSet::new()).iter().filter_map(|y| if s.contains(y) {Some(*y)} else {None}).collect::<HashSet<i64>>());
        }
        let mut ordered = Vec::new();
        let mut ready: Vec<i64> = s.iter().filter_map(|x| if rel[x].len() == 0 {Some(*x)} else {None}).collect();
        while let Some(x) = ready.pop() {
            ordered.push(x);
            for (y,set) in rel.iter_mut() {
                if set.remove(&x) && set.len() == 0 {
                    ready.push(*y);
                } 
            }
        }
        Some(ordered[ordered.len() / 2])
    }
    let ans2:i64 = pages.iter().filter_map(|s| if follows_rules(s,&rules) {None} else {sort_and_mid(s,&rules)}).sum();
    println!("part 2: {ans2}");
    Ok(())
}
fn day6() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(6,false,1)?;
    #[derive(Clone,Hash,PartialEq,Eq)]
    enum Direction {Up,Right,Down,Left}
    impl Default for Direction {
        fn default() -> Self { Direction::Up }
    }
    impl Direction {
        fn turn(&mut self) {
            match self {
                Up => *self = Right,
                Right => *self = Down,
                Down => *self = Left,
                Left => *self = Up,
            }
        }
    }
    use Direction::*;
    #[derive(Clone,Hash,Default,PartialEq,Eq)]
    struct Position {
        x: usize,
        y: usize,
    }
    #[derive(PartialEq)]
    enum State {Ready,Out,Cycle}
    use State::*;
    #[derive(Default,Clone)]
    struct Guard {
        grid: Vec<Vec<char>>,
        pos: Position,
        init: Position,
        facing: Direction,
        seen: HashSet<(Position, Direction)>,
    }
    impl fmt::Display for Guard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut temp = self.grid.clone();
            for (p,_) in self.seen.iter() {
                temp[p.x][p.y] = 'X';
            }
            write!(f, "{}", temp.into_iter().map(|row| row.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"))
        }
    }
    impl Guard {
        fn new(text: &str) -> Self {
            let grid = text.split('\n').map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
            let mut x:usize=0; let mut y:usize=0; 
            'outer: for i in 0..grid.len() { for j in 0..grid[0].len() { if grid[i][j] == '^' {(x,y) = (i,j); break 'outer}}} 
            let seen = HashSet::from([(Position{x,y},Up)]);
            Guard{grid,pos:Position{x,y},init:Position{x,y},seen,..Default::default()}
        }
        fn peek(&self,p: &Position) -> Option<char> {
            if p.x < self.grid.len() && p.y < self.grid[0].len() {
                Some(self.grid[p.x][p.y])
            } else {
                None
            }
        }
        fn step(&self) -> Position {
            let mut p = self.pos.clone();
            match self.facing {
                Up => p.x = p.x.wrapping_sub(1),
                Right => p.y += 1,
                Down => p.x += 1,
                Left => p.y = p.y.wrapping_sub(1),
            }
            p
        }
        fn forward(&mut self) -> State {
            let p = self.step();
            match self.peek(&p) {
                None => Out,
                Some('#') => {self.facing.turn(); self.forward()}
                Some('.') if self.seen.contains(&(p.clone(),self.facing.clone())) => Cycle,
                _ => {self.seen.insert((p.clone(),self.facing.clone())); self.pos = p; Ready}
            }
        }
        fn unique_positions(&self) -> HashSet<Position> {
            self.seen.iter().map(|(p,_)| p.clone()).collect::<HashSet<_>>()
        }

    }
    let orig = Guard::new(&text);
    let mut guard = orig.clone();
    loop {
        if guard.forward() == Out {break}
        //println!("{guard} \n");
    }
    println!("part 1: {}", guard.unique_positions().len());
    let ans2:usize = guard.unique_positions().par_iter().map(|p| {
        let mut ans2 = 0;
        if p != &guard.init {
            let mut temp = orig.clone();
            temp.grid[p.x][p.y] = '#';
            loop {
                match temp.forward() {
                    Out => break,
                    Cycle => {ans2 += 1; break},
                    Ready => (),
                }
            }
        }
        ans2
        //println!("{guard} \n");
    }).sum();
    println!("part 2: {}", ans2);
    Ok(())
}
fn day7() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(7,false,1)?;
    let mut tests = Vec::new();
    for line in text.split('\n') {
        let mut test:(i64,Vec<i64>) = (0,Vec::new());
        let mut nums = line.split(": ");
        test.0 = nums.next().unwrap().parse()?;
        test.1 = nums.next().unwrap().split_whitespace().filter_map(|x| x.parse().ok()).collect();
        tests.push(test);
    }
    let ans1: i64 = tests.iter()
        .filter_map(|(val,v)| {
            if dfs(*val,v) {Some(val)} else {None}
        }).sum();
    fn dfs(val: i64, v: &[i64]) -> bool {
        if v.len() == 1 {
            val == v[0] 
        } else {
            let n = v.len() - 1;
            dfs(val - v[n], &v[..n]) || (val % v[n] == 0 && dfs(val / v[n], &v[..n])) 
        }
    }
    println!("part 1: {ans1}");
    let ans2: i64 = tests.iter()
        .filter_map(|(val,v)| {
            if dfs2(*val,v) {Some(val)} else {None}
        }).sum();
    fn length_10(x:i64) -> i64 {10_i64.pow(x.ilog10() + 1)}
    fn dfs2(val: i64, v: &[i64]) -> bool {
        if v.len() == 1 {
            val == v[0] 
        } else {
            let n = v.len() - 1;
            (val >= v[n] && dfs2(val - v[n], &v[..n]) )
            || (val % v[n] == 0 && dfs2(val / v[n], &v[..n]))
            || (val % length_10(v[n]) == v[n] && dfs2(val / length_10(v[n]), &v[..n])) 
        }
    }
    println!("part 2: {ans2}");
    Ok(())
}
fn day8() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(8,false,1)?;
    let grid = text.split('\n').map(|row| row.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let mut seen:HashMap::<char,Vec<(usize,usize)>> = HashMap::new();
    let mut antinodes = HashSet::new();
    fn check_and_push(nx:usize,ny:usize,grid:&[Vec<char>], antinodes: &mut HashSet<(usize,usize)>) {
        if nx < grid.len() && ny < grid[0].len() {
            antinodes.insert((nx,ny));
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] != '.' {
                for &(x,y) in seen.get(&grid[i][j]).unwrap_or(&Vec::new()) {
                    let dx = i.wrapping_sub(x) ; //x <= i because of check order
                    let dy = j.max(y).wrapping_sub(j.min(y));
                    let (nx,ny) = (x.wrapping_sub(dx),if y >= j {y + dy} else {y.wrapping_sub(dy)});
                    check_and_push(nx,ny,&grid,&mut antinodes);
                    let (nx,ny) = (i + dx,if y < j {j + dy} else {j.wrapping_sub(dy)});
                    check_and_push(nx,ny,&grid,&mut antinodes);
                }
                seen.entry(grid[i][j]).or_insert(Vec::new()).push((i,j));
            }
        }
    }
    println!("part 1: {}", antinodes.len());
    for mul in 2..grid.len().min(grid[0].len()) {
        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] != '.' {
                    for &(x,y) in seen.get(&grid[i][j]).unwrap_or(&Vec::new()) {
                        let dx = i.wrapping_sub(x).saturating_mul(mul); //x <= i because of check order
                        let dy = j.max(y).wrapping_sub(j.min(y)).saturating_mul(mul);
                        if dx >= grid.len() || dy >= grid[0].len() {continue}
                        let (nx,ny) = (x.wrapping_sub(dx),if y >= j {y + dy} else {y.wrapping_sub(dy)});
                        check_and_push(nx,ny,&grid,&mut antinodes);
                        let (nx,ny) = (i + dx,if y < j {j + dy} else {j.wrapping_sub(dy)});
                        check_and_push(nx,ny,&grid,&mut antinodes);
                    }
                    seen.entry(grid[i][j]).or_insert(Vec::new()).push((i,j));
                }
            }
        }
    }
    //for &(x,y)in &antinodes { grid[x][y] = '#'; }
    //for row in grid { println!("{}",row.into_iter().collect::<String>()); }
    //println!("{antinodes:?}");
    println!("part 2: {}", antinodes.len());
    Ok(())
}
fn day9() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(9,false,1)?;
    let mut orig: VecDeque<_> = text.bytes().enumerate().flat_map(|(i,x)| iter::repeat(if i % 2 == 0 {Some(i/2)} else {None}).take((x-b'0') as usize)).collect();
    let mut ans = Vec::new();
    while let Some(x) = orig.pop_front() {
        if let Some(val) = x {
            ans.push(val);
        } else {
            while let Some(x) = orig.pop_back() {
                if let Some(val) = x {
                    ans.push(val); break
                }
            }
        }
    }
    //println!("{ans:?}");
    println!("part 1: {}", ans.into_iter().enumerate().map(|(i,x)| i * x).sum::<usize>());
    let mut files: Vec<_> = text.bytes().map(|x| (x - b'0') as usize).enumerate().map(|(i,x)| {
        (i % 2, if i%2==0 {i/2} else {0}, x) // (1 if is gap else 0, ind number, length)
    }).collect();
    let mut i = files.len() -1;
    'outer: while i > 0 {
        if files[i].0 == 0 {//gaps have index 0 == 1
            for j in (0..i).filter(|j| files[*j].0 == 1) {
                if files[j].2 >= files[i].2 {
                    files[j].2 -= files[i].2;
                    let mem = files[i].clone();
                    files[i].1 = 0;
                    files[i].0 = 1;
                    files.insert(j,mem);
                    continue 'outer
                }
            }
        }
        i -= 1;
    }
    //println!("{files:?}");
    println!("part 2: {:?}", files.into_iter().flat_map(|(_,ind,size)| iter::repeat(ind).take(size)).enumerate().map(|(i,x)| i * x).sum::<usize>());
    Ok(())
}
fn day10() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(10,false,5)?;
    let grid = text.split('\n').map(|row|row.as_bytes().to_vec()).collect::<Vec<_>>();
    fn count(i:usize,j:usize,grid: &[Vec<u8>],dedup: bool) -> usize {
        let mut v = Vec::from([(i,j)]);
        for val in b'1'..=b'9' {
            let mut nxt = Vec::new();
            for (i,j) in v {
                for (x,y) in [(i+1,j),(i.wrapping_sub(1),j),(i,j+1),(i,j.wrapping_sub(1))] {
                    if x < grid.len() && y < grid[0].len() && grid[x][y] == val {
                        nxt.push((x,y));
                    }
                }
            }
            if dedup { nxt.sort_unstable(); nxt.dedup(); }
            v = nxt;
        }
        v.len()
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'0' {
                ans1 += count(i,j,&grid, true);
                ans2 += count(i,j,&grid, false);
            }
        }
    }
    println!("part 1: {ans1}");
    println!("part 2: {ans2}");
    Ok(())
}
fn day11() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(11,false,1)?;
    let v:Vec<_> = text.split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect();
    fn tot_block(num:usize,steps:usize,mem: &mut HashMap<[usize;2],usize>) -> usize {
        if steps == 0 {return 1}
        if let Some(ans) = mem.get(&[num,steps]) {return *ans}
        let ans = match (num,if num == 0 {0} else {num.ilog10()+1}, steps) {
            (0,_,1) => 1,
            (0,_,_) => tot_block(1,steps-1,mem),
            (_,len,1) if len %2 == 0 => 2,
            (_,len,_) if len %2 == 0 => tot_block(num / 10_usize.pow(len/2),steps-1,mem) + tot_block(num%10_usize.pow(len/2),steps-1,mem),
            (_,_,1) => 1,
            _ => tot_block(num * 2024, steps -1, mem)
        };
        mem.insert([num,steps], ans);
        ans
    }
    let mut mem = HashMap::new();
    println!("part 1: {}", v.iter().map(|num| tot_block(*num,25,&mut mem)).sum::<usize>());
    println!("part 2: {}", v.iter().map(|num| tot_block(*num,75,&mut mem)).sum::<usize>());
    Ok(())
}
fn day12() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(12,false,1)?;
    let grid = text.split('\n').map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    //for row in grid.iter() { println!("{}",row.iter().collect::<String>()); }
    fn get_shape(i: usize,j:usize, grid: &[Vec<char>],shape: &mut HashSet<[usize;2]>, seen: &mut HashSet<[usize;2]>) {
        for (x,y) in [(i+1,j),(i.wrapping_sub(1),j),(i,j+1),(i,j.wrapping_sub(1))] {
            if x < grid.len() && y < grid[0].len() && grid[x][y] == grid[i][j] && shape.insert([x,y]) {
                seen.insert([x,y]);
                get_shape(x,y,grid,shape,seen);
            }
        }
    }
    fn get_area(shape: &HashSet<[usize;2]>) -> usize {
        shape.len()
    }
    fn get_perm1(shape: &HashSet<[usize;2]>) -> usize {
        let mut ans = 0;
        for &[x,y] in shape {
            if !shape.contains(&[x.wrapping_sub(1),y]) {ans += 1}
            if !shape.contains(&[x+1,y]) {ans += 1}
            if !shape.contains(&[x,y.wrapping_sub(1)]) {ans += 1}
            if !shape.contains(&[x,y+1]) {ans += 1}
        }
        ans
    }
    fn get_perm2(shape: &HashSet<[usize;2]>) -> usize {
        //only count left most and top most fences
        let mut ans = 0; 
        for &[x,y] in shape {
            if !shape.contains(&[x.wrapping_sub(1),y]) //top
                && (!shape.contains(&[x,y.wrapping_sub(1)]) || shape.contains(&[x.wrapping_sub(1),y.wrapping_sub(1)])) {ans += 1}
            if !shape.contains(&[x+1,y])  //bottom
                && (!shape.contains(&[x,y.wrapping_sub(1)]) || shape.contains(&[x+1,y.wrapping_sub(1)])) {ans += 1}
            if !shape.contains(&[x,y.wrapping_sub(1)]) // left
                && (!shape.contains(&[x.wrapping_sub(1),y]) || shape.contains(&[x.wrapping_sub(1),y.wrapping_sub(1)])) {ans += 1}
            if !shape.contains(&[x,y+1]) //right
                && (!shape.contains(&[x.wrapping_sub(1),y]) || shape.contains(&[x.wrapping_sub(1),y+1])) {ans += 1}
        }
        ans
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut seen = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if seen.insert([i,j]) {
                let mut shape = HashSet::from([[i,j]]);
                get_shape(i,j,&grid,&mut shape,&mut seen);
                ans1 += get_area(&shape) * get_perm1(&shape);
                ans2 += get_area(&shape) * get_perm2(&shape);
                //println!("{i} {j} {} {} ",get_area(&shape),get_perm2(&shape));
            }
        }
    }
    println!("part 1: {}", ans1);
    println!("part 2: {}", ans2);
    Ok(())
}
fn day13() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(13,false,1)?;
    struct Game {
        ax: i64, ay: i64,
        bx: i64, by: i64,
        prizex: i64, prizey: i64,
    }
    fn proc_line(s:&str,c:char) -> Vec<i64> {
        s.split(':').skip(1).next().unwrap().split(',')
        .filter_map(|s| s.split(c).skip(1).next().unwrap().parse::<i64>().ok())
        .collect()
    }
    impl Game {
        fn new(lines: &mut dyn Iterator<Item = &str>) -> Self {
            let axy = proc_line(lines.next().unwrap(),'+');
            let bxy = proc_line(lines.next().unwrap(),'+');
            let prizexy = proc_line(lines.next().unwrap(),'=');
            lines.next(); //consume empty line
            Game{ax:axy[0],ay:axy[1],bx:bxy[0],by:bxy[1],prizex:prizexy[0],prizey:prizexy[1]}
        }
        fn invert_2_win(&self) -> Option<i64> {
            let determinant = self.ax * self.by - self.bx * self.ay;
            if determinant != 0 {
                let a_push = self.by * self.prizex - self.bx * self.prizey;
                let b_push = - self.ay * self.prizex + self.ax * self.prizey;
                if a_push % determinant == 0 && b_push % determinant == 0 {
                    Some(3 * a_push / determinant + b_push / determinant)
                } else {None}
            } else {//hope for no collinear parts
                println!("found a collinear equation"); 
                None
            } 
        }
    }
    let mut lines = text.split('\n').peekable();
    let mut games = Vec::new();
    while lines.peek().is_some() {
        games.push(Game::new(&mut lines));
    }
    println!("part 1: {}", games.iter().filter_map(|game| game.invert_2_win()).sum::<i64>());
    for game in games.iter_mut() {
        game.prizex += 10000000000000;
        game.prizey += 10000000000000;
    }
    println!("part 2: {}", games.iter().filter_map(|game| game.invert_2_win()).sum::<i64>());
    Ok(())
}
fn day14() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(14,false,1)?;
    let wide = 101;// 11;
    let tall = 103;//7;
    let mut robots = Vec::new();
    for line in text.split('\n') {
        let mut robot = Vec::new();
        for part in line.split_whitespace() {
            for num in part.split('=').skip(1).next().unwrap().split(',') {
                robot.push(num.parse::<i64>().ok().unwrap());
            }
        }
        robots.push(robot);
    }
    let orig = robots.clone();
    for robot in robots.iter_mut() {
        robot[0] += robot[2] * 100;
        robot[0] = robot[0].rem_euclid(wide);
        robot[1] += robot[3] * 100;
        robot[1] = robot[1].rem_euclid(tall);
    }
    let mut quadrants = vec![0;4];
    for robot in robots.iter() {
        match (robot[0], robot[1]) {
            (x,y) if x == wide / 2 || y == tall / 2 => (),
            (x,y) if x < wide / 2 && y < tall / 2 => quadrants[0] += 1,
            (x,y) if x > wide / 2 && y < tall / 2 => quadrants[1] += 1,
            (x,y) if x < wide / 2 && y > tall / 2 => quadrants[2] += 1,
            (x,y) if x > wide / 2 && y > tall / 2 => quadrants[3] += 1,
            _ => ()
        }
    }
    println!("{quadrants:?}");
    println!("part 1: {}", quadrants.into_iter().fold(1,|a,b| a * b));
    robots = orig.clone();
    fn step(robots: &mut Vec<Vec<i64>>,wide:i64,tall:i64) {
        for robot in robots.iter_mut() {
            robot[0] += robot[2];
            robot[0] = robot[0].rem_euclid(wide);
            robot[1] += robot[3];
            robot[1] = robot[1].rem_euclid(tall);
        }
    }
    let mut input = "".to_string();
    for i in 1.. {
        step(&mut robots,wide,tall);
        println!("step is {i}");
        let mut grid = vec![vec![' ';wide as usize]; tall as usize];
        for robot in robots.iter() {
            grid[robot[1] as usize][robot[0] as usize] = 'X';
        }
        if grid.iter().any(|c| c.iter().collect::<String>().contains("XXXXXXX")) {
            for row in grid {println!("{}",row.iter().collect::<String>());}
            stdin().read_line(&mut input) .expect("Failed to read line");
            if input == "s".to_string() {break}
        }
    }
    Ok(())
}
fn main() {
let now = Instant::now();
let _ = day14();
println!("Elapsed: {:.2?}", now.elapsed());
}
