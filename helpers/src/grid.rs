use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;

pub type Coord = (usize, usize);

#[derive(Clone)]
pub struct Grid<T> {
    pub size: (usize, usize),
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(size: (usize, usize), cells: Vec<T>) -> Self {
        assert_eq!(size.0 * size.1, cells.len());

        Grid { size, cells }
    }

    pub fn len(&self) -> usize {
        self.size.0 * self.size.1
    }

    pub fn get(&self, coord: Coord) -> &T {
        &self.cells[self.index(coord)]
    }

    pub fn maybe_get(&self, coord: (isize, isize)) -> Option<&T> {
        if coord.0 < 0
            || coord.1 < 0
            || coord.0 >= self.size.0 as isize
            || coord.1 >= self.size.1 as isize
        {
            return None;
        }

        Some(&self.cells[self.index((coord.0 as usize, coord.1 as usize))])
    }

    pub fn get_mut(&mut self, coord: Coord) -> &mut T {
        let index = self.index(coord);
        self.cells.get_mut(index).unwrap()
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        let index = self.index(coord);
        self.cells[index] = value;
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Coord, &T)> + 'a> {
        Box::new(
            self.cells
                .iter()
                .enumerate()
                .map(move |(index, cell)| (self.coord(index), cell)),
        )
    }

    pub fn iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = (Coord, &'a mut T)> + 'a> {
        let size = self.size;

        Box::new(
            self.cells
                .iter_mut()
                .enumerate()
                .map(move |(index, cell)| (Self::_coord(size, index), cell)),
        )
    }

    pub fn neighbors_iter(&self, coord: Coord, with_diagonals: bool) -> NeighborIter<T> {
        assert!(coord.0 < self.size.0);
        assert!(coord.1 < self.size.1);

        NeighborIter::new(&self, coord, with_diagonals)
    }

    fn index(&self, coord: Coord) -> usize {
        assert!(coord.0 < self.size.0);
        assert!(coord.1 < self.size.1);

        coord.0 + coord.1 * self.size.0
    }

    fn coord(&self, index: usize) -> Coord {
        Self::_coord(self.size, index)
    }

    fn _coord(size: (usize, usize), index: usize) -> Coord {
        (index % size.0, index / size.0)
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;

        for (index, cell) in self.cells.iter().enumerate() {
            write!(f, "{:?} ", cell)?;

            if 0 == (index + 1) % self.size.0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cells == other.cells
    }
}

impl<T> FromIterator<(Coord, T)> for Grid<T> {
    fn from_iter<K: IntoIterator<Item = (Coord, T)>>(iter: K) -> Self {
        let mut cells = Vec::new();
        let mut size = (0, 0);

        for (coord, value) in iter.into_iter() {
            cells.push(value);

            if coord.0 > size.0 {
                size.0 = coord.0;
            }

            if coord.1 > size.1 {
                size.1 = coord.1;
            }
        }

        Grid::new((size.0 + 1, size.1 + 1), cells)
    }
}

struct NeighborCoordIter<'a, T> {
    grid: &'a Grid<T>,
    coord: Coord,
    visited: Vec<usize>,
    with_diagonals: bool,
}

impl<'a, T> NeighborCoordIter<'a, T> {
    pub fn new(grid: &'a Grid<T>, coord: Coord, with_diagonals: bool) -> Self {
        NeighborCoordIter {
            grid,
            coord,
            visited: vec![],
            with_diagonals,
        }
    }
}

