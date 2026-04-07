use std::io;

fn main() {
    println!("enter RGBA value, separate with a comma: ");

    //get user entry
    let mut user_entry: String= String::new();
    io::stdin().read_line(&mut user_entry).expect("Failed to readline");

    //extract value from user entry and store it in a vector
   let mut users_rgba_color: Vec<f32> = Vec::new();
     for color in user_entry.split(","){
        users_rgba_color.push(color.trim().parse().expect("falied to parse as f32"))
     }

    //setting background value to 0, used in calculation 
    let background_red:f32= 0.0;
    let background_green:f32= 0.0;
    let background_blue:f32= 0.0;
    let background_colors =  vec![background_red, background_green, background_blue];

     blend_color(users_rgba_color, background_colors)
}

fn blend_color(front_color: Vec<f32>, background_color: Vec<f32>){
    let a = front_color[3];

    //Math for color balancing with alpha
    let r = ((1.0 -a) * front_color[0] + a *  background_color[0]) as u8;
    let g = ((1.0 -a) * front_color[1] + a *  background_color[1]) as u8;
    let b = ((1.0 -a) * front_color[2] + a *  background_color[2]) as u8;
    print!("\x1b[38;2;{r};{g};{b}m ■\x1b[0m");
} 