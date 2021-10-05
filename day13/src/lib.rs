pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub componets: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        /*
        for c in self.componets.iter() {
            c.draw();
        }
        */
        self.componets.iter().map(|component| component.draw());
    }
}

// Compare with the below; where homogeneous collections are required
pub struct Screen2<T: Draw> {
    pub components: Vec<T>
}

impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        
    }
}
