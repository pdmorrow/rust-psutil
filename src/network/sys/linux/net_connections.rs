use crate::common::{NetConnectionType, TcpConnectionStatus};
use crate::{Fd, Pid};

pub struct NetConnection {}

impl NetConnection {
	pub fn fd(&self) -> Option<Fd> {
		todo!()
	}

	// TODO: return type
	pub fn family(&self) {
		todo!()
	}

	// TODO: return type
	pub fn address_type(&self) {
		todo!()
	}

	// TODO: return type
	pub fn local_addr(&self) {
		todo!()
	}

	// TODO: return type
	pub fn remote_addr(&self) {
		todo!()
	}

	pub fn status(&self) -> Option<TcpConnectionStatus> {
		todo!()
	}

	pub fn pid(&self) -> Option<Pid> {
		todo!()
	}
}

pub fn net_connections() -> Vec<NetConnection> {
	todo!()
}

pub fn net_connections_with_type(_type: NetConnectionType) -> Vec<NetConnection> {
	todo!()
}
