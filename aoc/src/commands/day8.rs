use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use std::collections::HashSet;

#[derive(Parser, Debug)]
pub struct Day8 {
    #[clap(long, short)]
    input: PathBuf,
}

pub fn count_visible_trees(grid: &Vec<u8>, nrows: usize, ncolumns: usize) -> usize {
    let mut invisibles: HashSet<usize> = HashSet::new();
    let mut invisible_other: HashSet<usize> = HashSet::new();

    let start: usize = ncolumns + 1;
    let stop: usize = (nrows * (ncolumns - 1)) - 1;
    let mut max_lhs: u8 = 0;
    let mut max_rhs: u8 = 0;
    for i in start..stop {
        if (i % nrows) == (nrows - 1) {
            max_lhs = 0;
            continue;
        }
        if (i % nrows) == 0 {
            continue;
        }
        max_lhs = if max_lhs < grid[i - 1] { grid[i - 1] } else { max_lhs };
        if grid[i] <= max_lhs {
            invisibles.insert(i);
        }
    }

    for i in 1..(stop - (start - 1)) {
        if ((stop - i) % nrows) == 0 {
            max_rhs = 0;
            continue;
        }
        if ((stop - i) % nrows) == (nrows - 1) {
            continue;
        }
        max_rhs = if max_rhs < grid[stop - i + 1] { grid[stop - i + 1] } else { max_rhs };
        let s: usize = stop - i;
        if grid[stop - i] <= max_rhs && invisibles.contains(&s) {
            invisible_other.insert(s);
        }
    }
    invisibles.clear();

    let mut c: usize = 1;
    let mut r: usize = 1;
    let mut max_top: u8 = 0;
    loop {
        if r == (nrows - 1) {
            max_top = 0;
            c += 1;
            r = 1;
            continue;
        }
        if c == (ncolumns - 1) {
            break;
        }
        let index: usize = r * ncolumns + c;
        let comparator: usize = index - ncolumns;
        max_top = if max_top < grid[comparator] { grid[comparator] } else { max_top };
        if grid[index] <= max_top && invisible_other.contains(&index) {
            invisibles.insert(index);
        }
        r += 1;
    }
    invisible_other.clear();

    c = 1;
    r = nrows - 2;
    let mut max_bottom: u8 = 0;
    loop {
        if r == 0 {
            max_bottom = 0;
            c += 1;
            r = nrows - 2;
            continue;
        }
        if c == (ncolumns - 1) {
            break;
        }
        let index: usize = r * ncolumns + c;
        let comparator: usize = index + ncolumns;
        max_bottom = if max_bottom < grid[comparator] { grid[comparator] } else { max_bottom };
        if grid[index] <= max_bottom && invisibles.contains(&index) {
            invisible_other.insert(index);
        }
        r -= 1;
    }

    grid.len() - invisible_other.len()
}

pub fn n_to_left(grid: &Vec<u8>, nrows: usize, ncols: usize) -> Vec<usize> {
    let mut lhs: Vec<usize> = vec![0; nrows * ncols];
    for i in 0..(nrows * ncols) {
        if (i % ncols) == 0 {
            continue;
        }
        if grid[i] > grid[i - 1] {
            let mut sum: usize = 0;
            for j in 1..(i % ncols + 1) {
                sum += 1;
                if grid[i] <= grid[i - j] {
                    break;
                }
            }
            lhs[i] = sum;
        } else {
            lhs[i] = 1;
        }
    }
    lhs
}

pub fn transpose(grid: &Vec<u8>, nrows: usize, ncols: usize) -> Vec<u8> {
    let mut tgrid: Vec<u8> = vec![0; grid.len()];
    for i in 0..ncols {
        for j in 0..nrows {
            if i != j {
                (tgrid[j * nrows + i], tgrid[i * nrows + j]) =
                    (grid[i * nrows + j], grid[j * nrows + i])
            } else {
                tgrid[j * nrows + i] = grid[j * nrows + i];
            }
        }
    }
    tgrid
}

pub fn mirror(grid: &Vec<u8>, nrows: usize, ncols: usize) -> Vec<u8> {
    let mut tgrid: Vec<u8> = vec![0; grid.len()];
    for i in 0..nrows {
        for j in 0..(ncols / 2) {
            let k = ncols - j - 1;
            (tgrid[i * nrows + j], tgrid[i * nrows + k]) =
                (grid[i * nrows + k], grid[i * nrows + j])
        }
        if ncols % 2 == 1 {
            let j = ncols % 2 + 1;
            for i in 0..nrows {
                tgrid[i * nrows + j] = grid[i * nrows + j];
            }
        }
    }
    tgrid
}

