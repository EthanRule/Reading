use clap::{App, Arg};
use rand;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::*;

fn main() {
    println!("Hello, world!");
}
