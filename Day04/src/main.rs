use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let feld: Vec<Vec<char>> = input.lines().map(|x| x.trim().chars().collect()).collect();
    println!("");

    // Part 1

    let mut sum = 0;
    for i in 0..feld.len() {
        for j in 0..feld[i].len() {
            if feld[i][j] == '@' {
                sum += checkrolls(j, i, &feld);
                println!("y {} x {}", j, i);
            }
        }
    }

    println!("{}", sum);

    // Part 2

    let mut removed = 0;
    loop {

        let mut sum=0;
        let mut koordinaten:Vec<[usize;2]> = Vec::new();
        for i in 0..feld.len() {
            for j in 0..feld[i].len() {
                if feld[i][j] == '@' {
                    sum += checkrolls(j, i, &feld);
                }
            }
        }
    }
}

fn checkrolls(posx: usize, posy: usize, map: &Vec<Vec<char>>) -> i32 {
    // y x
    let array = [
        [-1, 0],
        [1, 0],
        [-1, -1],
        [-1, 1],
        [0, 1],
        [0, -1],
        [1, -1],
        [1, 1],
    ];

    let x = posx as i32;
    let y = posy as i32;
    let mut sum = 0;
    for i in 0..array.len() {
        let yn = y + array[i][0];
        let xn = x + array[i][1];

        if 0 <= xn && 0 <= yn && yn < map.len() as i32 && xn < map[0].len() as i32 {
            if map[yn as usize][xn as usize] == '@' {
                sum += 1;
            }
        }
    }

    if sum < 4 {
        return 1;
    } else {
        return 0;
    }
}

/*
1 2 3
4 X 5
6 7 8
 */
