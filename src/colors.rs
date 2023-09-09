pub enum Color {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Gray,
    White,
    Gold,
    Silver,
}

impl Color {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Black => "чёрный",
            Self::Brown => "коричневый",
            Self::Red => "красный",
            Self::Orange => "оранжевый",
            Self::Yellow => "жёлтый",
            Self::Green => "зелёный",
            Self::Blue => "синий",
            Self::Purple => "фиолетовый",
            Self::Gray => "серый",
            Self::White => "белый",
            Self::Gold => "золотой",
            Self::Silver => "серебряный",
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::Black | Self::Gold | Self::Silver => 0,
            Self::Brown => 1,
            Self::Red => 2,
            Self::Orange => 3,
            Self::Yellow => 4,
            Self::Green => 5,
            Self::Blue => 6,
            Self::Purple => 7,
            Self::Gray => 8,
            Self::White => 9,
        }
    }

    pub fn to_multiplier(&self) -> f32 {
        match self {
            Self::Black => 1.,
            Self::Brown => 10.,
            Self::Red => 100.,
            Self::Orange => 1000.,
            Self::Yellow => 10_000.,
            Self::Green => 100_000.,
            Self::Blue => 1_000_000.,
            Self::Purple => 10_000_000.,
            Self::Gray => 100_000_000.,
            Self::White => 1_000_000_000.,
            Self::Gold => 0.1,
            Self::Silver => 0.01,
        }
    }

    pub fn to_deviation(&self) -> Option<f32> {
        match self {
            Self::Brown => Some(1.),
            Self::Red => Some(2.),
            Self::Gold => Some(5.),
            Self::Silver => Some(10.),
            Self::Gray => Some(0.05),
            Self::Purple => Some(0.1),
            Self::Blue => Some(0.25),
            Self::Green => Some(0.5),
            _ => None,
        }
    }
}
