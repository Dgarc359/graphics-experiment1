pub mod game {

    // TODO: private vs public methods in private module

    pub trait ExternalMinesweeper {
        fn get_width(&self) -> usize;
        fn get_height(&self) -> usize;
        /// Toggle the presence of a flag on a tile.
        fn toggle_flag(&self, x: usize, y: usize);
        /// Explore a tile. If it was a mine, BOOM! If not, flood fill mine scouting out from it.
        ///
        /// Returns true if BOOM!
        fn explore_tile(&self, x: usize, y: usize) -> bool;
        /// Return whether a tile should be displayed as blank, a flag, a cleared mine, a mine count 1-8, or a kaboom.
        fn get_tile(&self, x: usize, y: usize) -> MapTile;
    }

    #[allow(dead_code)]
    pub struct MapTile {
        x: usize,
        y: usize,
        flag_toggled: bool,
        has_bomb: bool,
    }


    pub struct GameState {
        map: Vec<&'static MapTile>,
    }

    pub struct Game {
        width: usize,
        height: usize,
        game_state: GameState,
    }

    #[allow(dead_code)]
    pub fn new_game(width: usize, height: usize) -> Game {
        let map = generate_map();
        Game { width, height, game_state: GameState { map } }
    }

    #[allow(dead_code)]
    pub fn generate_map() -> Vec<&'static MapTile> {
        vec![]
    }

    impl ExternalMinesweeper for Game {
        fn get_width(&self) -> usize {
            self.width
        }

        fn get_height(&self) -> usize {
            self.height
        }

        /// Toggle the presence of a flag on a tile.
        fn toggle_flag(&self, _x: usize, _y: usize) {}
        /// Explore a tile. If it was a mine, BOOM! If not, flood fill mine scouting out from it.
        ///
        /// Returns true if BOOM!
        fn explore_tile(&self, _x: usize, _y: usize) -> bool {
            true
        }
        /// Return whether a tile should be displayed as blank, a flag, a cleared mine, a mine count 1-8, or a kaboom.
        fn get_tile(&self, x: usize, y: usize) -> MapTile {
            MapTile {
                x,
                y,
                has_bomb: true,
                flag_toggled: false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game::*;

    #[test]
    fn test_new() {
        let new_game = game::new_game(10, 10);

        assert_eq!(new_game.get_width(), 10);
        assert_eq!(new_game.get_height(), 10);
    }
}
