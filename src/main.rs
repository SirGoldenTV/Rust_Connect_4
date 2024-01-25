mod classes;

use classes::connect4::Connect4;

fn main() {
    println!("Hello, World!");
    let _game = Connect4::new();
    _game.show();
    _game.check_line(0);
    _game.check_column(0);
    _game.check_diagonal();
}