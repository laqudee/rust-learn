pub trait Draw {
    fn draw(&self);
}

// Box<dyn Draw> 为一个trait对象：它是Box中任何实现了Draw trait 的类型的替身
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self)
    }
}
