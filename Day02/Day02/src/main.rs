use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let liste: Vec<(char, u16)> = input
        .lines()
        .map(|x| {
            let a = x.trim().split_at(1);

            let a1: char = a.0.chars().next().unwrap();
            let a2: u16 = a.1.parse().expect("fuc");

            (a1, a2)
        })
        .collect();

    // Part 1
    let mut intial: u32 = 50;
    let mut sump1: u32 = 0;
    let mut sump2: u32 = 0;
    for i in 0..liste.len() {
        let direction = liste[i].0;
        let value = liste[i].1 as u32 % 100;
        sump2 += liste[i].1 as u32 / 100;

        match direction {
            'L' => {
                if intial < value {
                    let a = value - intial;
                    let mut intial2 = 100 - a;
                    if intial != 0  && intial2 !=0{
                        sump2 += 1;
                    }

                    intial= intial2;
                } else {
                    intial = intial - value;
                }
            }
            'R' => {
                if value + intial > 99 {
                    let intial2 = value + intial - 100;

                    if intial != 0 && intial2 !=0 {
                        sump2 += 1;
                    }
                    intial = intial2;
                } else {
                    intial = value + intial;
                }
            }
            _ => unreachable!("fucked"),
        }

        if intial == 0 {
            sump1 += 1;
            sump2 += 1;
        }
    }
    println!("{}", sump1);
    println!("{}", sump2);
    // 2
}
