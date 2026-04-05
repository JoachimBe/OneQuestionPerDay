/* 
# Dessiner un tableau a deux dimensions en Rust
Pour dessiner un tableau généré aléatoirement en Rust il faut commencer par importer le crate _rand::RngExt_ qui va nous permettre de générer des nombres aléatoires. 
Pour cela il faut ajouter la dépendance au fichier Cargo.toml comme suit:
<code>[dependencies]
rand = "0.10.0"</code>
*/

use rand::RngExt;
use std::io;

const TREE: char= '🌳';
const FIR_TREE: char= '🌲';
const PALM_TREE: char='🌴';
const BLANK_SPACE: char='🟩';
const TREES:[char; 4] = [TREE, FIR_TREE,PALM_TREE,BLANK_SPACE];
fn main() {
    //creation du Vecteur seed
    let mut seed: Vec<u8> = Vec::new();
    //initialisation taille(height * width) de la carte de manière aléatoire entre 5 et 9 compris
    let mut map_height = rand::rng();//creation de notre variable rand de type ThreadRng pour générer un nombre aléatoir dans une certaine range
    let map_height: u32 =  map_height.random_range(5..10);//range allant de 5 à 9
    seed.push(map_height as u8); // push height comme premier element dans mon Vecteur
    let mut map_width = rand::rng();
    let map_width: u32 =  map_width.random_range(5..10);
    seed.push(map_width as u8);// push width comme second element dans le vecteur.

    let mut cpt_i=  0; //initialisation compteur pour la hauteur
    while cpt_i < map_height{
        let mut cpt_j =0;
        while cpt_j < map_width{
            let mut tree_type= rand::rng(); 
            let tree_type : usize= tree_type.random_range(0..4);
            print!("{}", TREES[tree_type ]);// imprime un element de notre forêt( arbre ou espace blanc)
            seed.push(tree_type as u8);//push du numero de référence de l'élément dans le vecteur seed
            cpt_j+=1;
        }
        print!("\n");
        cpt_i+=1;
    }
    for elem in seed{
        print!("{}",elem);//imprime le vecteur seed element par element
    }
    println!("");
    println!("Enter your seed:");
    let mut user_seed: String= String::new();
    io::stdin().read_line(&mut user_seed).expect("error while reading the line");// attend l'user entry
    if user_seed.trim() != ""{ // si l'uitlisateur entre une donnée differente que empty Enter
        create_map(user_seed);// on appelle la fonction create_map(String)
    }
}

fn create_map(seed: String){
    let mut map: Vec<u8> = Vec::new();
    let mut index= 0;
    println!("");

    let checker= "0"; 
    for number in seed.trim().split(""){  //on decoupe la chaine de character
        if number.as_bytes()>= checker.as_bytes() { //permet de verifier que il n'y a pas d'élément vide dans le tableau
            map.push(number.parse().unwrap() ); //converti un string en integer
        }
    }
    
    let mut cpt_i= 0;
    while cpt_i <  map[0]{
        let mut cpt_j = 0;
        while cpt_j < map[1]{
            print!("{}", TREES[map[index as usize +2] as usize]);// imprime un element de la carte
            cpt_j+=1;
            index+=1;
        }
        cpt_i+=1;

        print!("\n");    
    }
}



