use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy)]
pub enum Align {
    None,
    Left,
    Center,
    Right,
}

impl Align {
    pub fn new(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "l" | "left" => Self::Left,
            "c" | "center" => Self::Center,
            "r" | "right" => Self::Right,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell<T>
where
    T: std::fmt::Display,
{
    value: T,
    width: usize,
    align: Align,
}

impl<T> Cell<T>
where
    T: std::fmt::Display + Clone,
{
    pub fn new(v: T) -> Self {
        Self {
            value: v.clone(),
            width: format!("{}", v.clone()).width(),
            align: Align::None,
        }
    }

    pub fn set_width(&mut self, w: usize) -> &mut Self {
        self.width = w;
        self
    }

    pub fn set_align(&mut self, a: Align) -> &mut Self {
        self.align = a;
        self
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> std::fmt::Display for Cell<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.align {
            Align::Right => {
                format!(
                    "{}{}",
                    " ".repeat(self.width - format!("{}", self.value).width()),
                    self.value
                )
            }
            Align::None | Align::Left => format!(
                "{}{}",
                self.value,
                " ".repeat(self.width - format!("{}", self.value).width())
            ),
            Align::Center => {
                let pad = (self.width - format!("{}", self.value).width()) as f32 / 2.0;
                let left_pad = pad.ceil() as usize;
                let right_pad = pad.floor() as usize;
                format!(
                    "{}{}{}",
                    " ".repeat(left_pad),
                    self.value,
                    " ".repeat(right_pad)
                )
            }
        };

        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::{Align, Cell};

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

    #[test]
    fn display_cell_with_width() {
        let mut cell = Cell::new(String::from("Hello"));
        cell.set_width(10);
        assert_eq!("Hello     ", format!("{}", cell))
    }

    #[test]
    fn display_cell_with_align() {
        struct TestCase<'a> {
            value: &'a str,
            width: usize,
            align: Align,
            expect: &'a str,
        }

        let data_set = vec![
            TestCase {
                value: "Hello",
                width: 10,
                align: Align::None,
                expect: "Hello     ",
            },
            TestCase {
                value: "Hello",
                width: 10,
                align: Align::Left,
                expect: "Hello     ",
            },
            TestCase {
                value: "Hello",
                width: 10,
                align: Align::Center,
                expect: "   Hello  ",
            },
            TestCase {
                value: "Hello",
                width: 10,
                align: Align::Right,
                expect: "     Hello",
            },
        ];

        data_set.iter().for_each(|case| {
            let mut cell = Cell::new(case.value);
            cell.set_width(case.width).set_align(case.align);
            assert_eq!(case.expect, format!("{}", cell));
        });
    }
}