pub fn transpose_usize(grid: &Vec<usize>, nrows: usize, ncols: usize) -> Vec<usize> {
    let mut tgrid: Vec<usize> = vec![0; grid.len()];
    for i in 0..ncols {
        for j in 0..nrows {
            if i != j {
                (tgrid[j * nrows + i], tgrid[i * nrows + j]) =
                    (grid[i * nrows + j], grid[j * nrows + i])
            } else {
                tgrid[j * nrows + i] = grid[j * nrows + i];
            }
        }
    }
    tgrid
}

pub fn mirror_usize(grid: &Vec<usize>, nrows: usize, ncols: usize) -> Vec<usize> {
    let mut tgrid: Vec<usize> = vec![0; grid.len()];
    for i in 0..nrows {
        for j in 0..(ncols / 2) {
            let k = ncols - j - 1;
            (tgrid[i * nrows + j], tgrid[i * nrows + k]) =
                (grid[i * nrows + k], grid[i * nrows + j])
        }
        if ncols % 2 == 1 {
            let j = ncols % 2 + 1;
            for i in 0..nrows {
                tgrid[i * nrows + j] = grid[i * nrows + j];
            }
        }
    }
    tgrid
}

pub fn display(grid: &Vec<u8>, nrows: usize, ncols: usize) {
    for j in 0..nrows {
        for i in 0..ncols {
            print!("{:01} ", grid[i + j * nrows]);
        }
        println!("");
    }
}

pub fn display_usize(grid: &Vec<usize>, nrows: usize, ncols: usize) {
    for j in 0..ncols {
        for i in 0..nrows {
            print!("{:02} ", grid[i + j * nrows]);
        }
        println!("");
    }
}

pub fn display_u32(grid: &Vec<u32>, nrows: usize, ncols: usize) {
    for j in 0..ncols {
        for i in 0..nrows {
            print!("{:02} ", grid[i + j * nrows]);
        }
        println!("");
    }
}

pub fn tree_score(w: &Vec<usize>, x: &Vec<usize>, y: &Vec<usize>, z: &Vec<usize>) -> Vec<u32> {
    let mut a: Vec<u32> = vec![0; w.len()];
    for i in 0..(w.len()) {
        let mut val1 = w[i] as u32;
        let mut val2 = x[i] as u32;
        let mut val3 = y[i] as u32;
        let mut val4 = z[i] as u32;
        a[i] = val1 * val2 * val3 * val4;
    }
    a
}

impl CommandImpl for Day8 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;

        let ncolumns: usize = lines[0].len();
        let mut nrows: usize = 0;
        let mut grid: Vec<u8> = vec![];

        for line in lines {
            //println!("{line}");
            let chars: Vec<char> = line.chars().collect();
            for c in chars {
                grid.push(c as u8 - 48);
            }
            nrows += 1;
        }

        let grid_mirror = mirror(&grid, nrows, ncolumns);
        let visible_tree_count = count_visible_trees(&grid, nrows, ncolumns);
        let lhs = n_to_left(&grid, nrows, ncolumns);
        let rhs_m = n_to_left(&grid_mirror, nrows, ncolumns);
        let rhs = mirror_usize(&rhs_m, nrows, ncolumns);

        let mut tgrid: Vec<u8> = transpose(&grid, nrows, ncolumns);
        let mut tgrid_m = mirror(&tgrid, nrows, ncolumns);
        let above_t = n_to_left(&tgrid, nrows, ncolumns);
        let below_m = n_to_left(&tgrid_m, nrows, ncolumns);
        let above = transpose_usize(&above_t, nrows, ncolumns);
        let below_mm = mirror_usize(&below_m, nrows, ncolumns);
        let below = transpose_usize(&below_mm, nrows, ncolumns);

        let the_tree_score: Vec<u32> = tree_score(&lhs, &rhs, &above, &below);
        println!("scores:");
        //display_u32(&the_tree_score, nrows, ncolumns);
        let mut max: u32 = 0;
        for x in the_tree_score.iter() {
            if max < *x {
                max = *x;
            }
        }
        println!("max score: {max}");

        println!("visible tree count {visible_tree_count}");
        Ok(())
    }
}
