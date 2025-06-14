use std::error::Error;
use std::fs::File;
use std::net::AddrParseError;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(AddrParseError),
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")?.maybe_convert_to(UpstreamError);

    let _localhost = "::1".parse::<Ipv6Addr>()?.maybe_convert_to(UpstreamError);

    Ok(())
}
