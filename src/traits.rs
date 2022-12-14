pub trait RemoveLast {
    fn remove_last(&self) -> Self;
}

impl RemoveLast for String {
    fn remove_last(&self) -> Self {
        if self.len() > 1 {
            return self[0..self.len() - 2].to_string();
        }
        self.clone()
    }
}
pub trait StringtToStr {
    fn to_str(&self) -> &str;
}
impl StringtToStr for String {
    fn to_str(&self) -> &str {
        &self[..]
    }
}
pub trait AddDpPlace {
    fn add_dp(&self) -> Self;
}
impl AddDpPlace for String {
    fn add_dp(&self) -> Self {
        Self::from_utf8([self.clone().into_bytes(), vec![46, 48, 48]].concat())
            .expect("Unable to convert from utf8 to string")
    }
}
