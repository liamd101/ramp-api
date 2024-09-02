mod api;

mod config;
mod data;
// mod database;

use anyhow::{anyhow, Result};
use tracing::{error, info, debug, Level};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<()> {
    let log_file = std::fs::File::create("out.log")?;
    tracing_subscriber::fmt::fmt()
        .with_writer(std::sync::Mutex::new(log_file))
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(false)
        .init();

    info!("Starting the application");

    let settings = match config::Settings::new("config.toml".to_string()) {
        Ok(s) => {
            info!("Successfully loaded settings");
            s
        }
        Err(e) => {
            error!("Failed to load settings: {}", e);
            return Err(anyhow!("Error getting token: {}", e));
        }
    };

    let token = match api::get_token(settings.client).await {
        Ok(t) => {
            info!("Successfully retrieved token: {}", t);
            t
        }
        Err(e) => {
            error!("Error getting token: {}", e);
            return Err(anyhow!("Error getting token: {}", e));
        }
    };

    let data: Vec<data::DataEntry> = match api::get_data(&token).await {
        Ok(d) => {
            info!("Successfully retrieved data");
            info!("Data count: {}", d.len());
            info!("Data: {:?}", d);
            d
        }
        Err(e) => {
            error!("Error getting data: {}", e);
            return Err(anyhow!("Error getting data: {}", e));
        }
    };

    // match database::insert_data_server(settings.database, data) {
    //     Ok(()) => info!("Successfully inserted data"),
    //     Err(e) => {
    //         error!("Error inserting data: {}", e);
    //         return Err(anyhow!("Error inserting data: {}", e));
    //     }
    // }

    info!("Successfully ran application");

    Ok(())
}
