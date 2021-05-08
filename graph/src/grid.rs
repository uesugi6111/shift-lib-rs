#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Grid<T>(Vec<Vec<T>>);
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridIndex {
    pos: (usize, usize),
    h: usize,
    w: usize,
}

impl<T> Grid<T> {
    pub fn new(table: Vec<Vec<T>>) -> Self {
        Grid(table)
    }
}

impl<T> std::ops::Index<GridIndex> for Grid<T> {
    type Output = T;

    fn index(&self, g: GridIndex) -> &Self::Output {
        let (i, j) = g.pos;
        &self.0[i][j]
    }
}
impl GridIndex {
    pub fn new(pos: (usize, usize), h: usize, w: usize) -> Self {
        Self { pos, h, w }
    }
    pub fn neighbors(&self) -> Vec<Self> {
        let dx = [1, 0, -1, 0];
        let dy = [0, 1, 0, -1];
        let (y, x) = self.pos;
        let mut ret = Vec::new();
        for i in 0..4 {
            let ny = dy[i] + y as i32;
            let nx = dx[i] + x as i32;
            if 0 <= ny && ny < self.h as i32 && 0 <= nx && ny < self.w as i32 {
                ret.push(Self::new((ny as usize, nx as usize), self.h, self.w))
            }
        }
        ret
    }
}
#[test]
fn t() {
    let grid = Grid::new(vec![vec!['#', '.', '#'], vec!['.', '#', '.']]);
    let pos = GridIndex::new((1, 0), 2, 3);
    assert_eq!(
        pos.neighbors(),
        vec![GridIndex::new((1, 1), 2, 3), GridIndex::new((0, 0), 2, 3)]
    )
}
