#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 1 {
            return None;
        }
        let current = self.v;
        self.v = if current % 2 == 0 {
            current / 2
        } else {
            current * 3 + 1
        };
        Some(Collatz {v: current})
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0;
    }
    let mut collatz = Collatz::new(n);
    let mut steps = 0;
    while let Some(_) = collatz.next() {
        steps += 1;
    }
    steps
}