impl<'a, T> Iterator for NeighborCoordIter<'a, T> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        // Left
        if !self.visited.contains(&0) && self.coord.0 != 0 {
            self.visited.push(0);
            Some((self.coord.0 - 1, self.coord.1))
        } else
        // Left-top
        if self.with_diagonals
            && !self.visited.contains(&1)
            && self.coord.0 != 0
            && self.coord.1 != 0
        {
            self.visited.push(1);
            Some((self.coord.0 - 1, self.coord.1 - 1))
        } else
        // Top
        if !self.visited.contains(&2) && self.coord.1 != 0 {
            self.visited.push(2);
            Some((self.coord.0, self.coord.1 - 1))
        } else
        // Top-right
        if self.with_diagonals
            && !self.visited.contains(&3)
            && self.coord.0 + 1 != self.grid.size.0
            && self.coord.1 != 0
        {
            self.visited.push(3);
            Some((self.coord.0 + 1, self.coord.1 - 1))
        } else
        // Right
        if !self.visited.contains(&4) && self.coord.0 + 1 != self.grid.size.0 {
            self.visited.push(4);
            Some((self.coord.0 + 1, self.coord.1))
        } else
        // Bottom-right
        if self.with_diagonals
            && !self.visited.contains(&5)
            && self.coord.0 + 1 != self.grid.size.0
            && self.coord.1 + 1 != self.grid.size.1
        {
            self.visited.push(5);
            Some((self.coord.0 + 1, self.coord.1 + 1))
        } else
        // Bottom
        if !self.visited.contains(&6) && self.coord.1 + 1 != self.grid.size.1 {
            self.visited.push(6);
            Some((self.coord.0, self.coord.1 + 1))
        } else
        // Bottom-left
        if self.with_diagonals
            && !self.visited.contains(&7)
            && self.coord.0 != 0
            && self.coord.1 + 1 != self.grid.size.1
        {
            self.visited.push(7);
            Some((self.coord.0 - 1, self.coord.1 + 1))
        } else {
            None
        }
    }
}

pub struct NeighborIter<'a, T> {
    grid: &'a Grid<T>,
    coord_iter: NeighborCoordIter<'a, T>,
}

impl<'a, T> NeighborIter<'a, T> {
    pub fn new(grid: &'a Grid<T>, coord: Coord, with_diagonals: bool) -> Self {
        NeighborIter {
            grid,
            coord_iter: NeighborCoordIter::new(grid, coord, with_diagonals),
        }
    }
}

