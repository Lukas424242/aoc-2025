use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let liste: Vec<Vec<u64>> = input
        .split(',')
        .map(|x| x.split('-').map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut sum = 0;
    for i in 0..liste.len() {
        let a1 = liste[i][0];
        let a2 = liste[i][0];

        for i in a1..a2 + 1 {
            if i.to_string().len() % 2 == 0 {
                let a = i.to_string().len() / 2;

                let (g, h) = m.split_at(2);

                if g == h {
                    sum += i;
                }
            }
        }
    }

    let i = 2222;
    let mut m = i.to_string();
    let a = i.to_string().len() / 2;
    let (g, h) = m.split_at(2);

    println!("{} {}", g, h);

    for i in 0..5 {
        println!("{}", i);
    }
}
