use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use std::{env, time::Duration};

mod error;
pub use error:Error;
mod model;
mod ports;
mod subdomains;
use model::Subdomain;
mod common_ports;

fn main() -> Result<(), anyhow::Error> {
    // ...
}