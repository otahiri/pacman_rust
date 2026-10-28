use crate::modules::Direction;
use bevy::ecs::resource::Resource;
#[derive(Resource)]
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

    fn set_tile(&mut self, x: usize, y: usize, new_value: usize) {
        self.data[y][x] = new_value;
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_new_pos(&self, x: usize, y: usize, dir: &Direction) -> Option<(usize, usize)> {
        let (dx, dy) = dir.values();
        let new_x = x.checked_add_signed(dx as isize)?;
        let new_y = y.checked_add_signed(dy as isize)?;
        if new_x >= self.width || new_y >= self.height {
            return None;
        }
        return Some((new_x, new_y));
    }

    pub fn check_wall(&self, x: usize, y: usize, dir: &Direction) -> bool {
        let shift = dir.shift();
        self.get_tile(x, y) & (1 << shift) != 0
    }

    pub fn connect_cells(&mut self, direction: Direction, cords: &(usize, usize)) {
        let (x, y) = *cords;
        let mut cell: usize = self.get_tile(x, y);
        let shift = direction.shift();
        if let Some((new_x, new_y)) = self.get_new_pos(x, y, &direction) {
            let oshift = direction.opposite().shift();
            let mut opposite_cell: usize = self.get_tile(new_x, new_y);
            let mask = 1 << shift;
            let opposite_mask = 1 << oshift;
            cell &= !mask;
            opposite_cell &= !opposite_mask;
            self.set_tile(x, y, cell);
            self.set_tile(new_x, new_y, opposite_cell);
        } else {
            print!("trying to connect cells outside the maze\n")
        }
    }

    fn get_neighbors(
        &self,
        parent: (usize, usize),
    ) -> Vec<((usize, usize), (usize, usize), Direction)> {
        let mut edges: Vec<((usize, usize), (usize, usize), Direction)> = Vec::with_capacity(4);
        let (x, y) = parent;
        for dir in Direction::ALL {
            if let Some((new_x, new_y)) = self.get_new_pos(x, y, &dir) {
                edges.push(((x, y), (new_x, new_y), dir));
            }
        }
        return edges;
    }

    pub fn create_maze(&mut self) {
        let (width, height) = self.get_size();
        let x: usize = rand::random_range(0..width);
        let y: usize = rand::random_range(0..height);
        let mut frontier: Vec<((usize, usize), (usize, usize), Direction)> =
            Vec::with_capacity(width * height);
        let mut visited: Vec<(usize, usize)> = Vec::with_capacity(width * height);
        visited.push((x, y));
        frontier.extend(self.get_neighbors((x, y)));
        while frontier.len() > 0 {
            let idx = rand::random_range(0..frontier.len());
            let (parent, child, dir) = frontier.remove(idx);
            if !visited.contains(&child) {
                visited.push(child);
                self.connect_cells(dir, &parent);
                frontier.extend(self.get_neighbors(child));
            }
        }
    }
}
