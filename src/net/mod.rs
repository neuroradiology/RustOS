use std::io::IoResult;
use std::prelude::*;
use core::mem::{size_of, transmute};

use driver::NetworkDriver;

pub struct NetworkStack<'a> {
  card: Box<NetworkDriver + 'a>
}

impl<'a> NetworkStack<'a> {
  
  pub fn new(card: Box<NetworkDriver>) -> NetworkStack<'a> { 
    NetworkStack { card: card }
  }
  
  pub fn test(&mut self) -> IoResult<()> {
    let address = self.card.address();
      
    for i in range(0, 10u) {
      self.card.put_frame(format!("\nhello, etherworld! sending frame # {} !\n", i).as_bytes()).ok();
    }
      
    let source = address;
    let destination = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
      
    let raw = [b'u', b'd', b'p', b'!'];
    let u_header = UdpHeader::new(10, 10, raw.len() as u16);
    let i_header = IpHeader::new((raw.len() + size_of::<UdpHeader>()) as u16, 0x11, 15, 15);
    let header = EthernetHeader::new(source, destination, 0x0800);
      
    let to_send = &(header, i_header, u_header, raw);
      
    self.card.put_frame(unsafe { transmute ((to_send, size_of::<(EthernetHeader, IpHeader, UdpHeader)>() + raw.len())) }).ok();
    Ok(())
  }

}

#[allow(dead_code)]
#[repr(packed)]
struct UdpHeader {
  source_port: u16,
  destination_port: u16,
  length: u16,
  crc: u16
}

impl UdpHeader {

  fn new(source_port: u16, destination_port: u16, length: u16) -> UdpHeader {
    UdpHeader {
      source_port: source_port.to_be(),
      destination_port: destination_port.to_be(),
      length: (size_of::<UdpHeader>() as u16 + length).to_be(),
      crc: 0
    }
  }

}

#[allow(dead_code)]
#[repr(packed)]
struct IpHeader {
  version_length: u8,
  tos: u8,
  length: u16,
  
  id: [u8,..3],
  flags_fragment: u8,
  
  ttl: u8,
  protocol: u8,
  crc: u16,
  
  source: u32,
  
  destination: u32,
  
}

impl IpHeader {

  fn new(payload_length: u16, protocol: u8, source: u32, destination: u32) -> IpHeader {
    IpHeader {
      version_length: ((0x4) << 4) | 5,
      tos: 0,
      length: size_of::<IpHeader>() as u16 + payload_length.to_be(),
      id: [0, 0, 0],
      flags_fragment: 0,
      ttl: 30,
      protocol: protocol,
      source: source,
      destination: destination,
      crc: 0
    }
  }

}

#[allow(dead_code)]
#[repr(packed)]
struct EthernetHeader {
  //preamble: [u8,..8],
  destination: [u8,..6],
  source: [u8,..6],
  typ: u16,
}

impl EthernetHeader {

  fn new(source: [u8,..6], destination: [u8,..6], typ: u16) -> EthernetHeader {
    //let r = 0b10101010;
    //let n = 0b10101011;
    EthernetHeader {
      //preamble: [r, r, r, r, r, r, r, n],
      destination: destination,
      source: source,
      typ: typ.to_be()
    }
  }

}