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

    let user_color =  RGBA::new( 
        users_rgbacolor[0],
        users_rgbacolor[1],
        users_rgbacolor[2],
        users_rgbacolor[3]);

    let background_red:f32= 0.0;
    let background_green:f32= 0.0;
    let background_blue:f32= 0.0;
    let background_colors =  vec![background_red, background_green, background_blue];

    

    //print!("\x1b[38;2;{user_rgbacolor[0]};{user_rgbacolor[1]};{char_blue}m ■\x1b[0m");
     blend_color(user_color.expect(" failed to send RGBA to function").get_color(), background_colors)
}
//(1-a) × Cf + a × C
fn blend_color(cf: Vec<f32>, c: Vec<f32>){
    let a = cf[3];
    let r = ((1.0 -a) * cf[0] + a *  c[0]) as u8;
    let g = ((1.0 -a) * cf[1] + a *  c[1]) as u8;
    let b = ((1.0 -a) * cf[2] + a *  c[2]) as u8;
    print!("\x1b[38;2;{r};{g};{b}m ■\x1b[0m");
    //return vec![r,g,b]
}


pub struct RGBA{
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl RGBA{
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) ->Result<RGBA, &'static str>{
        let colors_vec = vec![red,green,blue];
        for color in colors_vec {
            if color < 0.0 || color > 255.0{
                return Err("One or many value of the RGB are incorect, please specify a value between 0 and 255")
            }else if alpha < 0.0 || alpha > 1.0{
                return Err(" Alpha value should be between 0.0 and 1.0")
            }
        }
        Ok(RGBA{
            red,
            green,
            blue,
            alpha,
        })
        
    }
    pub fn get_color(&self)-> Vec<f32>{
        vec![self.red, self.green, self.blue, self.alpha]
    }
}