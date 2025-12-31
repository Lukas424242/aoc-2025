use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("demo.txt").unwrap();
    let zahlen:Vec<Vec<f32>> = input.lines().map(|x|x.split(",").map(|x| x.parse().unwrap()).collect()).collect();


    let mut distances:Vec<[f32;3]> = Vec::new();
    for i in 0..zahlen.len(){
        let p1 = &zahlen[i];

        for j in 0..zahlen.len(){
            let p2 = &zahlen[j];

            if i != j {
                let a:f32 = (p1[0] -p2[0]).powf(2.0) as f32 + (p1[1] -p2[1]).powf(2.0) as f32 + (p1[2] -p2[2]).powf(2.0) as f32;
                let b:f32 = a.sqrt();

                distances.push([i as f32,j as f32,b]);
            }
        }
    }
   }
