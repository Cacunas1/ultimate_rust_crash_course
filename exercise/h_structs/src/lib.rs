pub struct Polygon {
    pub name: String,
    sides: u32,
    pub visible: bool,
}

impl Polygon {
    pub fn new(name: String) -> Self {
        Polygon {
            name: name,
            sides: 3,
            visible: true,
        }
    }
    pub fn shape(&self) -> String {
        match self.sides {
            3 => "triangle".to_string(),
            4 => "square".to_string(),
            5 => "pentagon".to_string(),
            _ => "polygon".to_string(),
        }
    }
    pub fn increment_sides(&mut self) {
        self.sides += 1;
    }
    pub fn sides(&self) -> u32 {
        self.sides
    }
}
