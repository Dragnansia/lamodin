use crate::error::Error;
use futures_util::StreamExt;
use reqwest::{self, header::USER_AGENT, Client};
use std::{fs::File, io::Write};

/// Trait to download file
///
/// # Exemple
/// ```
/// use purs::downloader::Download;
///
/// struct Dl;
/// impl Download for Dl {
///     fn init(&mut self, size: u64) {}
///     fn update(&mut self, chunk: &[u8]) {}
/// }
/// ```
pub trait Download {
    fn init(&mut self, size: u64);
    fn update(&mut self, chunk: &[u8]);
}

/// Download file on path
///
/// # Exemples
/// ```
/// use purs::downloader::{Download, file};
///
/// struct Dl;
/// impl Download for Dl {
///     fn init(&mut self, size: u64) {}
///     fn update(&mut self, chunk: &[u8]) {}
/// }
///
///
/// let url = "https://www.my-api.rs/big/file";
/// file(url, "path", &mut Dl {}).await;
///
/// ```
pub async fn file<D>(url: &str, path: &str, download: &mut D) -> Result<(), Error>
where
    D: Download,
{
    let client = Client::new();
    let response = client.get(url).header(USER_AGENT, "purs").send().await?;
    let size = response.content_length().unwrap_or_default();

    let mut file = File::create(&path)?;
    let mut stream = response.bytes_stream();

    download.init(size);
    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        download.update(&chunk);
    }

    Ok(())
}
