#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut first_field = None;
        let mut second_field = None;

        if self.r == first {
            first_field = Some(&mut self.r);
        } else if self.r == second {
            second_field = Some(&mut self.r);
        }

        if self.g == first {
            first_field = Some(&mut self.g);
        } else if self.g == second {
            second_field = Some(&mut self.g);
        }

        if self.b == first {
            first_field = Some(&mut self.b);
        } else if self.b == second {
            second_field = Some(&mut self.b);
        }

        if self.a == first {
            first_field = Some(&mut self.a);
        } else if self.a == second {
            second_field = Some(&mut self.a);
        }

        if let (Some(field1), Some(field2)) = (first_field, second_field) {
            std::mem::swap(field1, field2);
        }

        self
    }
}