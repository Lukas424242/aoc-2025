use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let feld: Vec<Vec<char>> = input.lines().map(|x| x.trim().chars().collect()).collect();

    // egal immer ein 1 Beam
    let mut list: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 1;

    let mut posx = 0;
    for i in 0..feld[0].len() {
        if feld[0][i] == 'S' {
            posx = i;
            break;
        }
    }
    tracer(posx, 1, &feld, &mut list);
    println!("{}", list.len());

    let mut sum=0;
    tracercounter(posx, 1, &feld, &mut sum);
    println!("{}", sum);
}

fn tracer(posx: usize, posy: usize, feld: &Vec<Vec<char>>, liste: &mut HashSet<(usize, usize)>) {
    if let Some(x) = liste.get(&(posy, posx)) {
        return;
    } else {
        if posx < feld[0].len() && posy < feld.len() {
            if feld[posy][posx] == '^' {
                liste.insert((posy, posx));

                tracer(posx - 1, posy, feld, liste);
                tracer(posx + 1, posy, feld, liste);
            } else {
                tracer(posx, posy + 1, feld, liste);
            }
        }
    }
}

fn tracercounter(posx: usize, posy: usize, feld: &Vec<Vec<char>>, sum: &mut usize, memo: &mut HashMap<(usize,usize), usize>) -> usize {

    if let Some(x) =  memo.get(&(posy, posx)) {
        return *x;
        
    }
    else {
        
    
    if posy == feld.len()-1 {
        *sum+=1;
        return;
    } else {
        if posx < feld[0].len() && posy < feld.len() {
            if feld[posy][posx] == '^' {

                tracercounter(posx - 1, posy, feld, sum);
                tracercounter(posx + 1, posy, feld, sum);
            } else {
                tracercounter(posx, posy + 1, feld, sum);
            }
        }
    }
}
}
// ich weiß nicht warum es funktioniert, aber es funktioniert
// komisch dass es net mit der sum funktioniert
