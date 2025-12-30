use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

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

    let mut liste: Vec<Vec<char>> = Vec::new();

    let mut sum: usize = 0;
    for i in (0..d2[0].len()).rev() {

        if iswhite(i, &d2) {
            let mut zahlen: Vec<usize> = Vec::new();
            for k in 0..liste.len() {
                let h = &liste[k];

                let mut a = 1;
                let mut sum1 = 0;
                for j in (0..h.len()).rev() {
                    if h[j]!='X' {                        
                        sum1+= h[j].to_digit(10).unwrap() as usize  *a;
                        a = a*10; 
                    }

                }
                
                zahlen.push(sum1);
            
            }

            let mut sump=0;
            match operationen[zeichen].as_str() {
                "+" => {
                     sump = zahlen.iter().sum();
                    sum += sump;
                }
                "*" => {
                     sump = zahlen.iter().product();
                    sum += sump;
                }
                _ => {
                    unreachable!("fuck");
                }
            }
            liste.clear();

            println!("");

            if i!=0{
            zeichen -= 1;
        
            }
    
        } else {
            let mut zahlen: Vec<char> = Vec::new();

            for l in 0..d2.len() {
                if d2[l][i] == ' ' {
                    zahlen.push('X');
                } else {
                    println!("{}",d2[l][i] );
                    zahlen.push(d2[l][i]);
                }
            }

            liste.push(zahlen);
        }
    }

    if liste.len() !=0{
        let mut zahlen: Vec<usize> = Vec::new();
            for k in 0..liste.len() {
                let h = &liste[k];

                let mut a = 1;
                let mut sum1 = 0;
                for j in (0..h.len()).rev() {
                    if h[j]!='X' {                        
                        sum1+= h[j].to_digit(10).unwrap() as usize  *a;
                        a = a*10; 
                    }

                }
                
                zahlen.push(sum1);
            
            }

            let mut sump=0;
            match operationen[zeichen].as_str() {
                "+" => {
                     sump = zahlen.iter().sum();
                    sum += sump;
                }
                "*" => {
                     sump = zahlen.iter().product();
                    sum += sump;
                }
                _ => {
                    unreachable!("fuck");
                }
            }
    }

    println!("{}", sum);
}

fn iswhite(spalte: usize, daten: &Vec<Vec<char>>) -> bool {
    for i in 0..daten.len() {
        if daten[i][spalte] != ' ' {
            return false;
        }
    }

    true
}
