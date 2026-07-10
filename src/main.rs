use std::{vec, io};

#[derive(Clone)]
struct Chip {
    exists: bool,
    player: usize
}

fn main() {

    let mut board: Vec<Chip> = vec![Chip {exists: false, player: 0}; 42];
    let mut scoreboard: (i8,i8) = (0,0);

    fn show_board (board: &Vec<Chip>) {

        let chip_render = vec![" ","\x1b[94mX\x1b[0m","\x1b[91m0\x1b[0m"];

        for y in 0..6 {
            for x in 0..7 {
                print!("|{}", chip_render[board[y*7+x].player]);
            }
            print!("|\n");
        }

        for i in 1..8 {
            print!(" {}",i)
        } print!("\n");
    }

    fn get_lowest_chip_in_colum (colum: &i32, board: &Vec<Chip>) -> usize {
        
        // check for 0
        if colum < &1 {return 42;}

        let mut current_chip = colum.clone() as usize -1;

        // check for out of bounds or full colum
        if current_chip > 41 || board[current_chip].exists {
            return 42;
        }

        // add until at next chip exists and return
        while (current_chip as i8) < 35 && !board[current_chip+7].exists {current_chip+=7;}
        return current_chip;
    }

    fn check_for_wins (current_player: &usize, last_chip: &usize, board: &Vec<Chip>, scoreboard: &mut (i8,i8)) -> bool {

        let mut is_win: bool = false;
        let mut streak: [i8; 4] = [0,0,0,0];

        for i in 0..7 as usize { //check for wins
            if i == 3 {continue;}

            //check for out of bounds and increase streak
            if !(last_chip % 7+i < 3|| last_chip % 7+i-3 > 6) {if board[last_chip+i-3].player == current_player.clone() {streak[0]+=1;} else {streak[0] = 0}}
            
            if !(last_chip+i*7 < 21 || last_chip+i*7-21 > 41) {if board[last_chip+i*7-21].player == current_player.clone() {streak[1]+=1;} else {streak[1] = 0}}

            if !(last_chip+i*8 < 24 || last_chip+i*8-24 > 41) && (last_chip+i*8-24)/7+3 == (last_chip/7)+i {if board[last_chip+i*8-24].player == current_player.clone() {streak[2]+=1;} else {streak[2] = 0}}

            if !(last_chip+i*6 < 18 || last_chip+i*6-18 > 41) && (last_chip+i*6-18)/7+3 == (last_chip/7)+i {if board[last_chip+i*6-18].player == current_player.clone() {streak[3]+=1;} else {streak[3] = 0}}

            // check if someone won
            for i in 0..4 {
                if streak[i] == 3 {is_win = true;}
            }
        }

        // check if there is a win, then check if its player 1, then add 1 to their score, else add 1 to player 2's score
        if is_win {if current_player == &1 {scoreboard.0 += 1} else {scoreboard.1 +=1}}
        return is_win;
    }

    fn get_int () -> i32 {
    
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
    
    // game loop
    let mut current_player: usize = 1;

    loop {
        let mut selected_colum: i32 = 43;

        println!("Scores:\n\x1b[94m{}\x1b[0m : \x1b[91m{}\x1b[0m", scoreboard.0 ,scoreboard.1);

        while !matches!(&selected_colum, 1..8) || get_lowest_chip_in_colum(&selected_colum, &board) == 42{
            show_board(&board);
            
            println!("Input colum: [1-7]");
            selected_colum = get_int();

            if !matches!(&selected_colum, 1..8) {println!("Please choose a vailid colum!");}
            else if get_lowest_chip_in_colum(&selected_colum, &board) == 42 {println!("This colum is full!");}
        }
            
        let thrown_chip = get_lowest_chip_in_colum(&selected_colum, &board);

        board[thrown_chip].exists = true;
        board[thrown_chip].player = current_player.clone();

        if check_for_wins(&current_player, &thrown_chip, &board, &mut scoreboard) {
            println!("Player {} wins!",&current_player);
            current_player = 2; // set player to 2 to have it set to 1 laters
            board = vec![Chip {exists: false, player: 0}; 42];
        }

        if current_player == 1 {current_player = 2} else {current_player = 1}
    }
}
