use deadqueue::unlimited::Queue;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, USER_AGENT};
use tokio::sync::Mutex;

use crate::types::download::DownloadJob;

lazy_static! {
    pub static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .default_headers(HeaderMap::from_iter([
            (USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36")),
            (ACCEPT_ENCODING, HeaderValue::from_static("gzip")),
        ]))
        .build()
        .unwrap();

    pub static ref QUEUE: Queue<DownloadJob> = Queue::new();

    pub static ref MULTI_PROGRESS_BAR: indicatif::MultiProgress = indicatif::MultiProgress::new();

    pub static ref TOTAL_PROGRESS_BAR: ProgressBar = MULTI_PROGRESS_BAR.add(ProgressBar::new(0));

    pub static ref SUCCESSFUL_DOWNLOADS: Mutex<Vec<DownloadJob>> = Mutex::new(Vec::new());

    pub static ref FAILED_DOWNLOADS: Mutex<Vec<DownloadJob>> = Mutex::new(Vec::new());
}
