use std::io;

fn main() {
    loop{
        println!("Nettoyer la console?(oui/non)");
        let mut catch_answer =  String::new();
        io::stdin()
            .read_line(&mut catch_answer)
            .expect("Failed to read line");

        let catch_answer: &str = catch_answer.trim();
        match catch_answer{
            "Oui"| "oui"|"y" => print!("\x1bc"),
            "Non"|"non"| "no" => print!("Ok on laisse comme ça."),
            _=>print!("répondez pour \"oui\" ou \"non\""),
        }

        
    }
}
