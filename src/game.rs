
pub mod game {

    // TODO: private vs public methods in private module

    use std::collections::HashMap;

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
        fn get_tile(&self, x: usize, y: usize) -> Option<&MapTile>;
    }

    pub struct MapTile {
        pub x: usize,
        pub y: usize,
        pub flag_toggled: bool,
        pub has_bomb: bool,
    }

    pub struct GameState {
        pub map: HashMap<String, MapTile>,
    }

    pub struct Game {
        width: usize,
        height: usize,
        game_state: GameState,
    }

    pub fn new_game(width: usize, height: usize) -> Game {
        let map = generate_map(width, height);
        Game {
            width,
            height,
            game_state: GameState { map },
        }
    }

    pub fn generate_map(x: usize, y: usize) -> HashMap<String, MapTile> {
        let mut hm = HashMap::new();
        for x_cord in 0..x {
            for y_cord in 0..y {
                let map_key = format!("{}:{}", x_cord, y_cord);
                let map_tile = MapTile { x: x_cord, y: y_cord, has_bomb: false, flag_toggled: false };

                hm.insert(
                    map_key,
                    map_tile,
                );
            }
        }
        hm
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
        fn get_tile(&self, x: usize, y: usize) -> Option<&MapTile> {
            let key = &format!("{}:{}",x, y);
            self.game_state.map.get(key)
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

    #[test]
    fn test_map() {
        let new_game = game::new_game(10, 10);

        let res = new_game.get_tile(1, 1).unwrap().to_owned();
        assert_eq!(false, res.has_bomb);
        assert_eq!(false, res.flag_toggled);

    }
}