impl<'a, T> Iterator for NeighborIter<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.coord_iter
            .next()
            .map(|coord| (coord, self.grid.get(coord)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid() -> Grid<char> {
        Grid::new(
            (4, 5),
            [
                'a', 'b', 'c', 'd', //
                'e', 'f', 'g', 'h', //
                'i', 'j', 'k', 'l', //
                'm', 'n', 'o', 'p', //
                'q', 'r', 's', 't', //
            ]
            .to_vec(),
        )
    }

    #[test]
    fn test_len() {
        let grid = create_grid();
        assert_eq!(grid.len(), 20);
    }

    #[test]
    fn test_get() {
        let grid = create_grid();
        assert_eq!(grid.get((0, 0)), &'a');
        assert_eq!(grid.get((2, 3)), &'o');
        assert_eq!(grid.get((3, 4)), &'t');
    }

    #[test]
    fn test_get_mut() {
        let mut grid = create_grid();

        assert_eq!(grid.get((2, 3)), &'o');

        let cell = grid.get_mut((2, 3));
        *cell = 'x';

        assert_eq!(grid.get((2, 3)), &'x');
    }

    #[test]
    fn test_set() {
        let mut grid = create_grid();
        assert_eq!(grid.get((2, 3)), &'o');

        grid.set((2, 3), 'x');
        assert_eq!(grid.get((2, 3)), &'x');
    }

    #[test]
    fn test_iter_cells() {
        let grid = create_grid();
        let mut iter = grid.iter();

        assert_eq!(iter.next(), Some(((0, 0), &'a')));
        assert_eq!(iter.next(), Some(((1, 0), &'b')));
        assert_eq!(iter.next(), Some(((2, 0), &'c')));
        assert_eq!(iter.next(), Some(((3, 0), &'d')));
        assert_eq!(iter.next(), Some(((0, 1), &'e')));
        assert_eq!(iter.next(), Some(((1, 1), &'f')));
        assert_eq!(iter.next(), Some(((2, 1), &'g')));
        assert_eq!(iter.next(), Some(((3, 1), &'h')));
        assert_eq!(iter.next(), Some(((0, 2), &'i')));
        assert_eq!(iter.next(), Some(((1, 2), &'j')));
        assert_eq!(iter.next(), Some(((2, 2), &'k')));
        assert_eq!(iter.next(), Some(((3, 2), &'l')));
        assert_eq!(iter.next(), Some(((0, 3), &'m')));
        assert_eq!(iter.next(), Some(((1, 3), &'n')));
        assert_eq!(iter.next(), Some(((2, 3), &'o')));
        assert_eq!(iter.next(), Some(((3, 3), &'p')));
        assert_eq!(iter.next(), Some(((0, 4), &'q')));
        assert_eq!(iter.next(), Some(((1, 4), &'r')));
        assert_eq!(iter.next(), Some(((2, 4), &'s')));
        assert_eq!(iter.next(), Some(((3, 4), &'t')));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut_cells() {
        let mut grid = create_grid();

        {
            let mut iter = grid.iter_mut();
            let next = iter.next();
            let (coord, value) = next.unwrap();
            *value = 'x';
            assert_eq!(coord, (0, 0));
        }

        assert_eq!(grid.get((0, 0)), &'x');
    }

    #[test]
    fn test_iter_cell_neighbors() {
        let grid = create_grid();

        assert_eq!(
            grid.neighbors_iter((0, 0), false)
                .collect::<Vec<(Coord, &char)>>(),
            [((1, 0), &'b'), ((0, 1), &'e')].to_vec()
        );
        assert_eq!(
            grid.neighbors_iter((2, 0), false)
                .collect::<Vec<(Coord, &char)>>(),
            [((1, 0), &'b'), ((3, 0), &'d'), ((2, 1), &'g')].to_vec()
        );
        assert_eq!(
            grid.neighbors_iter((1, 2), false)
                .collect::<Vec<(Coord, &char)>>(),
            [
                ((0, 2), &'i'),
                ((1, 1), &'f'),
                ((2, 2), &'k'),
                ((1, 3), &'n')
            ]
            .to_vec()
        );
        assert_eq!(
            grid.neighbors_iter((3, 4), false)
                .collect::<Vec<(Coord, &char)>>(),
            [((2, 4), &'s'), ((3, 3), &'p')].to_vec()
        );
    }

    #[test]
    fn test_get_the_cell_neighbors_with_diagonals() {
        let grid = create_grid();

        assert_eq!(
            grid.neighbors_iter((0, 0), true)
                .collect::<Vec<(Coord, &char)>>(),
            [((1, 0), &'b'), ((1, 1), &'f'), ((0, 1), &'e')].to_vec()
        );
        assert_eq!(
            grid.neighbors_iter((2, 0), true)
                .collect::<Vec<(Coord, &char)>>(),
            [
                ((1, 0), &'b'),
                ((3, 0), &'d'),
                ((3, 1), &'h'),
                ((2, 1), &'g'),
                ((1, 1), &'f')
            ]
            .to_vec()
        );
        assert_eq!(
            grid.neighbors_iter((1, 2), true)
                .collect::<Vec<(Coord, &char)>>(),
            [
                ((0, 2), &'i'),
                ((0, 1), &'e'),
                ((1, 1), &'f'),
                ((2, 1), &'g'),
                ((2, 2), &'k'),
                ((2, 3), &'o'),
                ((1, 3), &'n'),
                ((0, 3), &'m')
            ]
            .to_vec()
        );
        assert_eq!(
            grid.neighbors_iter((3, 4), true)
                .collect::<Vec<(Coord, &char)>>(),
            [((2, 4), &'s'), ((2, 3), &'o'), ((3, 3), &'p')].to_vec()
        );
    }
}
