pub mod history;
pub mod packet_buffer;

use log::{debug, info, trace};
use history::History;

#[derive(PartialEq, Debug)]
pub enum NPerfMode {
    Client,
    Server,
}

#[derive(PartialEq, Debug)]
pub enum ExchangeFunction {
    Normal,
    Msg,
    Mmsg
}

pub fn parse_mode(mode: String) -> Option<NPerfMode> {
    match mode.as_str() {
        "client" => Some(NPerfMode::Client),
        "server" => Some(NPerfMode::Server),
        _ => None,
    }
}


pub fn process_packet(buffer: &[u8], next_packet_id: u64, history: &mut History) -> u64 {
    let packet_id = u64::from_be_bytes(buffer[0..8].try_into().unwrap());
    debug!("Received packet number: {}", packet_id);
    process_packet_number(packet_id, next_packet_id, history)
}

// Packet reordering taken from iperf3 and rperf https://github.com/opensource-3d-p/rperf/blob/14d382683715594b7dce5ca0b3af67181098698f/src/stream/udp.rs#L225
// https://github.com/opensource-3d-p/rperf/blob/14d382683715594b7dce5ca0b3af67181098698f/src/stream/udp.rs#L225 
fn process_packet_number(packet_id: u64, next_packet_id: u64, history: &mut History) -> u64 {
    if packet_id == next_packet_id {
        return 1
    } else if packet_id > next_packet_id {
        let lost_packet_count = (packet_id - next_packet_id) as u64;
        history.amount_omitted_datagrams += lost_packet_count as i64;
        info!("Reordered or lost packet received! Expected number {}, but received {}. {} packets are currently missing", next_packet_id, packet_id, lost_packet_count);
        return lost_packet_count + 1; // This is the next packet id that we expect, since we assume that the missing packets are lost
    } else { // If the received packet_id is smaller than the expected, it means that we received a reordered (or duplicated) packet.
        if history.amount_omitted_datagrams > 0 { 
            history.amount_omitted_datagrams -= 1;
            history.amount_reordered_datagrams  += 1;
            info!("Received reordered packet number {}, but expected {}", packet_id, next_packet_id);
        } else { 
            history.amount_duplicated_datagrams += 1;
            info!("Received duplicated packet");
        }
        return 0
    }
}

pub fn create_msghdr(buffer: &mut [u8], buffer_len: usize) -> libc::msghdr {
    let mut msghdr: libc::msghdr = unsafe { std::mem::zeroed() };

    let iov = Box::new(libc::iovec {
        iov_base: buffer.as_mut_ptr() as *mut _,
        iov_len: buffer_len,
    });

    msghdr.msg_name = std::ptr::null_mut();
    msghdr.msg_namelen = 0;
    msghdr.msg_iov = Box::into_raw(iov) as *mut _ as _;
    msghdr.msg_iovlen = 1;
    msghdr.msg_control = std::ptr::null_mut();
    msghdr.msg_controllen = 0;

    msghdr
}

pub fn add_cmsg_buffer(msghdr: &mut libc::msghdr) {
    let control = Box::new([0u8; 1000]);
    let control_len = control.len();
    msghdr.msg_control = Box::into_raw(control) as *mut _ as _;
    msghdr.msg_controllen = control_len as _;
}

pub fn get_gso_size_from_cmsg(msghdr: &mut libc::msghdr) -> Option<u32> {
    let mut cmsg: *mut libc::cmsghdr = unsafe { libc::CMSG_FIRSTHDR(msghdr) };
    while !cmsg.is_null() {
        let level = unsafe { (*cmsg).cmsg_level };
        let cmsg_type = unsafe { (*cmsg).cmsg_type };
        let cmsg_len = unsafe { (*cmsg).cmsg_len };
        debug!("Received cmsg with level: {}, type: {}, len: {}", level, cmsg_type, cmsg_len);

        if level == libc::SOL_UDP && cmsg_type == libc::UDP_GRO {
            let data_ptr = unsafe { libc::CMSG_DATA(cmsg) };
            let gso_size = unsafe { *(data_ptr as *const u32) };
            debug!("Received GSO size in cmsg: {}", gso_size);
            return Some(gso_size);
        }

        cmsg = unsafe { libc::CMSG_NXTHDR(msghdr, cmsg) };
    }
    None
}