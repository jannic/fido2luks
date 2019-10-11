#[macro_use]
extern crate failure;
extern crate ctap_hmac as ctap;
use crate::cli::*;
use crate::config::*;
use crate::device::*;
use crate::error::*;
use digest::Digest;
use sha2::Sha256;
use cryptsetup_rs as luks;
use cryptsetup_rs::Luks1CryptDevice;

use std::io::{self};
use std::path::PathBuf;

mod cli;
mod config;
mod device;
mod error;
mod util;

fn open_container(device: &PathBuf, name: &str, secret: &[u8; 32]) -> Fido2LuksResult<()> {
    let mut handle = luks::open(device.canonicalize()?)?.luks1()?;
    let _slot = handle.activate(name, &secret[..])?;
    Ok(())
}

fn assemble_secret(hmac_result: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.input(salt);
    digest.input(hmac_result);
    digest.result().into()
}

fn main() -> Fido2LuksResult<()> {
    run_cli()
}
