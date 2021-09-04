use self::cell::Cell;

mod cell;

#[derive(Debug, Clone, Copy)]
pub enum Style {
    Markdown,
}

type Row<T> = Vec<Cell<T>>;

pub struct Table<T>
where
    T: std::fmt::Display,
{
    cols: Vec<Row<T>>,
		header: Option<Row<T>>,
    style: Style,
}

impl<T> Table<T>
where
    T: std::fmt::Display,
{
    pub fn new() -> Self {
        Self {
            cols: Vec::new(),
						header: None,
            style: Style::Markdown,
        }
    }

    pub fn push_row(&mut self, row: Row<T>) {
        self.cols.push(row);
    }

		pub fn set_header(&mut self, header: Option<Row<T>>) -> &mut Self {
			self.header = header;
			self
		}
}

impl<T> std::fmt::Display for Table<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.cols.iter().map(|row| {
            row.iter()
                .map(|cell| format!("{}", cell))
                .collect::<Vec<_>>()
                .join("|")
        }).map(|x| format!("|{}|", x))
				.collect::<Vec<_>>()
				.join("\n");

				write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use crate::table::{cell::Cell, Table};

    #[test]
    fn create_table() {
        let mut table = Table::new();
        table.push_row(vec![Cell::new("0"), Cell::new("1"), Cell::new("2")]);

        assert_eq!(table.cols.len(), 1);
        let row = table.cols.get(0).unwrap();
        assert_eq!(row.len(), 3);
    }

    #[test]
    fn display_table() {
        let mut table = Table::new();
        table.push_row(vec![Cell::new("0"), Cell::new("1"), Cell::new("2")]);

        let expected = "|0|1|2|";
        let actual = format!("{}", table);
        assert_eq!(expected, actual);
    }

		#[test]
    fn display_table_multiline() {
        let mut table = Table::new();
        table.push_row(vec![Cell::new("00"), Cell::new("01"), Cell::new("02")]);
        table.push_row(vec![Cell::new("10"), Cell::new("11"), Cell::new("12")]);

        let expected = "|00|01|02|\n|10|11|12|";
        let actual = format!("{}", table);
        assert_eq!(expected, actual);
    }
}
