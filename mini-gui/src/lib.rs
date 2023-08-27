pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 實際畫出按鈕的程式碼
    }
}

pub struct Screen {
    // trait objects
    // Box 內的任何型別都得有實作 Draw 特徵
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // run 不需要知道每個元件的實際型別為何
    pub fn run(&self) {
        for component in self.components.iter() {
            // polymorphism: draw can be called on any type that implements the Draw trait
            component.draw();
        }
    }
}

