use rand::{Rng, SeedableRng};

pub struct GameManager {
    width: usize,
    height: usize,
    seed: Option<u64>,
    cells: Vec<(bool, Cell)>,
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Empty,
    Bomb,
    NearBomb(usize),
}

impl GameManager {
    pub fn new(width: usize, height: usize, seed: Option<u64>) -> Self {
        GameManager {
            width,
            height,
            seed,
            cells: vec![],
        }
    }

    pub fn generate(&mut self) {
        let len = (self.height + 2) * (self.width + 2);
        let cells = vec![Cell::Empty; len as usize];

        let mut rng = if let Some(seed) = self.seed {
            rand::rngs::StdRng::seed_from_u64(seed)
        } else {
            rand::rngs::StdRng::from_entropy()
        };

        let cells: Vec<(bool, Cell)> = cells
            .iter()
            .enumerate()
            .map(|(index, _)| {
                // two side cols
                if index % (self.width + 2) == 0 || index % (self.width + 2) == self.width + 1 {
                    return Cell::Empty;
                }

                //top row
                if (0..=(self.width + 1)).contains(&index) {
                    return Cell::Empty;
                }

                // bot row
                if (((self.width + 2) * (self.height + 2) - (self.width + 2))
                    ..=((self.width + 2) * (self.height + 2) - 1))
                    .contains(&index)
                {
                    return Cell::Empty;
                }

                match rng.gen_bool(0.2) {
                    true => Cell::Bomb,
                    false => Cell::Empty,
                }
            })
            // all boxes are hidden from user
            .map(|el| match el {
                Cell::Empty => (true, el),
                _ => (false, el),
            })
            .collect();

        self.cells = cells;

        let cells: Vec<(bool, Cell)> = self
            .cells
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, el)| {
                if index % (self.width + 2) == 0 || index % (self.width + 2) == self.width + 1 {
                    return (true, Cell::Empty);
                }

                //top row
                if (0..=(self.width + 1)).contains(&index) {
                    return (true, Cell::Empty);
                }

                // bot row
                if (((self.width + 2) * (self.height + 2) - (self.width + 2))
                    ..=((self.width + 2) * (self.height + 2) - 1))
                    .contains(&index)
                {
                    return (true, Cell::Empty);
                }

                if el.1 == Cell::Bomb {
                    return el;
                }

                let count = self.cell_count_bomb_around(index);
                match count {
                    0 => el,
                    other => (false, Cell::NearBomb(other)),
                }
            })
            .collect();

        self.cells = cells;
    }

    fn cell_count_bomb_around(&self, index: usize) -> usize {
        let right = self.cells.get(index + 1).unwrap_or(&(true, Cell::Empty));
        let left = self.cells.get(index - 1).unwrap_or(&(true, Cell::Empty));
        let top = &self.cells[(index - (self.width + 3))..=(index - (self.width + 1))];
        let bot = &self.cells[(index + (self.width + 1))..=(index + (self.width + 3))];

        let mut count = 0;

        if right.1 == Cell::Bomb {
            count += 1;
        }

        if left.1 == Cell::Bomb {
            count += 1;
        }

        count += top
            .iter()
            .map(|(_, el)| el)
            .filter(|el| *el == &Cell::Bomb)
            .count();

        count += bot
            .iter()
            .map(|(_, el)| el)
            .filter(|el| *el == &Cell::Bomb)
            .count();

        count
    }

    pub fn show(&self) {
        for chunk in self.cells.chunks(self.width + 2) {
            for c in chunk {
                match c {
                    (false, _) => print!("   *   "),
                    (_, Cell::Bomb) => print!(" {:#?}  ", c.1),
                    (_, Cell::Empty) => print!(" {:#?} ", c.1),
                    (_, Cell::NearBomb(count)) => print!("   {}   ", count),
                };
            }

            println!();
        }
    }
}
