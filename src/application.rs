use crate::input;
use anyhow::Result;
use clap::{App, Arg};
use tokio::io::BufReader;
use tableview::data::Data;
use tableview::table::{cell::Align, style::Style, Table};

pub struct Application<'a, 'b> {
    app: App<'a, 'b>,
    path: Option<String>,
    sort_key: Option<String>,
    align: Align,
    style: Style,
    no_headers: bool,
    recursive: bool,
}

impl<'a, 'b> Application<'a, 'b> {
    pub fn new() -> Self {
        let app = App::new("tv")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .global_setting(clap::AppSettings::ColoredHelp)
            .arg(Arg::with_name("PATH").help("json file path").index(1))
            .arg(
                Arg::with_name("sort")
                    .short("s")
                    .long("sort")
                    .value_name("SORT_KEY")
                    .help("Options for sorting by key")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("align")
                    .short("a")
                    .long("align")
                    .value_name("left | center | right | none")
                    .help("Table alignment")
                    .takes_value(true)
                    .default_value("none"),
            )
            .arg(
                Arg::with_name("style")
                    .long("style")
                    .value_name("ascii | sharp | rounded | markdown | plane")
                    .help("Table style")
                    .takes_value(true)
                    .default_value("ascii"),
            )
            .arg(
                Arg::with_name("no headers")
                    .long("no-headers")
                    .help("Specify that the input has no header row"),
            )
            .arg(
                Arg::with_name("recursive")
                    .short("r")
                    .long("recursive")
                    .help("Recursive display"),
            );

        let matcher = app.clone().get_matches();
        let path = matcher.value_of("PATH").map(String::from);
        let sort_key = matcher.value_of("sort").map(String::from);
        let align = matcher
            .value_of("align")
            .map(String::from)
            .map(Align::new)
            .unwrap_or(Align::None);
        let style = matcher
            .value_of("style")
            .map(String::from)
            .map(Style::new)
            .unwrap_or(Style::Ascii);
        let no_headers = matcher.is_present("no headers");
        let recursive = matcher.is_present("recursive");

        Self {
            app,
            path,
            sort_key,
            align,
            style,
            no_headers,
            recursive,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        if self.path.is_none() && !input::is_pipe() {
            self.app.print_long_help()?;
            return Ok(());
        }

        let raw = match self.path.clone() {
            Some(p) => {
                let file = tokio::fs::File::open(String::from(p)).await?;
                let mut reader = BufReader::new(file);
                input::read(&mut reader).await
            }
            None => input::read_stdin().await,
        }?;

        let mut data = Data::from(&raw)?;
        data.set_sort_key(self.sort_key.clone());

        self.show(data);

        Ok(())
    }

    fn show(&self, data: Data) {
        let mut table: Table<String> = data.clone().into();
        table
            .set_style(self.style)
            .set_align(self.align)
            .set_no_headers(self.no_headers);

        println!("{}", table);

        if self.recursive {
            let nested_fields = data.clone().nested_fields();
            nested_fields.into_iter().for_each(|(k, v)| {
                println!("\n# {}", k);
                let mut data = v;
                data.set_sort_key(self.sort_key.clone());
                self.show(data);
            });
        }
    }
}
