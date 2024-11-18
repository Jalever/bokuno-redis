use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("./src/foo.txt").await?;
    let mut buf = [0; 10];

    let n = f.read(&mut buf[..]).await?;
    println!("The bytes is {:?}", &buf[..n]);
    Ok(())
}
