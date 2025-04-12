
#[derive(Copy, Clone)]
pub struct Data {
    pub x1: i64,
    pub x2: i64,
    pub x3: i64
}

impl Data {
    // Constructor
    fn new(x1: i64, x2: i64, x3: i64) -> Self {
        Self {x1: x1, x2: x2, x3: x3}
    }
}

