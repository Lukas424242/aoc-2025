use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("demo.txt").unwrap();

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

    // Part2
    let mut sum = 0;
    for a in 0..liste.len() {
        let a1 = liste[a][0];
        let a2 = liste[a][1];

        for i in a1..a2 + 1 {
            let a: Vec<u8> = i
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect();

            // welche Vielfachen müssen wir bestimmen
            // alle zwei, drei usw.
            for j in 0..a.len() - 1 {
                let a1 = j + 1;

                if a.len() % a1 == 0 {
                    let mut list: HashSet<Vec<u8>> = HashSet::new();

                    // warum nicht rekursuv, du bist dumm
                    let mut begins: Vec<u8> = Vec::new();
                    let mut mul = 0;
                    begins.push(0);
                    loop {
                        mul += 1;
                        if a1 as u8 * mul >= a.len() as u8 {
                            break;
                        } else {
                            begins.push(a1 as u8 * mul);
                        }
                    }

                    let mut current: usize = 0;
                    let mut liste: Vec<u8> = Vec::new();
                    for k in 0..a.len() {
                        if k as u8 == begins[current] {
                            list.insert(liste.clone());
                            list.clear();
                            liste.push(a[k]);

                            if current < begins.len() - 1 {
                                current += 1;
                            }
                        } else {
                            liste.push(a[k]);
                        }
                    }

                    if list.len() == 1 {
                        sum += i;
                    }
                }
            }
        }
    }

    // test

    // welche Vielfachen müssen wir bestimmen
    // alle zwei, drei usw.

    let a = vec![1,1,8,8,5,1,1,8,8,5];
    let a = vec![1,1,8,8,5,1,1,8,8,5];

    for j in 0..a.len() - 2 {
        let a1 = j + 1;

        if a.len() % a1 == 0 {
            let mut list: HashSet<Vec<u8>> = HashSet::new();

            // warum nicht rekursuv, du bist dumm
            let mut begins: Vec<u8> = Vec::new();
            let mut mul = 0;
            begins.push(0);
            loop {
                mul += 1;
                if a1 as u8 * mul >= a.len() as u8 {
                    break;
                } else {
                    begins.push(a1 as u8 * mul);
                }
            }

            let mut current: usize = 0;
            let mut liste: Vec<u8> = Vec::new();
            for k in 0..a.len() {
                if k as u8 == begins[current] {
                    if !liste.is_empty() {
                        list.insert(liste.clone());
                        liste.clear();
                    }
                    liste.push(a[k]);

                    if current < begins.len() - 1 {
                        current += 1;
                    }
                } else {
                    liste.push(a[k]);
                }
            }
            list.insert(liste.clone());

            if list.len() == 1 {
                println!("yey");
            }
        }
    }
    println!("{}", sum);
}
