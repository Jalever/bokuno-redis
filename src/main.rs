use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::create("./src/write_all-to-file.txt").await?;
    file.write_all(b"some bytes").await?;
    Ok(())
}
