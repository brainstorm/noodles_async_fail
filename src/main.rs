use tokio::fs::File;

use noodles::sam;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let reader = File::open("example").await.map(noodles_bam::AsyncReader::new)?;
  let header: sam::Header = reader.read_header().await?.parse()?;
  tokio::spawn(async move {
    reader.read_reference_sequences().await;
  });
  Ok(())
}
