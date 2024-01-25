mod classes;

use classes::connect4::Connect4;

fn main() {
    println!("Hello, World!");
    let mut _game = Connect4::new();
    _game.show();
    _game.check_line(0);
    _game.check_column(0);
    _game.check_diagonal();
    _game.change_player();
    _game.play(0);
}