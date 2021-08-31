use crate::input;
use anyhow::Result;
use clap::{App, Arg};
use tokio::io::BufReader;
use tv::data::{Align, Data};

pub struct Application<'a, 'b> {
    app: App<'a, 'b>,
    path: Option<String>,
    sort_key: Option<String>,
    is_plane: bool,
    align: Align,
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
                Arg::with_name("plane")
                    .short("p")
                    .long("plane")
                    .help("Do not Display border"),
            )
            .arg(
                Arg::with_name("align")
                    .short("a")
                    .long("align")
                    .value_name("left | center | right | none")
                    .help("Table alignment")
                    .takes_value(true)
                    .default_value("none"),
            );

        let matcher = app.clone().get_matches();
        let path = matcher.value_of("PATH").map(String::from);
        let sort_key = matcher.value_of("sort").map(String::from);
        let is_plane = matcher.is_present("plane");
        let align = matcher
            .value_of("align")
            .map(String::from)
            .map(Align::new)
            .unwrap_or(Align::None);

        Self {
            app,
            path,
            sort_key,
            is_plane,
            align,
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
        data.set_sort_key(self.sort_key.clone())
            .set_is_plane(self.is_plane)
            .set_align(self.align);

        println!("{}", data);

        Ok(())
    }
}
