pub enum Color {
    /// Чёрный цвет
    Black,

    /// Коричневый цвет
    Brown,

    /// Красный цвет
    Red,

    /// Оранжевый цвет
    Orange,

    /// Жёлтый цвет
    Yellow,

    /// Зелёный цвет
    Green,

    /// Синий цвет
    Blue,

    /// Фиолетовый цвет
    Purple,

    /// Серый цвет
    Gray,

    /// Белый цвет
    White,
}

pub enum Multiplier {
    /// Чёрный цвет
    Black,

    /// Коричневый цвет
    Brown,

    /// Красный цвет
    Red,

    /// Оранжевый цвет
    Orange,

    /// Жёлтый цвет
    Yellow,

    /// Зелёный цвет
    Green,

    /// Синий цвет
    Blue,

    /// Золото
    Gold,

    /// Серебро (серый)
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
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Self::Black => 0,
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
}

impl Multiplier {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Black => "чёрный",
            Self::Brown => "коричневый",
            Self::Red => "красный",
            Self::Orange => "оранжевый",
            Self::Yellow => "жёлтый",
            Self::Green => "зелёный",
            Self::Blue => "синий",
            Self::Gold => "золотой",
            Self::Silver => "серебряный",
        }
    }

    pub fn to_multiplier(&self) -> f32 {
        match self {
            Self::Black => 1.,
            Self::Brown => 10.,
            Self::Red => 100.,
            Self::Orange => 1_000.,
            Self::Yellow => 10_000.,
            Self::Green => 100_000.,
            Self::Blue => 1_000_000.,
            Self::Gold => 0.1,
            Self::Silver => 0.01,
        }
    }

    pub fn to_label(&self) -> &str {
        match self {
            Self::Black | Self::Brown | Self::Red | Self::Gold | Self::Silver => "Ом",
            Self::Orange | Self::Yellow | Self::Green => "кОм",
            _ => "МОм",
        }
    }
}
