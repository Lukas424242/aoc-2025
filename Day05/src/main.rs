use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let t: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<[usize; 2]> = t[0]
        .lines()
        .map(|x| {
            let a: Vec<usize> = x.split("-").map(|x| x.parse().unwrap()).collect();
            [a[0], a[1]]
        })
        .collect();

    let numbers:Vec<usize> = t[1].lines().map(|x|x.parse().unwrap()).collect();

    println!("");

    let mut sum=0;
    for i in 0..numbers.len(){

        let num = numbers[i];
        'fucked: for j in 0..ranges.len(){

            let l = ranges[j][0];
            let r = ranges[j][1];

            if l<= num && num <=r {
                sum+=1;
                break 'fucked;
                
            }
            
        }
    }
    println!("{}", sum);

    // Part 2
    let mut nrange:Vec<[usize;2]> = Vec::new();
    nrange.push(nrange[0]);

    // keine Überschneidungen mehr, dann normal rechnen
    for i in 1..ranges.len(){
        let l = ranges[i][0];
        let r = ranges[i][1];

        for j in 0..nrange.len(){
            let prev:i32 = j as i32 - 1;
            let next:i32 = j as i32 + 1;

            if 0<=prev && next <= nrange.len() as i32 -1  {
                
            }
            



        }
        
    }
}
/* 
1. Fall, neue range
2. Fall, fällt komplett in eine range rein
3. Fall, überschneidet sich
ordnung muss beibehalten werden
*/