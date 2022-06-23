use crate::{
    consts::tiles,
    domain::{commons::position::Position, tile::tile::Tile},
};

use super::board::Board;

pub struct BoardCreator;

impl BoardCreator {
    pub fn create_from(tile_symbols_matrix: &[Vec<char>], name: &str) -> Board {
        let mut tiles = Vec::with_capacity(tile_symbols_matrix.len());

        for row_index in 0..tile_symbols_matrix.len() {
            let mut new_row = Vec::with_capacity(tile_symbols_matrix.len());
            let row = &tile_symbols_matrix[row_index];

            for column_index in 0..row.len() {
                let tile_symbol = row[column_index];
                let tile_kind = tiles::SYMBOLS_AND_KINDS.get(&tile_symbol);

                new_row.push(Tile::new(
                    *tile_kind.unwrap(),
                    Position::new(column_index as i32, row_index as i32),
                ));
            }

            tiles.push(new_row);
        }

        Board::new(name.to_string(), tiles)
    }
}
