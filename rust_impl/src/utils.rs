pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}
pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

pub trait StringUtils {
    fn sub_str(&self, start: usize, len: usize) -> Self;
}
impl StringUtils for String {
    fn sub_str(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}
