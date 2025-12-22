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
        let a2 = liste[i][1];

        for i in a1..a2 + 1 {

            if i.to_string().len() % 2 == 0 {
                let a = i.to_string().len() / 2;

                let s = i.to_string();
                let (g, h) = s.split_at(a);

                if g == h {
                    sum += i;
                }
            }
        }
    }

    println!("{}", sum);
}
