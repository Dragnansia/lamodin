//! Used to download file

use crate::error::Error;
use futures_util::StreamExt;
use reqwest::{self, header::USER_AGENT, Client};
use std::{fs::File, io::Write};

/// Trait to download file
///
/// # Example
/// ```
/// use lamodin::downloader::Download;
///
/// struct Dl {
///     total_size: u64,
/// }
///
/// impl Download for Dl {
///     fn init(&mut self, size: u64) {
///         self.total_size = size;
///     }
///
///     fn update(&mut self, chunk: &[u8]) {}
/// }
/// ```
pub trait Download {
    /// Initialize struct with size of download file
    fn init(&mut self, size: u64);

    /// Give Bytes download
    fn update(&mut self, chunk: &[u8]);
}

/// Download file on path
///
/// # Example
/// ```
/// use lamodin::{
///     downloader::{Download, file},
///     error::Error
/// };
/// use std::cmp::min;
///
/// #[derive(Default)]
/// struct Dl {
///     total_size: u64,
///     download: u64,
/// };
///
/// impl Download for Dl {
///     fn init(&mut self, size: u64) {
///         self.total_size = size;
///     }
///
///     fn update(&mut self, chunk: &[u8]) {
///         self.download = min(self.download + (chunk.len() as u64), self.total_size);
///         println!("Total Download: {}", self.download);
///     }
/// }
///
/// async fn func() -> Result<(), Error> {
///     let url = "https://www.my-api.rs/big/file";
///     file(url, "path", &mut Dl::default()).await?;
///     Ok(())
/// }
/// ```
pub async fn file<D>(url: &str, path: &str, download: &mut D) -> Result<(), Error>
where
    D: Download,
{
    let client = Client::new();
    let response = client.get(url).header(USER_AGENT, "purs").send().await?;
    let size = response
        .content_length()
        .ok_or("Can't get content length")?;

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
