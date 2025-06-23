use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        let mut result = Vec::new();
        let mappings = [
            (1000, M, None),
            (900, C, Some(M)),
            (500, D, None),
            (400, C, Some(D)),
            (100, C, None),
            (90, X, Some(C)),
            (50, L, None),
            (40, X, Some(L)),
            (10, X, None),
            (9, I, Some(X)),
            (5, V, None),
            (4, I, Some(V)),
            (1, I, None),
        ];

        for &(value, digit, subtractive) in mappings.iter() {
            while num >= value {
                num -= value;
                result.push(digit);
                if let Some(sub_digit) = subtractive {
                    result.push(sub_digit);
                }
            }
        }

        RomanNumber(result)
    }
}