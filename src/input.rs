use std::io;

pub fn get_int() -> i32 {
    
    let mut input: String = "".to_string();
    let mut running: bool = true;
    let mut input32: i32 = 0;

    while running {

        io::stdin().read_line(&mut input).expect("Failed");

        input = input.to_lowercase().replace(&['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','!', '"', '§', '$', '%', '&', '/', '(', ')', '=', '?', '{', '[', ']', '}', ' ', '+', '-', '*', '#', '\u{0027}', '~',',',';','.',':','_','@', '^'], "");
        
        if input.trim() == "" {
        
            println!("Please input a number!");
        
        } else {

            input32 = input.replace('-', "").trim().parse::<i32>().unwrap();
            running = false
        }
    }
    return input32;
}
