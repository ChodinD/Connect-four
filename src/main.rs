mod input;
use std::vec;

#[derive(Clone)]
struct Chip {
    exists: bool,
    player: usize
}

fn main() {

    let mut board: Vec<Chip> = vec![Chip {exists: false, player: 0}; 42];
    let mut scoreboard: (i8,i8) = (0,0);

    fn show_board (board: &Vec<Chip>) {

        let chip_render = vec![' ','X','O'];

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

    fn get_lowest_chip_in_colum (colum: i32, board: &Vec<Chip>) -> usize {
        
        let mut current_chip = colum as usize -1;

        // add until at bottom
        while (current_chip as i8) < 35 && !board[current_chip+7].exists {current_chip+=7;}
        return current_chip;
    }
    
    // game loop

    loop  {
        let mut current_player: usize = 1;
        let mut selected_colum: i32 = 0;

        show_board(&board);
        
        // pasta code; TODO: add spaghetti
        while selected_colum != 1 && selected_colum != 2 && selected_colum != 3 && selected_colum != 4 && selected_colum != 5 && selected_colum != 6 && selected_colum != 7 {
            
            println!("Input colum: [1-7]");
            selected_colum = input::get_int();
        }
        
        let thrown_chip = get_lowest_chip_in_colum(selected_colum, &board);

        board[thrown_chip].exists = true;
        board[thrown_chip].player = current_player.clone();
    }
}
