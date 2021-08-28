use anyhow::Result;
use clap::{App, Arg};
use tokio::io::BufReader;

mod data;
mod input;
mod utils;

pub struct Application<'a, 'b> {
    app: App<'a, 'b>,
}

impl<'a, 'b> Application<'a, 'b> {
    pub fn new() -> Self {
        let app = App::new("tv")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("Format json and csv into table view")
            .arg(Arg::with_name("PATH").help("json, csv file path").index(1))
            .arg(
                Arg::with_name("sort")
                    .short("s")
                    .long("sort")
                    .value_name("SORT_KEY")
                    .help("Options for sorting by key")
                    .takes_value(true),
            );

        Self { app }
    }

    pub async fn run(&mut self) -> Result<()> {
        let matcher = self.app.clone().get_matches();
        let path = matcher.value_of("PATH").map(String::from);

        if path.is_none() && !input::is_pipe() {
            let mut out = std::io::stdout();
            self.app.write_long_help(&mut out)?;
            return Ok(());
        }

        let raw = match path {
            Some(p) => {
                let file = tokio::fs::File::open(String::from(p)).await?;
                let mut reader = BufReader::new(file);
                input::read(&mut reader).await
            }
            None => input::read_stdin().await,
        }?;

        let mut data = data::Data::from(&raw)?;
        let sort_key = matcher.value_of("sort");
        if let Some(key) = sort_key {
            data.set_sort_key(key);
        }

        println!("{}", data);

        Ok(())
    }
}
