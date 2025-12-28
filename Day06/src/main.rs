use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    
    let mut daten:Vec<Vec<String>> = input.lines().map(|x|x.split_whitespace().map(|x|x.to_string()).collect()).collect();

    let operationen = daten[daten.len()-1].clone();
    daten.remove(daten.len()-1);

    let dat:Vec<Vec<usize>> = daten.iter().map(|x|x.iter().map(|x|x.parse().unwrap()).collect()).collect();

    let mut sum: usize=0;

    for i in 0..operationen.len(){

        let mut list:Vec<usize> = Vec::new();

        for j in 0..dat.len(){
            list.push(dat[j][i]);
        }

        match operationen[i].as_str() {

            "+" =>{
                let sump:usize = list.iter().sum();
                sum+=sump;

            },
            "*"=>{
                let sump:usize = list.iter().product();
                sum+=sump;
            }
            _=>{
                unreachable!("fuck");
            }
            
        }
    }
    println!("{}", sum);
   }
