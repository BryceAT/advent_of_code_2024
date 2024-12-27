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
fn day15() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(15,false,2)?;
    let mut it = text.split('\n');
    let mut grid = it.by_ref().take_while(|row| !row.is_empty()).map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let directions: Vec<char> = it.flat_map(|row| row.chars()).collect();
    let mut cur = (0,0);
    'outer: for (i,row) in grid.iter().enumerate() {for (j,c) in row.iter().enumerate() {if *c == '@' {cur = (i,j); break 'outer}}}
    fn step(cur: &mut (usize,usize), grid: &mut [Vec<char>], d: char) {
        match d {
            '<' if grid[cur.0][..cur.1].contains(&'.') => {
                let mut y = 0;
                for j in (0..cur.1).rev() {match grid[cur.0][j] {'#'=>return (), '.'=> {y=j;break},_=> ()}}
                for j in y..cur.1 {
                    grid[cur.0][j] = grid[cur.0][j+1];
                }
                grid[cur.0][cur.1] = '.';
                cur.1 -= 1;
            },
            '>' if grid[cur.0][cur.1..].contains(&'.') => {
                let mut y = 0;
                for j in cur.1.. {match grid[cur.0][j] {'#'=>return (), '.' => {y=j;break},_=>()}}
                for j in (cur.1 .. y).rev() {
                    grid[cur.0][j+1] = grid[cur.0][j];
                }
                grid[cur.0][cur.1] = '.';
                cur.1 += 1;
            },
            '^' if (0..cur.0).any(|x| grid[x][cur.1] == '.') => {
                let mut x = 0;
                for i in (0..cur.0).rev() {match grid[i][cur.1] {'#'=>return (), '.'=> {x=i;break}, _=>()}}
                for i in x..cur.0 {
                    grid[i][cur.1] = grid[i+1][cur.1];
                }
                grid[cur.0][cur.1] = '.';
                cur.0 -= 1;
            },
            'v' if (cur.0..grid.len()).any(|x| grid[x][cur.1] == '.') => {
                let mut x = 0;
                for i in cur.0.. {match grid[i][cur.1] {'#'=>return (), '.'=> {x=i;break}, _=>()}}
                for i in (cur.0 .. x).rev() {
                    grid[i+1][cur.1] = grid[i][cur.1];
                }
                //println!("x is {x} cur is {cur:?}");
                grid[cur.0][cur.1] = '.';
                cur.0 += 1;
            },
            _ => ()
        }
    }
    for d in directions { 
        step(&mut cur, &mut grid, d); 
        //println!("{d}");
        //for row in grid.iter() { println!("{}", row.into_iter().collect::<String>()); }
    }
    //for row in grid.iter() { println!("{}", row.into_iter().collect::<String>()); }
    println!("part 1: {:?}", grid.iter().enumerate().map(|(i,row)| row.iter().enumerate().filter_map(|(j,x)| if *x == 'O' {Some(i*100 + j)} else {None}).sum::<usize>()).sum::<usize>());
    let mut it2 = text.split('\n');
    let mut grid2 = it2.by_ref().take_while(|row| !row.is_empty()).map(|row| row.chars().flat_map(|c| match c {
        '#' => ['#','#'].into_iter(),
        'O' => ['[',']'].into_iter(),
        '.' => ['.','.'].into_iter(),
        _ => ['@','.'].into_iter(),
    }).collect::<Vec<_>>()).collect::<Vec<_>>();
    let directions: Vec<char> = it2.flat_map(|row| row.chars()).collect();
    let mut cur = (0,0);
    'outer: for (i,row) in grid2.iter().enumerate() {for (j,c) in row.iter().enumerate() {if *c == '@' {cur = (i,j); break 'outer}}}
    fn check(cur: (usize,usize), grid: &[Vec<char>], d: char) -> bool {
        match d {
            '<' => {
                for j in (0..cur.1).rev() {
                    match grid[cur.0][j] {
                        '#'=> return false, 
                        '.'=> return true,
                        _ => ()
                    }
                }
            },
            '>' => {
                for j in cur.1 + 1 .. {
                    match grid[cur.0][j] {
                        '#'=> return false, 
                        '.'=> return true,
                        _ => ()
                    }
                }
            },
            '^' => {
                for i in (0..cur.0).rev() {
                    match grid[i][cur.1] {
                        '#' => return false, 
                        '.' => return true,
                        '[' => return check((i,cur.1),grid,d) && check((i,cur.1 + 1),grid,d),
                        ']' => return check((i,cur.1 -1 ),grid,d) && check((i,cur.1),grid,d),
                        _ => () 
                    }
                }
            },
            'v' => {
                for i in cur.0 + 1 .. {
                    match grid[i][cur.1] {
                        '#' => return false, 
                        '.' => return true,
                        '[' => return check((i, cur.1),grid,d) && check((i, cur.1 + 1),grid,d),
                        ']' => return check((i, cur.1 - 1 ),grid,d) && check((i , cur.1),grid,d),
                        _ => () 
                    }
                }
            },
            _ => unreachable!("d must be ^v<> not {d}"),
        }
        false
    }
    fn step_rec(cur: (usize,usize), grid: &mut Vec<Vec<char>>, d: char) {
        //assume move is possible
        //for row in grid.iter() { println!("{}", row.into_iter().collect::<String>()); }
        match d {
            '<' => {
                match grid[cur.0][cur.1 -1] {
                    '.' => (),
                    _ => step_rec((cur.0,cur.1-1),grid,d),
                } 
                grid[cur.0][cur.1 -1] = grid[cur.0][cur.1];
                grid[cur.0][cur.1] = '.';
            },
            '>' => {
                match grid[cur.0][cur.1 +1] {
                    '.' => (),
                    _ => step_rec((cur.0,cur.1 + 1),grid,d),
                } 
                grid[cur.0][cur.1 + 1] = grid[cur.0][cur.1];
                grid[cur.0][cur.1] = '.';
            },
            '^' => {
                match grid[cur.0 - 1][cur.1] {
                    '.' => (),
                    '[' => {
                        step_rec((cur.0 - 1,cur.1),grid,d);
                        step_rec((cur.0 - 1,cur.1+1),grid,d)
                    },
                    ']' => {
                        step_rec((cur.0 - 1,cur.1),grid,d);
                        step_rec((cur.0 - 1,cur.1-1),grid,d)
                    },
                    _ => step_rec((cur.0 - 1,cur.1),grid,d),
                } 
                grid[cur.0 - 1][cur.1] = grid[cur.0][cur.1];
                grid[cur.0][cur.1] = '.';
            },
            'v' => {
                match grid[cur.0 + 1][cur.1] {
                    '.' => (),
                    '[' => {
                        step_rec((cur.0 + 1,cur.1),grid,d);
                        step_rec((cur.0 + 1,cur.1+1),grid,d)
                    },
                    ']' => {
                        step_rec((cur.0 + 1,cur.1),grid,d);
                        step_rec((cur.0 + 1,cur.1-1),grid,d)
                    },
                    _ => step_rec((cur.0 + 1,cur.1),grid,d),
                } 
                grid[cur.0 + 1][cur.1] = grid[cur.0][cur.1];
                grid[cur.0][cur.1] = '.';
            },
            _ => unreachable!("d is not valid {cur:?}")
        }
    }
    //let mut input = "press enter to continue".to_string();
    for d in directions { 
        if check(cur.clone(),&grid2,d) {
            step_rec(cur.clone(),&mut grid2, d);
            cur = match d {
                '<' => (cur.0, cur.1 - 1),
                '>' => (cur.0, cur.1 + 1),
                '^' => (cur.0 - 1, cur.1),
                'v' => (cur.0 + 1, cur.1),
                _ => unreachable!("d is not valid {d} {cur:?}"),
            }
        }
        //println!("{d}");
        //for row in grid2.iter() { println!("{}", row.into_iter().collect::<String>()); }
        //stdin().read_line(&mut input).expect("Failed to read line");
    }
    for row in grid2.iter() { println!("{}", row.into_iter().collect::<String>()); }
    println!("part 2: {:?}", grid2.iter().enumerate().map(|(i,row)| row.iter().enumerate().filter_map(|(j,x)| if *x == '[' {Some(i*100 + j)} else {None}).sum::<usize>()).sum::<usize>());
    
    Ok(())
}
fn day16() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(16,false,1)?;
    let grid = text.split('\n').map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let foward_cost = 1;
    let turn_cost = 1_000;
    let mut heap = BinaryHeap::new();
    let mut start_x = 0; let mut start_y = 0;
    for i in 0..grid.len() {for j in 0..grid[0].len() {if grid[i][j] == 'S' { (start_x,start_y) = (i,j);}}}
    heap.push((Reverse(0),start_x,start_y,0_i32));
    let mut seen:HashMap<(usize,usize,i32),i64> = HashMap::new();
    let mut best = 0;
    while let Some((Reverse(cost),x,y,d)) = heap.pop() {
        if grid[x][y] == 'E' {
            println!("part 1 {cost}"); best = cost; break
        }
        for (nxt_cost,a,b,e) in [
            match d {
                0 => (cost + foward_cost, x,y+1,d),
                1 => (cost + foward_cost, x+1,y,d),
                2 => (cost + foward_cost, x,y.wrapping_sub(1),d),
                _ => (cost + foward_cost, x.wrapping_sub(1),y,d),
            },
            (cost + turn_cost, x,y,(d + 1).rem_euclid(4)),
            (cost + turn_cost, x,y,(d - 1).rem_euclid(4))] {
            if a < grid.len() && b < grid[0].len() && grid[a][b] != '#' && nxt_cost < *seen.get(&(a,b,e)).unwrap_or(&i64::MAX) {
                heap.push((Reverse(nxt_cost),a,b,e));
                seen.insert((a,b,e),nxt_cost);
            }
        }
    }
    let mut seats = HashSet::new();
    fn dfs(cost: i64, grid: &[Vec<char>], seats: &mut HashSet<[usize;2]>, path: &mut Vec<(usize,usize,i32)>, best: i64,seen: &HashMap<(usize,usize,i32),i64>) {
        if path.is_empty() {return ()}
        let &(x,y,d) = path.last().unwrap();
        if cost == best && grid[x][y] == 'E' {
            for &(a,b,_) in path.iter() {
                seats.insert([a,b]);
            }
            return ()
        }
        for (nxt_cost,a,b,e) in [
            match d {
                0 => (cost + 1, x,y+1,d),
                1 => (cost + 1, x+1,y,d),
                2 => (cost + 1, x,y.wrapping_sub(1),d),
                _ => (cost + 1, x.wrapping_sub(1),y,d),
            },
            (cost + 1_000, x,y,(d + 1).rem_euclid(4)),
            (cost + 1_000, x,y,(d - 1).rem_euclid(4))] {
            if a < grid.len() && b < grid[0].len() && grid[a][b] != '#' && nxt_cost <= best && *seen.get(&(a,b,e)).unwrap_or(&0) == nxt_cost {
                path.push((a,b,e));
                dfs(nxt_cost,grid, seats, path, best,seen);
                path.pop();
            }
        }
    }
    dfs(0,&grid,&mut seats, &mut vec![(start_x, start_y, 0)], best,&seen);
    println!("part 2: {}", seats.len());
    Ok(())
}
fn day17() -> Result<(), Box<dyn Error>> {
    let text: String = get_text(17,false,1)?;
    let it: Vec<_> = text.split('\n').take(3).filter_map(|line| line.split(": ").skip(1).next().unwrap().parse::<i64>().ok()).collect();
    let program: Vec<_> = text.split('\n').skip(4).next().unwrap().split(": ").skip(1).next().unwrap().split(',').filter_map(|x| x.parse::<i64>().ok()).collect();
    fn combo(operand: i64, reg: &[i64; 3]) -> i64 {
        match operand {
            0..=3 => operand,
            4 => reg[0],
            5 => reg[1],
            6 => reg[2],
            _ => unreachable!("7 is reserved and does not appear in valid programs")
        }
    }
    let mut outputs = Vec::new();
    fn step(state: (usize,[i64; 3]), program: &[i64], outputs: &mut Vec<i64>) -> (usize,[i64; 3]) {
        let i = state.0;
        let reg = state.1;
        let opcode = program[i]; 
        let operand = program[i+1];
        match opcode {
            0 => (i+2,[reg[0] / 2_i64.pow(combo(operand,&reg) as u32), reg[1], reg[2]]),
            1 => (i+2,[reg[0], reg[1] ^ operand as i64, reg[2]]),
            2 => (i+2,[reg[0], combo(operand,&reg) as i64 % 8, reg[2]]),
            3 if reg[0] == 0 => (i+2,[reg[0],reg[1],reg[2]]),
            3 => (operand as usize, [reg[0],reg[1],reg[2]]),
            4 => (i+2,[reg[0], reg[1] ^ reg[2], reg[2]]),
            5 => {outputs.push(combo(operand,&reg) % 8 ); (i+2,[reg[0], reg[1], reg[2]])},
            6 => (i+2,[reg[0], reg[0] / 2_i64.pow(combo(operand,&reg) as u32), reg[2]]),
            7 => (i+2,[reg[0], reg[1], reg[0] / 2_i64.pow(combo(operand,&reg) as u32)]),
            _ => unreachable!("opcode must be less than 8"),
        }
    }
    let mut state = (0,[it[0],it[1],it[2]]);
    while state.0 < program.len() -1 {
        state = step(state, &program, &mut outputs);
    }
    println!("{}",outputs.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(","));
    //let program = vec![0,3,5,4,3,0];
    /*let obscure = |a| {((a % 8) ^ 3 ^ (a / (1 << ((a%8)^5)) as i64)) % 8};
    let mut ans: Vec<_> = (10 ..  30_000_000_000_000).into_par_iter()
        .filter(|&a| obscure(a / 32768) == 5)
        .filter(|&a| obscure(a) == 2)
        .filter(|&a| obscure(a / 8) == 4)
        .filter(|&a| obscure(a / 64 ) == 1)
        .filter(|&a| obscure(a / 512 ) == 5)
        .filter(|&a| obscure(a / 4096 ) == 7)
        .filter(|&a| obscure(a / 262144) == 1)
        .filter(|&a| obscure(a / 2097152) == 6)
        .filter(|&a| obscure(a / 16777216) == 4)
        .filter(|&a| obscure(a / 134217728) == 3)
        .filter(|&a| obscure(a / 1073741824) == 5)
        .filter(|&a| obscure(a / 8589934592) == 5)
        .filter(|&a| obscure(a / 68719476736) == 0)
        .filter(|&a| obscure(a / 549755813888) == 3)
        .filter(|&a| obscure(a / 4398046511104) == 3)
        .filter(|&a| obscure(a / 35184372088832) == 0)
        .collect();
    ans.sort_unstable();
    println!("{ans:?}");
    */
    
    //If register C contains 9, the program 2,6 would set register B to 1.
    //println!("{} = 1", {let mut state = (0,[0,0,9]); let program = [2,6]; while state.0 < program.len() {state = step(state, &program, &mut outputs)} state.1[1]});
    //If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    //println!("{:?} = [0,1,2]", {let mut outputs = Vec::new(); let mut state = (0,[10,0,0]); let program = [5,0,5,1,5,4]; while state.0 < program.len() {state = step(state, &program, &mut outputs)} outputs});
    //If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    //println!("{:?} = [4,2,5,6,7,7,7,7,3,1,0]", {let mut outputs = Vec::new(); let mut state = (0,[2024,6,6]); let program = [0,1,5,4,3,0]; while state.0 < program.len() {state = step(state, &program, &mut outputs)} outputs});
    //println!("{:?} = 0", {let mut outputs = Vec::new(); let mut state = (0,[2024,6,6]); let program = [0,1,5,4,3,0]; while state.0 < program.len() {state = step(state, &program, &mut outputs)} state.1[0]});
    //If register B contains 29, the program 1,7 would set register B to 26.
    //println!("{:?} = 26", {let mut outputs = Vec::new(); let mut state = (0,[6,29,6]); let program = [1,7]; while state.0 < program.len() {state = step(state, &program, &mut outputs)} state.1[1]});
    //If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    //println!("{:?} = 44354", {let mut outputs = Vec::new(); let mut state = (0,[6,2024,43690]); let program = [4,0]; while state.0 < program.len() {state = step(state, &program, &mut outputs)} state.1[1]});
    let track:Vec<_> = program.iter().rev().cloned().collect();
    fn dfs(cur: i64, i:usize, track: &[i64]) -> Option<i64> {
        if i == track.len() {return Some(cur)}
        for x in cur * 8 .. cur * 8 + 8 {
            if ((x%8) ^ 3 ^ (x/2_i64.pow((x%8) as u32 ^ 5))) % 8 == track[i] {
                if let Some(ans) = dfs(x,i+1,track) {
                    return Some(ans)
                }
            }
        }
        None
    }
    println!("part 2: {}", dfs(0,0,&track).unwrap());
    Ok(())
}
fn day18() -> Result<(), Box<dyn Error>> {
    let text = get_text(18, false, 1)?;
    let blocks:HashMap<Vec<_>,usize> = text.split('\n').enumerate()
        .map(|(i,s)| (s.split(',').filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<_>>(),i)).collect();
    fn steps(blocks: &HashMap<Vec<i32>,usize>, time: usize) -> Option<i32> {
        let n = 71;
        let mut level = VecDeque::new();
        let mut seen = HashSet::new();
        level.push_back((vec![0,0],0));
        while let Some((cur,step)) = level.pop_front() {
            let a = cur[0];
            let b = cur[1];
            if a == n-1 && b == n-1 {return Some(step)}
            for (x,y) in [(a +1,b),(a-1,b),(a,b+1),(a,b-1)] {
                if x >= 0 && y >= 0 && x < n && y < n && seen.insert([x,y]) && *blocks.get(&vec![x,y]).unwrap_or(&usize::MAX) >= time {
                    level.push_back((vec![x,y], step + 1));
                }
            }
        }
        None
    }
    if let Some(ans) = steps(&blocks, 1024) {println!("part 1: {ans}");}
    let mut l = 12;
    let mut r = blocks.len();
    while l < r {
        let mid = (l + r ) / 2;
        if steps(&blocks, mid).is_some() {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    println!("part 2: {}",text.split('\n').skip(l-1).next().unwrap());
    Ok(())
}
fn day19() -> Result<(), Box<dyn Error>> {
    let text = get_text(19, false, 1)?;
    let pats: Vec<_> = text.split('\n').next().unwrap().split(", ").collect();
    let words: Vec<_> = text.split('\n').skip(2).collect();
    let possible = |w: &str| {
        let mut dp = vec![0_usize; w.len() + 1];
        dp[0] = 1;
        for i in 0..dp.len() - 1 {
            if dp[i] > 0 {
                for p in pats.iter() {
                    if w[i..].starts_with(p) {
                        match (w.len(),i + p.len()) {
                            (a,b) if a < b => (),
                            (_,b) => dp[b] += dp[i],
                        }
                    }
                }
            }
        }
        dp.pop().unwrap()
    };
    println!("part 1: {}",words.iter().filter(|w| possible(w) > 0).count());
    println!("part 2: {}",words.iter().map(|w| possible(w) ).sum::<usize>());
    Ok(())
}
fn day20() -> Result<(), Box<dyn Error>> {
    let text = get_text(20, false, 1)?;
    let grid: Vec<_> = text.split('\n').map(|row| row.chars().collect::<Vec<_>>()).collect();
    let start = {
        let mut start = (0,0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'S' {start = (i,j);}
            }
        }
        start
    };
    let (mut i, mut j) = start.clone();
    let mut path = Vec::new();
    path.push(start.clone());
    'outer: loop {
        for (x,y) in [(i+1,j),(i.wrapping_sub(1),j),(i,j+1),(i,j.wrapping_sub(1))] {
            if x < grid.len() && y < grid[0].len() && !path.contains(&(x,y)) && grid[x][y] != '#' {
                path.push((x,y));
                match grid[x][y] {
                    '.' => { (i,j) = (x,y); break;},
                    'E' => break 'outer,
                    c => unreachable!("grid should not contain {c}"),
                }
            }
        }
    }
    let solution_len = path.len() - 1;
    println!("no cheating needs {solution_len}");
    let dist:HashMap::<(usize,usize),usize> = path.iter().rev().enumerate().map(|(i,p)| (p.clone(),i)).collect();
    let mut path_lens = HashMap::new();
    for (step,(i,j)) in path.iter().cloned().enumerate() {
        for p in [(i+2,j),(i.wrapping_sub(2),j),(i,j+2),(i,j.wrapping_sub(2))] {
            if let Some(&d) = dist.get(&p) {
                if d + step + 2 < solution_len {
                    path_lens.entry(d + step + 2).and_modify(|ct| *ct += 1).or_insert(1);
                }
            }
        }
    }
    println!("part 1: {}", path_lens.iter().filter_map(|(d,ct)| if *d +100 <= solution_len {Some(ct)} else {None}).sum::<usize>());
    path_lens.clear();
    for (step,(i,j)) in path.iter().cloned().enumerate() {
        for cheat_x in -20 ..= 20_i32 {
            for cheat_y in -20 ..= 20_i32 {
                if cheat_x.abs() + cheat_y.abs() <= 20 {
                    if let Ok(x) = usize::try_from(i as i32 + cheat_x) {
                        if let Ok(y) = usize::try_from(j as i32 + cheat_y) {
                            let p = (x,y);
                            if let Some(&d) = dist.get(&p) {
                                let new_len = d + step + cheat_x.abs() as usize + cheat_y.abs() as usize;
                                if new_len + 100 <= solution_len {
                                    path_lens.entry(new_len).and_modify(|ct| *ct += 1).or_insert(1);
                                }
                            }
                        }
                    }
                    
                }
            }
        }
        
    }
    println!("part 2: {}", path_lens.iter().filter_map(|(d,ct)| if *d +100 <= solution_len {Some(ct)} else {None}).sum::<usize>());
    Ok(())
}
fn day22() -> Result<(), Box<dyn Error>> {
    let text = get_text(22, false, 6)?;
    let list:Vec<_> = text.split('\n').filter_map(|line| line.parse::<i64>().ok()).collect();
    fn step(mut num: i64) -> i64 {
        let mix = |a,b| a^b;
        let prune = |x| x % 16777216;
        num = mix(num, num * 64);
        num = prune(num);

        num = mix(num, num / 32);
        num = prune(num);

        num = mix(num, num * 2048);
        prune(num)
    }
    println!("part 1: {}", list.iter().cloned().map(|mut num| {for _ in 0..2000 {num = step(num)} num}).sum::<i64>());
    fn mem_sell(num: i64, a: i64, b: i64, c: i64, d: i64, sell_mem: &mut HashMap<i64,HashMap<[i64;4],i64>>) -> i64 {
        if let Some(ans) = sell_mem.get(&num) {return *ans.get(&[a,b,c,d]).unwrap_or(&0)}
        let nums = (1..2_000).fold((vec![num],num),|(mut v, mut n),_| {n = step(n); v.push(n); (v,n)}).0;
        let diffs: Vec<i64> = iter::once(nums[0]%10).chain(nums.iter().zip(nums.iter().skip(1)).map(|(a,b)| b%10 - a%10)).collect();
        let mut ans:HashMap<[i64;4],i64> = HashMap::new();
        for (i,w) in diffs.windows(4).enumerate() {
            ans.entry([w[0],w[1],w[2],w[3]]).or_insert(nums[i+3] % 10);
        }
        sell_mem.insert(num,ans);
        mem_sell(num,a,b,c,d,sell_mem)
    }
    let mut sell_mem = HashMap::new();
    
    let mut best = 0;
    for a in -9..=9 {
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    best = best.max(list.iter().cloned().map(|num| mem_sell(num,a,b,c,d, &mut sell_mem)).sum::<i64>())
                }
            }
        }
    }
    println!("part 2: {}", best);
    
    Ok(())
}
fn day23() -> Result<(), Box<dyn Error>> {
    let text = get_text(23, false, 1)?;
    let mut pairs = HashMap::new();
    for line in text.split('\n') {
        let p: Vec<_> = line.split('-').collect();
        pairs.entry(p[0].to_string()).and_modify(|v: &mut Vec<_>| v.push(p[1].to_string())).or_insert(vec![p[1].to_string()]);
        pairs.entry(p[1].to_string()).and_modify(|v: &mut Vec<_>| v.push(p[0].to_string())).or_insert(vec![p[0].to_string()]);
    }
    let mut ans = 0;
    for (k,v) in pairs.iter() {
        if k.starts_with("t") {
            for k2 in v {
                if k2.starts_with("t") && k > k2 {continue}
                for k3 in pairs.get(k2).unwrap() {
                    if k2 < k3 && pairs[k3].contains(k) {
                        //println!("{k} {k2} {k3}");
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("part 1 {ans}");
    let mut keys = HashSet::new();
    let mut pairs = HashSet::new();
    for line in text.split('\n') {
        let p: Vec<_> = line.split('-').map(|x| x.to_string()).collect();
        for x in p.iter() {
            keys.insert(x.to_string());
        }
        pairs.insert([p[0].clone(),p[1].clone()]);
        pairs.insert([p[1].clone(),p[0].clone()]);
    }
    let mut level: VecDeque<HashSet<String>> = pairs.iter().cloned().map(|p| p.iter().cloned().collect()).collect();
    let mut seen : HashSet<Vec<String>> = HashSet::new();
    while let Some(set) = level.pop_front() {
        for k in keys.iter() {
            if !set.contains(k) && set.iter().all(|k2| pairs.contains(&[k.to_string(),k2.to_string()])) {
                let mut temp = set.clone();
                temp.insert(k.to_string());
                if seen.insert({let mut v:Vec<String> = temp.iter().cloned().collect(); v.sort_unstable(); v}){
                    level.push_back(temp);
                }
            }
        }
        if level.is_empty() {
            let mut v = set.into_iter().collect::<Vec<_>>();
            v.sort();
            println!("{}",v.join(",") );
        }
    }
    Ok(())
}
fn main() {
    let now = Instant::now();
    let _ = day23();
    println!("Elapsed: {:.2?}", now.elapsed());
}
