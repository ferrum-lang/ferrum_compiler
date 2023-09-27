use std::ops::Deref;

pub trait StringUtils {
    fn char_at(&self, index: usize) -> Option<char>;
}

impl<T: Deref<Target = str>> StringUtils for T {
    fn char_at(&self, index: usize) -> Option<char> {
        if index >= self.len() {
            return None;
        }

        return self.chars().nth(index);
    }
}
