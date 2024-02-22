mod classes;

use classes::connect4::Connect4;
use std::io;

fn main() {
    loop {
        println!("Hello, World! New Grid.");
        let mut _game = Connect4::new();
        let mut _input = String::new(); 
        
    
        while !_game.finish {
            _input.clear();
            _game.show();
            println!("Please enter a number corresponding to a column, or \"exit\" to end the grid:");
            io::stdin().read_line(&mut _input).expect("Error when reading input.");
            if _input.trim() == "exit" {
                println!("You have entered 'exit' as a number. This grid is finished.");
                return;
            }

            let number: Result<i32,_> = _input.trim().parse();
            
            match number {
                Ok(number) => {
                    _game.play(number as u8);
                    continue;
                }
                Err(error_parsing) => {
                    println!("Error: Parsing. {}", error_parsing);
                }
            }


        }

    }
        
}