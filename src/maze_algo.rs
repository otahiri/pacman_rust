use rand;
pub struct Maze {
    data: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Self { data: vec![vec![15; width]; height], width, height}

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
}
