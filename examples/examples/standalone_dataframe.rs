use ballista::prelude::{BallistaConfig, BallistaContext, Result};
use datafusion::prelude::CsvReadOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let config = BallistaConfig::builder()
        .set("ballista.shuffle.partitions", "1")
        .build()?;

    let ctx = BallistaContext::standalone(&config, 2).await?;

    let filename = "/home/tinlou/Documents/datasets/significant_earthquakes/database.csv";

    // define the query using the DataFrame trait
    let df = ctx
        .read_csv(filename, CsvReadOptions::new())
        .await?
        .select_columns(&["Date", "Depth"])?;

    df.show().await?;
    Ok(())
}
