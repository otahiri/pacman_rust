crate::modules::Direction;
use rand;
pub struct Maze {
    data: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![vec![15; width]; height],
            width,
            height,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> usize {
        self.data[y][x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, new_value: usize) {
        self.data[y][x] = new_value;
    }

    pub fn get_size(self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn connect_cells(self, direction: Direction, cords: (usize, usize)) {
        let (x, y) = cords;
        let cell: usize = self.get_tile(x, y);
        let (dx, dy, shift) = direction;
        let (ox, oy) = (x + dx, x + dy);
        let (_, _, oshift) = direction.oppsite();
        let opposite_cell: usize = self.get_tile(x + dx, y + dy);
        let mask = 1 << shift;
        let opposite_mask = 1 << oshift;
        cell &= !mask;
        opposite_cell &= !opposite_mask;
        self.set_tile(x, y, cell);
        self.set_tile(ox, oy, cell);
    }
}
