#[derive(Clone)]
pub struct World {
    size: usize,
    cells: Vec<bool>,
}

impl World {
    pub fn new(size: usize) -> World {
        let cells = vec![false; size * size];

        World { size, cells }
    }

    pub fn state(&self) -> (&Vec<bool>, usize) {
        (&self.cells, self.size)
    }

    #[inline]
    fn pos(&self, x: i32, y: i32) -> usize {
        let x = x.rem_euclid(self.size as i32);
        let y = y.rem_euclid(self.size as i32);
        ((x * self.size as i32) + y) as usize
    }

    pub fn is_alive(&self, x: i32, y: i32) -> bool {
        self.cells[self.pos(x, y)]
    }

    pub fn spawn(&mut self, x: i32, y: i32) {
        let pos = self.pos(x, y);
        self.cells[pos] = true;
    }

    pub fn kill(&mut self, x: i32, y: i32) {
        let pos = self.pos(x, y);
        self.cells[pos] = false;
    }

    fn neighbors(&self, x: i32, y: i32) -> Vec<bool> {
        (-1..=1)
            .flat_map(|dx| (-1..=1).map(|dy| (dx, dy)).collect::<Vec<(i32, i32)>>())
            .filter(|(x, y)| x != &0 || y != &0)
            .map(|(dx, dy)| self.is_alive(x + dx, y + dy))
            .collect::<Vec<bool>>()
    }

    pub fn alive_neighbors(&self, x: i32, y: i32) -> i32 {
        self.neighbors(x, y).iter().filter(|&&v| v).count() as i32
    }

    pub fn step(self) -> World {
        let mut new_world = self.clone();

        for x in 0..(self.size as i32) {
            for y in 0..(self.size as i32) {
                let alive_neighbors = self.alive_neighbors(x, y);

                if self.is_alive(x, y) {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        new_world.kill(x, y);
                    }
                } else {
                    if alive_neighbors == 3 {
                        new_world.spawn(x, y);
                    }
                }
            }
        }

        new_world
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    #[test]
    fn test_new_world() {
        World::new(100);
    }

    #[test]
    fn is_alive() {
        let world = World::new(100);
        assert_eq!(world.is_alive(1, 5), false);
    }

    #[test]
    fn get_wraps_around() {
        let mut world = World::new(5);
        world.spawn(1, 2);
        assert_eq!(world.is_alive(6, 7), true);
    }

    #[test]
    fn test_spawn() {
        let mut world = World::new(5);
        world.spawn(1, 1);
        assert_eq!(world.is_alive(1, 1), true);
    }

    #[test]
    fn test_kill() {
        let mut world = World::new(5);
        world.spawn(1, 1);
        world.kill(1, 1);
        assert_eq!(world.is_alive(1, 1), false);
    }

    #[test]
    fn alive_neighbors() {
        let mut world = World::new(10);
        world.spawn(1, 1);
        world.spawn(1, 3);

        assert_eq!(world.alive_neighbors(1, 2), 2);
    }

    #[test]
    fn test_step_single_cell_dies() {
        let mut world = World::new(10);
        world.spawn(1, 1);
        world = world.step();
        assert_eq!(world.is_alive(1, 1), false);
    }

    #[test]
    fn test_step_three_cells_rotate() {
        let mut world = World::new(10);
        world.spawn(2, 1);
        world.spawn(2, 2);
        world.spawn(2, 3);
        world = world.step();
        assert_eq!(world.is_alive(1, 2), true);
        assert_eq!(world.is_alive(2, 2), true);
        assert_eq!(world.is_alive(3, 2), true);
        assert_eq!(world.is_alive(2, 1), false);
        assert_eq!(world.is_alive(2, 3), false);
    }
}
