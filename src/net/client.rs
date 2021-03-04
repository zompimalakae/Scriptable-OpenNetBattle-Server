use super::Navi;
use crate::packets::PacketShipper;
use std::collections::HashSet;

pub(super) struct Client {
  pub socket_address: std::net::SocketAddr,
  pub packet_shipper: PacketShipper,
  pub navi: Navi,
  pub warp_in: bool,
  pub ready: bool,
  pub cached_assets: HashSet<String>,
}