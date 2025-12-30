use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let feld:Vec<Vec<char>> = input.lines().map(|x|x.trim().chars().collect()).collect();

    // egal immer ein 1 Beam
    let mut list:HashSet<(usize,usize)> =HashSet::new();
    let mut sum=1;

    let mut posx=0;
    for i in 0..feld[0].len(){
        if feld[0][i] =='S'{
            posx=i;
            break;
        }

    }
    tracer(posx, 1, &feld, &mut sum, &mut list);
    println!("{}", list.len());
    

   }

   fn tracer(posx:usize, posy:usize, feld:&Vec<Vec<char>>, sum:&mut usize,  liste:&mut HashSet<(usize,usize)>){

    if let Some(x) = liste.get(&(posy,posx)) {
        
        *sum-=1;
        return;
    }
    else {
        
    if  posx < feld[0].len() && posy < feld.len() {
        if feld[posy][posx] == '^' {
                    liste.insert((posy,posx));

            *sum+=2;
            tracer(posx-1, posy, feld, sum, liste);
            tracer(posx+1, posy, feld, sum, liste);
        }
        else {
            tracer(posx, posy+1, feld, sum, liste);
        }

        
    }
}

   }
