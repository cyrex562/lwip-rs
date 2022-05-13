#[derive(Default,Debug,Clone)]
pub struct Queue<T> {
    elements: Vec<T>
}

impl Queue<T> {
    pub fn new() -> Self {
        Self {
            ..Default()
        }
    }

    pub fn push(&mut self, t: &mut T) {
        self.elements.insert(0, t)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn clear(&mut self) {
        self.clear()
    }
}
