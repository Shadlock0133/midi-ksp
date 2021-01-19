pub struct Cache<T> {
    current: T,
    new: T,
}

impl<T> Cache<T> {
    pub fn new(value: T) -> Self
    where
        T: Clone,
    {
        Self {
            current: value.clone(),
            new: value,
        }
    }

    pub fn set(&mut self, value: T) {
        self.new = value;
    }

    pub fn update<F: FnMut(&mut T)>(&mut self, mut f: F) {
        f(&mut self.new)
    }

    pub fn has_changed(&self) -> bool
    where
        T: PartialEq,
    {
        self.current != self.new
    }

    pub fn get(&mut self) -> Option<T>
    where
        T: Clone + PartialEq,
    {
        if self.has_changed() {
            self.current = self.new.clone();
            Some(self.new.clone())
        } else {
            None
        }
    }
}
