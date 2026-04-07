use std::io;

fn main() {
    println!("entrer une valeur RGBA séparer par une virgule: ");
    let mut user_entry: String= String::new();
    io::stdin().read_line(&mut user_entry).expect("Failed to readline");
   // println!("{}", user_entry);
   let mut users_rgbacolor: Vec<f32> = Vec::new();
     for color in user_entry.split(","){
        users_rgbacolor.push(color.trim().parse().expect("falied to parse as f32"))
     }

    let background_red:f32= 0.0;
    let background_green:f32= 0.0;
    let background_blue:f32= 0.0;
    let background_colors =  vec![background_red, background_green, background_blue];

    //print!("\x1b[38;2;{user_rgbacolor[0]};{user_rgbacolor[1]};{char_blue}m ■\x1b[0m");
     calcul_melange_couleur(users_rgbacolor, background_colors)
}
//(1-a) × Cf + a × C
fn calcul_melange_couleur(cf: Vec<f32>, c: Vec<f32>){
    let a = cf[3];
    let r = ((1.0 -a) * cf[0] + a *  c[0]) as u8;
    let g = ((1.0 -a) * cf[1] + a *  c[1]) as u8;
    let b = ((1.0 -a) * cf[2] + a *  c[2]) as u8;
    print!("\x1b[38;2;{r};{g};{b}m ■");
    //return vec![r,g,b]
}