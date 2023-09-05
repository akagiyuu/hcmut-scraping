#![feature(never_type)]
mod news;
mod diff;

use std::time::Duration;

use anyhow::Result;

fn main() -> Result<!> {
    news::sync_start(Duration::from_secs(3600))?;
}
