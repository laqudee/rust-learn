mod game_body;

use game_body::margic_maze::MagicMaze;
use game_body::ordinary_maze::OrdinaryMaze;

fn main() {
    let ordinary_maze = OrdinaryMaze::new();
    game_body::game::run(ordinary_maze);

    let magic_maze = MagicMaze::new();
    game_body::game::run(magic_maze);
}
