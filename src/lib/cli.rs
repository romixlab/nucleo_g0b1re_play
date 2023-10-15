use crate::{print_fw_info, RttIo};
use core::fmt::Write;
use core::str::SplitAsciiWhitespace;

pub fn cli_print_prompt(io: &mut RttIo) {
    write!(io.cli_output, "cli> ").ok();
}

pub fn cli<F: FnMut(&str, &mut SplitAsciiWhitespace, &mut dyn Write)>(
    io: &mut RttIo,
    mut handler: F,
) {
    let buf = &mut io.cli_buf;
    let read_len = io.cli_input.read(&mut buf[io.cli_buf_pos..]);
    if read_len == 0 {
        return;
    }
    io.cli_output
        .write(&buf[io.cli_buf_pos..io.cli_buf_pos + read_len]);
    io.cli_buf_pos += read_len;
    if buf[io.cli_buf_pos - 1] != b'\n' {
        return;
    }
    let Ok(args) = core::str::from_utf8(&buf[..io.cli_buf_pos]) else {
        io.cli_buf_pos = 0;
        return;
    };
    io.cli_buf_pos = 0;
    let mut args = args.split_ascii_whitespace();
    let Some(cmd) = args.next() else {
        return;
    };
    if cmd == "fwinfo" {
        print_fw_info(io);
    } else {
        handler(cmd, &mut args, &mut io.cli_output);
    }

    cli_print_prompt(io);
}
