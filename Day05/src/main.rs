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

    let mut list:HashSet<usize> = HashSet::new();

    for i in 0..ranges.len(){
        println!("{}", i/ranges.len());
        let l = ranges[i][0];
        let r = ranges[i][1] +1;

        for m in l..r{
            list.insert(m);
        }

    }
    println!("{}", list.len());
}
