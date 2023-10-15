#![no_std]
use rtt_target::rprintln;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn print_fw_info(bin_name: &'static str) {
    rprintln!(
        "{}/{} v{} debug={} opt={} {}",
        built_info::PKG_NAME,
        bin_name,
        // built_info::BIN_NAME,
        built_info::PKG_VERSION,
        built_info::DEBUG,
        built_info::OPT_LEVEL,
        built_info::PKG_REPOSITORY
    );
    rprintln!(
        "target={} built_on={} {} {}",
        built_info::TARGET,
        built_info::HOST,
        built_info::BUILT_TIME_UTC,
        built_info::RUSTC_VERSION
    );
    rprintln!(
        "git hash={:?} dirty={:?}",
        built_info::GIT_COMMIT_HASH_SHORT,
        built_info::GIT_DIRTY
    );
    rprintln!("features=\"{}\"", built_info::FEATURES_STR);
}
