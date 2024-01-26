mod classes;

use classes::connect4::Connect4;

fn main() {
    println!("Hello, World!");
    let mut _game = Connect4::new();
    let mut _str = String::new(); 
    _game.show();
    _game.check_win_line(0);
    _game.check_win_column(0);
    _game.check_win_diagonal();
    _game.change_player();
    _game.play(0, &mut _str);
}