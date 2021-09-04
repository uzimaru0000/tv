pub struct Cell<T>
where
    T: std::fmt::Display,
{
    value: T,
}

impl<T> Cell<T>
where
    T: std::fmt::Display,
{
    pub fn new(v: T) -> Self {
        Self { value: v }
    }
}

impl<T> std::fmt::Display for Cell<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn create_cell() {
        let cell = Cell::new(String::from("Hello"));
        assert_eq!(cell.value, String::from("Hello"));
    }

    #[test]
    fn display_cell() {
        let cell = Cell::new(String::from("Hello"));
        assert_eq!("Hello", format!("{}", cell))
    }
}
