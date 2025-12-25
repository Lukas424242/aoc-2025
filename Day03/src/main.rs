use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let zahlen: Vec<Vec<u32>> = input
        .lines()
        .map(|x| x.trim().chars().map(|a| a.to_digit(10).unwrap()).collect())
        .collect();

    println!();


    let mut sum=0;
    for i in 0..zahlen.len() {
        let a = &zahlen[i];
        let mut max:u32 = u32::min_value();

        for j in 0..a.len() {
            for l in j..a.len() {

                if max < a[j]*10 +a[l] && j!=l{
                    max = a[j]*10 +a[l];
                }
            }
        }
        println!();
        sum+=max;
    }

    println!("{}", sum);
}
