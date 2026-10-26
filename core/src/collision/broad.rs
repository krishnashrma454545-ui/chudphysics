use crate::dynamics::Body;
use crate::math::Vec2;
use std::collections::HashMap;

pub struct BroadPhase {
    cell_size: f64,
    grid: HashMap<(i64, i64), Vec<usize>>,
}

impl BroadPhase {
    pub fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            grid: HashMap::new(),
        }
    }

    pub fn candidates(&mut self, bodies: &[Body]) -> Vec<(usize, usize)> {
        self.grid.clear();

        for (i, body) in bodies.iter().enumerate() {
            let (x0, y0) = self.cell_of(body.pos - Vec2::new(body.radius, body.radius));

            let (x1, y1) = self.cell_of(body.pos + Vec2::new(body.radius, body.radius));

            for cx in x0..=x1 {
                for cy in y0..=y1 {
                    self.grid.entry((cx, cy)).or_default().push(i);
                }
            }
        }

        let mut pairs = Vec::new();
        for cell_bodies in self.grid.values() {
            for a in 0..cell_bodies.len() {
                for b in (a + 1)..cell_bodies.len() {
                    let (i, j) = (cell_bodies[a], cell_bodies[b]);
                    pairs.push((i.min(j), i.max(j)));
                }
            }
        }
        pairs.sort_unstable();
        pairs.dedup();
        pairs
    }

    fn cell_of(&self, p: Vec2) -> (i64, i64) {
        (
            (p.x / self.cell_size).floor() as i64,
            (p.y / self.cell_size).floor() as i64,
        )
    }
}
