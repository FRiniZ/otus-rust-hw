pub trait Progress {
    fn build(&self, pos: f32) -> String;
    fn draw(&self, pos: f32);
}

pub struct BaseProgress {}

impl Progress for BaseProgress {
    fn build(&self, pos: f32) -> String {
        format!("\r{}%", pos)
    }

    fn draw(&self, pos: f32) {
        print!("{}", self.build(pos));
    }
}

pub struct BracketsDecorator<T: Progress> {
    decorated: T,
}

impl<T: Progress> BracketsDecorator<T> {
    pub fn new(decorated: T) -> Self {
        Self { decorated }
    }
}

impl<T: Progress> Progress for BracketsDecorator<T> {
    fn build(&self, pos: f32) -> String {
        let mut s = self.decorated.build(pos);
        s.insert(1, '[');
        s.push(']');
        s
    }
    fn draw(&self, pos: f32) {
        print!("{}", self.build(pos));
    }
}

pub struct ProgressBarDecorator<T: Progress> {
    decorated: T,
}

impl<T: Progress> ProgressBarDecorator<T> {
    pub fn new(decorated: T) -> Self {
        Self { decorated }
    }
}

impl<T: Progress> Progress for ProgressBarDecorator<T> {
    fn build(&self, pos: f32) -> String {
        let ret = termsize::get();
        if ret.is_none() {
            return "Can't get rows,cols of terminal".to_string();
        }

        let mut s = self.decorated.build(pos);
        let size = ret.unwrap();
        let mut cols = (size.cols as f32 / 100.0 * pos) as usize;
        if cols < s.len() {
            cols = 1;
        } else {
            cols -= s.len();
        }

        let line = format!("{:=>cols$}", ">");
        s.insert_str(1, &line);
        s
    }
    fn draw(&self, pos: f32) {
        print!("{}", self.build(pos));
    }
}
