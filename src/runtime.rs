use crate::config::Config;
use anyhow::Error;
use tokio::runtime::{Builder, Runtime};

#[tracing::instrument(level = "debug")]
#[cfg_attr(not(feature = "threaded"), allow(unused_variables))]
pub fn build(config: &Config) -> Result<Runtime, Error> {
    let mut builder = Builder::new();
    builder.basic_scheduler().enable_all();

    #[cfg(feature = "threaded")]
    {
        builder.threaded_scheduler();

        let config = &config.threads;
        if let Some(ct) = config.core_threads {
            builder.core_threads(ct);
        }
        if let Some(mt) = config.max_threads {
            builder.max_threads(mt);
        }
    }

    Ok(builder.build()?)
}
