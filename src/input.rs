use anyhow::Result;
use tokio::io::{stdin, AsyncReadExt, BufReader};

pub async fn read<R: AsyncReadExt + std::marker::Unpin>(
    reader: &mut BufReader<R>,
) -> Result<String> {
    let mut buf = String::new();
    reader.read_to_string(&mut buf).await?;
    Ok(buf)
}

pub async fn read_stdin() -> Result<String> {
    let mut reader = BufReader::new(stdin());
    read(&mut reader).await
}

pub fn is_pipe() -> bool {
    atty::isnt(atty::Stream::Stdin)
}
