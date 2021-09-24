use self::cell::{Align, Cell};
use self::style::{Frame, Style};

pub mod cell;
pub mod style;

type Row<T> = Vec<Cell<T>>;

fn display_row<T>(row: &Row<T>, width_list: &Vec<usize>, align: Align, style: Style) -> String
where
    T: std::fmt::Display + Clone,
{
    let frame: Frame = style.into();

    row.iter()
        .zip(width_list.clone())
        .map(|(cell, width)| {
            let mut cell = cell.clone();
            format!("{}", cell.set_width(width).set_align(align))
        })
        .collect::<Vec<_>>()
        .join(&frame.separator)
}

pub struct Table<T>
where
    T: std::fmt::Display + Clone,
{
    cols: Vec<Row<T>>,
    header: Option<Row<T>>,
    style: Style,
    align: Align,
    no_headers: bool,
    // cache
    row_len: usize,
}

impl<T> Table<T>
where
    T: std::fmt::Display + Clone,
{
    pub fn new() -> Self {
        Self {
            cols: Vec::new(),
            header: None,
            style: Style::Markdown,
            align: Align::None,
            no_headers: false,
            row_len: 0,
        }
    }

    pub fn push_row(&mut self, row: Row<T>) {
        self.cols.push(row.clone());
        self.row_len = self.row_len.max(row.len());
    }

    pub fn set_header(&mut self, header: Option<Row<T>>) -> &mut Self {
        self.header = header;
        self
    }

    pub fn set_align(&mut self, align: Align) -> &mut Self {
        self.align = align;
        self
    }

    pub fn set_style(&mut self, style: Style) -> &mut Self {
        self.style = style;
        self
    }

    pub fn set_no_headers(&mut self, no_headers: bool) -> &mut Self {
        self.no_headers = no_headers;
        self
    }

    fn cell_width_list(&self) -> Vec<usize> {
        let widths = self
            .cols
            .iter()
            .map(|xs| xs.iter().map(|x| x.width()).collect::<Vec<_>>());

        let widths = (0..self.row_len).map(|idx| {
            widths
                .clone()
                .map(move |x| x.get(idx).map(Clone::clone).unwrap_or_default())
                .max()
                .unwrap_or_default()
        });

        widths
            .enumerate()
            .map(|(idx, w)| {
                self.header
                    .clone()
                    .and_then(|x| x.get(idx).map(|x| x.width()).map(|x| x.max(w)))
                    .unwrap_or(w)
            })
            .map(|x| {
                if let Style::Markdown = self.style {
                    match self.align {
                        Align::Center => x.max(3),
                        Align::Left | Align::Right => x.max(2),
                        Align::None => x,
                    }
                } else {
                    x
                }
            })
            .collect()
    }
}

impl<T> std::fmt::Display for Table<T>
where
    T: std::fmt::Display + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width_list = self.cell_width_list();
        let frame: Frame = self.style.into();

        if frame.has_cover {
            write!(
                f,
                "{}{}{}\n",
                frame.top_left,
                width_list
                    .clone()
                    .into_iter()
                    .map(|x| frame.border.repeat(x))
                    .collect::<Vec<_>>()
                    .join(&frame.top),
                frame.top_right
            )?;
        }

        if !self.no_headers {
            if let Some(header) = &self.header {
                write!(
                    f,
                    "{}{}{}\n",
                    frame.separator,
                    display_row(header, &width_list, self.align, self.style),
                    frame.separator,
                )?;
                let border = width_list
                    .clone()
                    .into_iter()
                    .map(|x| {
                        if let Style::Markdown = self.style {
                            frame.align_border(&self.align, x)
                        } else {
                            frame.border.repeat(x)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(&frame.center);
                write!(f, "{}{}{}\n", frame.left, border, frame.right)?;
            }
        }

        let table = self
            .cols
            .iter()
            .map(|row| display_row(row, &width_list, self.align, self.style))
            .map(|x| format!("{}{}{}", frame.separator, x, frame.separator))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", table)?;

        if frame.has_cover {
            write!(
                f,
                "\n{}{}{}",
                frame.bottom_left,
                width_list
                    .clone()
                    .into_iter()
                    .map(|x| frame.border.repeat(x))
                    .collect::<Vec<_>>()
                    .join(&frame.bottom),
                frame.bottom_right
            )?;
        }

        Ok(())
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

    #[test]
    fn cell_width() {
        let mut table = Table::new();
        table.push_row(vec![Cell::new("00000"), Cell::new("0001"), Cell::new("02")]);
        table.push_row(vec![Cell::new("10"), Cell::new("11"), Cell::new("12")]);

        let expected = vec![5, 4, 2];
        assert_eq!(expected, table.cell_width_list());
    }

    #[test]
    fn cell_width_with_header() {
        let mut table = Table::new();
        table.push_row(vec![Cell::new("00000"), Cell::new("0001"), Cell::new("02")]);
        table.push_row(vec![Cell::new("10"), Cell::new("11"), Cell::new("12")]);
        table.set_header(Some(vec![
            Cell::new("hogehogehoge"),
            Cell::new("abcdefg"),
            Cell::new("x"),
        ]));

        let expected = vec![12, 7, 2];
        assert_eq!(expected, table.cell_width_list());
    }

    #[test]
    fn display_table_other_width() {
        let mut table = Table::new();
        table.push_row(vec![Cell::new("00000"), Cell::new("0001"), Cell::new("02")]);
        table.push_row(vec![Cell::new("10"), Cell::new("11"), Cell::new("12")]);

        let expected = "|00000|0001|02|\n|10   |11  |12|";
        let actual = format!("{}", table);
        assert_eq!(expected, actual);
    }

    #[test]
    fn display_table_width_header() {
        let mut table = Table::new();
        table.push_row(vec![Cell::new("00000"), Cell::new("0001"), Cell::new("02")]);
        table.push_row(vec![Cell::new("10"), Cell::new("11"), Cell::new("12")]);
        table.set_header(Some(vec![
            Cell::new("hogehogehoge"),
            Cell::new("abcdefg"),
            Cell::new("x"),
        ]));

        let expected =
            "|hogehogehoge|abcdefg|x |\n|------------|-------|--|\n|00000       |0001   |02|\n|10          |11     |12|";
        let actual = format!("{}", table);
        assert_eq!(expected, actual);
    }
}
