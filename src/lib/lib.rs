#![no_std]
use crate::cli::cli_print_prompt;
use core::fmt::Write;
use rtt_target::{rprintln, rtt_init, DownChannel, UpChannel};

pub mod cli;
#[allow(dead_code)]
pub mod vt100;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub struct RttIo {
    pub cli_output: UpChannel,
    pub cli_input: DownChannel,
    cli_buf: [u8; 128],
    cli_buf_pos: usize,
    pub log: UpChannel,
}

pub fn init(bin_name: &'static str) -> RttIo {
    let channels = rtt_init! {
        up: {
            0: {
                size: 512
                // mode: BlockIfFull
                mode: NoBlockSkip
                name: "Terminal"
            }
            1: {
                size: 512
                name: "Log"
            }
            // 2: {
            //     size: 128
            //     name: "Up two"
            // }
        }
        down: {
            0: {
                size: 128
                mode: NoBlockSkip
                name: "Terminal"
            }
        }
    };
    let mut io = RttIo {
        cli_output: channels.up.0,
        cli_input: channels.down.0,
        cli_buf: [0u8; 128],
        cli_buf_pos: 0,
        log: channels.up.1,
    };
    print_fw_info(&mut io, bin_name);
    cli_print_prompt(&mut io);
    io
}

pub fn print_fw_info(io: &mut RttIo, bin_name: &'static str) {
    writeln!(io.cli_output, "{}", vt100::DIM).ok();
    writeln!(
        io.cli_output,
        "{}/{} v{} debug={} opt={} {}",
        built_info::PKG_NAME,
        bin_name,
        // built_info::BIN_NAME,
        built_info::PKG_VERSION,
        built_info::DEBUG,
        built_info::OPT_LEVEL,
        built_info::PKG_REPOSITORY
    )
    .ok();
    writeln!(
        io.cli_output,
        "target={} built_on={} {} {}",
        built_info::TARGET,
        built_info::HOST,
        built_info::BUILT_TIME_UTC,
        built_info::RUSTC_VERSION
    )
    .ok();
    writeln!(
        io.cli_output,
        "git {} hash={} dirty={}",
        built_info::GIT_HEAD_REF.unwrap_or(""),
        built_info::GIT_COMMIT_HASH_SHORT.unwrap_or(""),
        built_info::GIT_DIRTY.unwrap_or(true),
    )
    .ok();
    writeln!(io.cli_output, "features=\"{}\"", built_info::FEATURES_STR).ok();
    writeln!(io.cli_output, "{}", vt100::DEFAULT).ok();
}
