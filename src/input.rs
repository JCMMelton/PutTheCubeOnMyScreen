

pub struct Input {
    pub forward:    bool,
    pub backward:   bool,
    pub left:       bool,
    pub right:      bool,
    pub up:         bool,
    pub down:       bool,
    pub rotate:     bool
}

impl Input {
    pub fn new() -> Self {
        Input {
            forward:    false,
            backward:   false,
            left:       false,
            right:      false,
            up:         false,
            down:       false,
            rotate:     false,
        }
    }
}