#![no_std]
use rtt_target::rprintln;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
    pub const BIN_NAME: &str = env!("CARGO_BIN_NAME");
}

pub fn print_fw_info() {
    rprintln!(
        "{}/{} v{} debug={} opt={} {}",
        built_info::PKG_NAME,
        built_info::BIN_NAME,
        built_info::PKG_VERSION,
        built_info::DEBUG,
        built_info::OPT_LEVEL,
        built_info::PKG_REPOSITORY
    );
    rprintln!(
        "target={} built_on={} {}",
        built_info::TARGET,
        built_info::HOST,
        built_info::RUSTC_VERSION
    );
    rprintln!("features=\"{}\"", built_info::FEATURES_STR);
}
