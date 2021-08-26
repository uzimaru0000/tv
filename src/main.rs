use std::collections::VecDeque;

use anyhow::Result;
use tokio::io::BufReader;
use tv::input;
use tv::data;

struct Args {
    path: Option<String>
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = get_args();
    let raw = match args.path {
        Some(path) => {
            let file = tokio::fs::File::open(path).await?;
            let mut reader = BufReader::new(file);
            input::read(&mut reader).await
        }
        None => input::read_stdin().await
    }?;
    let data = data::Data::from(&raw)?;

    println!("{}", data);
    Ok(())
}

fn get_args() -> Args {
    let mut args = std::env::args().collect::<VecDeque<_>>();
    args.pop_front();
    let path = args.pop_front();

    Args {
        path
    }
}
