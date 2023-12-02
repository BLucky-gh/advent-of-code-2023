pub struct LinearBuffer<T: Copy, const CAP: usize> {
    buff: [Option<T>; CAP],
    offset: usize,
}

impl<T: Copy, const CAP: usize> LinearBuffer<T, CAP> {
    /// Capacity is inferred from type param
    pub fn new() -> Self {
        Self {
            buff: [None; CAP],
            offset: 0,
        }
    }
    pub fn push(&mut self, val: T) -> Option<T> {
        let ret = self.buff[self.offset];
        self.buff[self.offset] = Some(val);
        self.offset = (self.offset + 1) % CAP;
        ret
    }
}
impl<const CAP: usize, T: Copy> LinearBuffer<T, CAP>
where
    Option<char>: From<Option<T>>,
{
    pub fn ends_with(&self, text: &str) -> bool {
        let len = text.len();
        if len > CAP {
            return false;
        };
        for (i, c) in text.char_indices() {
            let offset = self.buff.len() - len;
            if Some(c) != self[offset + i].into() {
                return false;
            }
        }
        true
    }
}

impl<T: Copy, const CAP: usize> Default for LinearBuffer<T, CAP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const CAP: usize> std::ops::Index<usize> for LinearBuffer<T, CAP> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let i = (self.offset + index) % CAP;
        &self.buff[i]
    }
}
