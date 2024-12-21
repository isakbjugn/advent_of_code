
pub trait Expandable<T> {
    fn with(&self, new_element: T) -> Self;
}

impl Expandable<u32> for Vec<u32> {
    fn with(&self, new_element: u32) -> Self {
        self.iter().chain([new_element].iter()).cloned().collect()
    }
}

impl Expandable<char> for Vec<char> {
    fn with(&self, new_element: char) -> Self {
        self.iter().chain([new_element].iter()).cloned().collect()
    }
}

impl <'a> Expandable<&'a str> for Vec<&'a str> {
    fn with(&self, new_element: &'a str) -> Self {
        self.iter().chain([new_element].iter()).cloned().collect()
    }
}