use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string, mem,
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

    let mut memo:HashMap<(usize,usize), usize>= HashMap::new();
    tracercounter(posx, 1, &feld, &mut memo);
    println!("{}", memo.get(&(2,posx)).unwrap());
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

fn tracercounter(posx: usize, posy: usize, feld: &Vec<Vec<char>>, memo:&mut HashMap<(usize,usize), usize>) -> usize{

    if let Some(x) = memo.get(&(posy,posx))  {
        return *x;
        
    }
    else {


        if posy == feld.len()-1{
            return 1;
        }

        if posx < feld[0].len() && posy < feld.len() {

            if feld[posy][posx] == '^' {

                let a = tracercounter(posx - 1, posy, feld, memo);
                let b = tracercounter(posx + 1, posy, feld, memo);

                if let Some(x) = memo.get_mut(&(posy, posx)) {
                    *x+= a +b;
                    return *x;
                    
                }
                else {
                    memo.insert((posy, posx), a+b);
                    return a+b;
                }

            } else {
                return tracercounter(posx, posy + 1, feld, memo);
            }

        }
        else {
            unreachable!("fucked");
        }
        
        
    }
    
}
// ich weiß nicht warum es funktioniert, aber es funktioniert
// komisch dass es net mit der sum funktioniert
