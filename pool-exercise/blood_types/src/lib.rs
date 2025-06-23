use std::cmp::{Ord, Ordering};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("Blood type string too short".to_string());
        }

        let antigen = Antigen::from_str(&s[..s.len()-1])?;
        let rh_factor = RhFactor::from_str(&s[s.len()-1..])?;

        Ok(BloodType { antigen, rh_factor })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_ok = match (self.antigen.clone(), other.antigen.clone()) {
            (Antigen::AB, _) => true,
            (Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
            (Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
            (Antigen::O, Antigen::O) => true,
            _ => false,
        };

        let rh_ok = match (self.rh_factor.clone(), other.rh_factor.clone()) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };

        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut donors = Vec::new();
        let antigens = match self.antigen {
            Antigen::AB => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
            Antigen::A => vec![Antigen::A, Antigen::O],
            Antigen::B => vec![Antigen::B, Antigen::O],
            Antigen::O => vec![Antigen::O],
        };

        for antigen in antigens {
            for rh in [RhFactor::Negative, RhFactor::Positive].iter() {
                let donor = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if self.can_receive_from(&donor) {
                    donors.push(donor);
                }
            }
        }
        donors
    }

    pub fn recipients(&self) -> Vec<Self> {
        let mut recipients = Vec::new();
        let antigens = match self.antigen {
            Antigen::AB => vec![Antigen::AB],
            Antigen::A => vec![Antigen::A, Antigen::AB],
            Antigen::B => vec![Antigen::B, Antigen::AB],
            Antigen::O => vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O],
        };

        for antigen in antigens {
            for rh in [RhFactor::Negative, RhFactor::Positive].iter() {
                let recipient = BloodType {
                    antigen: antigen.clone(),
                    rh_factor: rh.clone(),
                };
                if recipient.can_receive_from(self) {
                    recipients.push(recipient);
                }
            }
        }
        recipients
    }
}