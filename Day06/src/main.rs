use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("demo.txt").unwrap();

    let mut daten: Vec<Vec<String>> = input
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.to_string()).collect())
        .collect();

    let operationen = daten[daten.len() - 1].clone();
    daten.remove(daten.len() - 1);

    let dat: Vec<Vec<usize>> = daten
        .iter()
        .map(|x| x.iter().map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut sum: usize = 0;

    for i in 0..operationen.len() {
        let mut list: Vec<usize> = Vec::new();

        for j in 0..dat.len() {
            list.push(dat[j][i]);
        }

        match operationen[i].as_str() {
            "+" => {
                let sump: usize = list.iter().sum();
                sum += sump;
            }
            "*" => {
                let sump: usize = list.iter().product();
                sum += sump;
            }
            _ => {
                unreachable!("fuck");
            }
        }
    }
    println!("{}", sum);

    // neue DS

    let mut d2: Vec<Vec<char>> = input
        .lines()
        .map(|x: &str| x.chars().map(|c| c).collect())
        .collect();
    d2.remove(d2.len() - 1);

    let mut zeichen = operationen.len() - 1;

    let mut liste: Vec<Vec<usize>> = Vec::new();

    let mut sump: usize=0;
    for i in (0..d2[0].len()).rev() {
        if iswhite(i, &d2) {

            
            match operationen[zeichen].as_str(){
                "+"=>{

                    for k in 0..liste.len(){
                        let l: &Vec<usize> = &liste[k];
                        sump+= l.iter().sum::<usize>();
                    }
                    liste.clear();


                },
                "*"=>{
                    for k in 0..liste.len(){
                        let l: &Vec<usize> = &liste[k];
                        sump+= l.iter().product::<usize>();
                    }
                    liste.clear();


                },
                _=> unreachable!("uck")
            }
            zeichen -= 1;
             
        } else {
            let mut zahlen: Vec<usize> = Vec::new();

            match operationen[zeichen].as_str() {
                "+" => {
                    for l in 0..d2.len() {
                        if d2[l][i] == ' ' {
                            zahlen.push(0);
                        } else {
                            let zahl = d2[l][i].to_digit(10).unwrap() as usize;
                            zahlen.push(zahl);
                        }
                    }
                }
                "*" => {
                    for l in 0..d2.len() {
                        if d2[l][i] == ' ' {
                            zahlen.push(1);
                        } else {
                            let zahl = d2[l][i].to_digit(10).unwrap() as usize;
                            zahlen.push(zahl);
                        }
                    }
                }

                _ => unreachable!("fucked"),
            }

            liste.push(zahlen);
        }
    }

    println!("{}", sump);
}

fn iswhite(spalte: usize, daten: &Vec<Vec<char>>) -> bool {
    for i in 0..daten.len() {
        if daten[i][spalte] != ' ' {
            return false;
        }
    }

    true
}
