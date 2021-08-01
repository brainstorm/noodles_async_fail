use tokio::fs::File;

use noodles::sam;
use noodles_bgzf::VirtualPosition;
use noodles::bam::Record;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // let mut reader = File::open("example").await.map(noodles_bam::AsyncReader::new)?;
  // let header: sam::Header = reader.read_header().await?.parse()?;
  // tokio::spawn(async move {
  //   reader.read_reference_sequences().await;
  // });
  // Ok(())

  let mut async_reader = File::open("data/htsnexus_test_NA12878.bam").await.map(noodles_bam::AsyncReader::new)?;
  async_reader.read_header().await?;
  async_reader.read_reference_sequences().await?;

  let mut blocking_reader = std::fs::File::open("data/htsnexus_test_NA12878.bam").map(noodles_bam::Reader::new)?;
  blocking_reader.read_header()?;
  blocking_reader.read_reference_sequences()?;

  let mut async_bytes_read = async_reader.read_record(&mut Record::default()).await?;
  let mut blocking_bytes_read = blocking_reader.read_record(&mut Record::default())?;
  let mut async_vpos = async_reader.virtual_position();
  let mut blocking_vpos = blocking_reader.virtual_position();
  let mut i = 0;

  while async_bytes_read != 0 && blocking_bytes_read != 0 {
    if async_bytes_read != blocking_bytes_read {
      println!("Iteration {}, async bytes read: {}, blocking bytes read: {}", i, async_bytes_read, blocking_bytes_read);
    }
    if async_vpos != blocking_vpos {
      println!("Iteration {}, async virtual position: {}, blocking virtual position: {}", i, u64::from(async_vpos), u64::from(blocking_vpos));
    }

    async_bytes_read = async_reader.read_record(&mut Record::default()).await?;
    blocking_bytes_read = blocking_reader.read_record(&mut Record::default())?;
    async_vpos = async_reader.virtual_position();
    blocking_vpos = blocking_reader.virtual_position();
    i += 1;
  }

  Ok(())
}
