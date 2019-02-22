use std::ops;

fn main() {
    
}

fn noise(coord: Coord<f32>) -> f32 {
    let grid = Coord::new(coord[0] as i32, coord[1] as i32);
    let mut local = Coord::new(1, 1);
    let mut cur = (Coord::zero(), 2.);
    let mut allowed = vec![Neighborhood::full(); 2];
    let mut all = true;
    
    for i in 0..2 {
        if all {
            let new_cur = closest_sample(coord, grid + local);
            if new_cur.1 < cur.1 {
                cur = new_cur;
            } else {
                allowed[i].set(local[i], false);
                local = Coord::new(1, 1);
            }
        } else {
            all = true;
        }
        let (_, dist) = cur;
        if dist < coord[i] {
            allowed[i].set(local[i] + 1, false);
            local[i] -= 1;
        } else if dist < 1. - coord[i] {
            allowed[i].set(local[i], false);
            local[i] += 1;
        } else {
            allowed[i].set(local[i] + 1, false);
            allowed[i].set(local[i], false);
            all = false;
        }
    }
    
    for (i, x) in allowed[0].0.iter().enumerate() {
        for (j, y) in allowed[1].0.iter().enumerate() {
            if *x && *y {
                let local = Coord::new(i as i32 * 2 - 1, j as i32 * 2 - 1);
                let new_cur = closest_sample(coord, grid + local);
                if new_cur.1 < cur.1 {
                    cur = new_cur;
                }
            }    
        }
    }
    
    println!("{:?}", cur);
    return 0.;
}

fn closest_sample(coord: Coord<f32>, grid: Coord<i32>) -> (Coord<f32>, f32) {
    unimplemented!()
}

#[derive(Debug, Clone, Copy)]
struct Coord<F: Copy>([F; 2]);

impl<F: Copy> Coord<F> {
    fn new(x: F, y: F) -> Self {
        Coord([x, y])
    }
}

impl Coord<f32> {
    fn zero() -> Self {
        Coord([0.; 2])
    }
}

impl<F: Copy> ops::Index<usize> for Coord<F> {
    type Output = F;
    fn index(&self, index: usize) -> &F {
        &self.0[index]
    }
}

impl<F: Copy> ops::IndexMut<usize> for Coord<F> {
    fn index_mut(&mut self, index: usize) -> &mut F {
        &mut self.0[index]
    }
}


impl<F: ops::AddAssign + Copy> ops::Add for Coord<F> {
    type Output = Self;
    fn add(mut self, lhs: Self) -> Self::Output {
        self.0[0] += lhs.0[0];
        self.0[1] += lhs.0[1];
        self
    }
}

#[derive(Clone)]
struct Neighborhood([bool; 2]);

impl Neighborhood {
    fn full() -> Self {
        Neighborhood([true; 2])
    }
    fn set(&mut self, index: i32, value: bool) {
        if 0 <= index && index < 2 {
            self.0[index as usize] = value;
        }
    }
}