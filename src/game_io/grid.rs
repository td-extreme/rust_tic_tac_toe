pub struct GridRow<T> {
    row: Vec<T>,
}

impl <T: Clone + PartialEq> GridRow<T> {
    pub fn new(size: usize, fill: T) -> GridRow<T> {
        let row = vec!(fill; size);
        GridRow {
            row: row,
        }
    }

    pub fn len(&self) -> usize {
       self.row.len()
    }

    pub fn get(&self, index: usize) -> T {
       self.row[index].clone()
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.row[index] = value;
    }
}

pub struct Grid<T> {
    fill_value: T,
    grid: Vec<GridRow<T>>,
}

impl <T: Clone + PartialEq> Grid<T> {
    pub fn new(rows: usize, cols: usize, fill: T) -> Grid<T> {
        let blank = fill.clone();
        let mut grid = Vec::new();
        for index in 0..rows {
            let row = GridRow::new(cols, fill.clone());
            grid.push(row);
        }
        Grid {
            fill_value: blank,
            grid: grid,
        }
    }

    pub fn fill_value(&self) -> T {
        self.fill_value.clone()
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.grid[row].set(col, value);
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.grid[row].get(col)
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }
 }
