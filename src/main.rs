use std::io; //utilisaztion de la Rust Standard Library (Input/output)

//création de constante par couleur
const RED:&str ="[91m";
const GREEN:&str ="[92m";
const BLUE:&str = "[94m";
const CYAN:&str = "[96m";
const YELLOW:&str = "[93m";
const PURPLE:&str = "[95m";

//création d'un tableau de constante
const COLORS :[&str;6]  = [RED, GREEN, BLUE,CYAN, YELLOW, PURPLE ];

fn main() {
    let mut index = 0;//index qui  va iterer a chaque Enter
    loop{ //boucle du programme
        if index >= COLORS.len(){ //si l'index est plus grand ou égale que la taille de mon tableau
            index =0 ;//je resset l'index a zéro
        }
        println!("\x1b{}Hello World", COLORS[index]);
      
      //me permet d'attendre chaque Enter avant que la loop ne continue 
      let mut enter_is_pressed = String::new(); 
        io::stdin()//appelle la fonction stdin
            .read_line(&mut enter_is_pressed)
            .expect("Failed to read line"); 
        index+=1;// ajoute +1 a l'index
    }
    
}

/*
RED = '\x1b[91m'
GREEN = '\x1b[92m'
BLUE = '\x1b[94m'
CYAN = '\x1b[96m'
YELLOW = '\x1b[93m'
PURPLE = '\x1b[95m'
 */