const GRID_SIZE: usize = 9;
const GRID_CENTER: usize = (GRID_SIZE - 1) / 2;

pub struct SpellGrid {
    pub spell: Spell,
    pub grid_data: [[Option<SpellPiece>; GRID_SIZE]; GRID_SIZE],

    empty: bool,

    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
}

impl SpellGrid {
    fn new(spell: Spell) -> SpellGrid {
        let mut grid_data = [[None; GRID_SIZE]; GRID_SIZE];
        for piece in spell.pieces.iter() {
            grid_data[piece.x][piece.y] = Some(*piece);
        }
        SpellGrid { spell, grid_data }
    }

    fn piece_at(&self, x: usize, y: usize) -> Option<SpellPiece> {
        if x >= GRID_SIZE || y >= GRID_SIZE {
            None
        } else {
            self.grid_data[x][y]
        }
    }

    fn set_piece_at(&mut self, piece: SpellPiece) {
        self.grid_data[piece.x][piece.y] = Some(piece);
    }

    fn recalculate_boundaries(&mut self) {
        empty = true;
        self.left = GRID_SIZE;
        self.right = 0;
        self.top = GRID_SIZE;
        self.bottom = 0;

        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                if self.grid_data[x][y].is_some() {
                    if x < self.left {
                        self.left = x;
                    }
                    if x > self.right {
                        self.right = x;
                    }
                    if y < self.top {
                        self.top = y;
                    }
                    if y > self.bottom {
                        self.bottom = y;
                    }
                }
            }
        }
    }

    fn get_size(&self) -> usize {
        recalculate_boundaries();

        if empty {
            return 0;
        }

        (self.right - self.left + 1) * (self.bottom - self.top + 1)
    }
}
