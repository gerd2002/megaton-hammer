
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IClient(Session);

impl IClient {
	pub fn get_service() -> Result<IClient> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"bsd:u\0\0\0").map(|s| unsafe { IClient::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"bsd:s\0\0\0").map(|s| unsafe { IClient::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IClient {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IClient {
	pub fn Initialize(&self, config: ::nn::socket::BsdBufferConfig, pid: u64, transferMemorySize: u64, unk3: &KObject) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			config: ::nn::socket::BsdBufferConfig,
			pid: u64,
			transferMemorySize: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				config,
				pid,
				transferMemorySize,
			})
			.send_pid()
			.copy_handle(unk3)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn StartMonitoring(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Socket(&self, domain: u32, ty: u32, protocol: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			domain: u32,
			ty: u32,
			protocol: u32,
		}
		let req = Request::new(2)
			.args(InRaw {
				domain,
				ty,
				protocol,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn SocketExempt(&self, unk0: u32, unk1: u32, unk2: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn Open(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Select(&self, nfds: u32, timeout: ::nn::socket::timeout, readfds_in: &[::nn::socket::fd_set], writefds_in: &[::nn::socket::fd_set], errorfds_in: &[::nn::socket::fd_set], readfds_out: &mut [::nn::socket::fd_set], writefds_out: &mut [::nn::socket::fd_set], errorfds_out: &mut [::nn::socket::fd_set]) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			nfds: u32,
			timeout: ::nn::socket::timeout,
		}
		let req = Request::new(5)
			.args(InRaw {
				nfds,
				timeout,
			})
			.descriptor(IPCBuffer::from_slice(readfds_in, 0x21))
			.descriptor(IPCBuffer::from_slice(writefds_in, 0x21))
			.descriptor(IPCBuffer::from_slice(errorfds_in, 0x21))
			.descriptor(IPCBuffer::from_mut_slice(readfds_out, 0x22))
			.descriptor(IPCBuffer::from_mut_slice(writefds_out, 0x22))
			.descriptor(IPCBuffer::from_mut_slice(errorfds_out, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn Poll(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Sysctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Recv(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RecvFrom(&self, sock: u32, flags: u32, message: &mut [i8], unk6: &mut ::nn::socket::sockaddr) -> Result<(i32, u32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			sock: u32,
			flags: u32,
		}
		let req = Request::new(9)
			.args(InRaw {
				sock,
				flags,
			})
			.descriptor(IPCBuffer::from_mut_slice(message, 0x22))
			.descriptor(IPCBuffer::from_mut_ref(unk6, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	pub fn Send(&self, socket: u32, flags: u32, unk2: &[i8]) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			flags: u32,
		}
		let req = Request::new(10)
			.args(InRaw {
				socket,
				flags,
			})
			.descriptor(IPCBuffer::from_slice(unk2, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn SendTo(&self, socket: u32, flags: u32, unk2: &[i8], unk3: &::nn::socket::sockaddr) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			flags: u32,
		}
		let req = Request::new(11)
			.args(InRaw {
				socket,
				flags,
			})
			.descriptor(IPCBuffer::from_slice(unk2, 0x21))
			.descriptor(IPCBuffer::from_ref(unk3, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn Accept(&self, socket: u32, addr: &mut ::nn::socket::sockaddr) -> Result<(i32, u32, u32)> {
		let req = Request::new(12)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_ref(addr, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	pub fn Bind(&self, socket: u32, unk1: &::nn::socket::sockaddr) -> Result<(i32, u32)> {
		let req = Request::new(13)
			.args(socket)
			.descriptor(IPCBuffer::from_ref(unk1, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn Connect(&self, socket: u32, unk1: &::nn::socket::sockaddr) -> Result<(i32, u32)> {
		let req = Request::new(14)
			.args(socket)
			.descriptor(IPCBuffer::from_ref(unk1, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn GetPeerName(&self, socket: u32, addr: &mut ::nn::socket::sockaddr) -> Result<(i32, u32, u32)> {
		let req = Request::new(15)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_ref(addr, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	pub fn GetSockName(&self, socket: u32, addr: &mut ::nn::socket::sockaddr) -> Result<(i32, u32, u32)> {
		let req = Request::new(16)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_ref(addr, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	// fn GetSockOpt(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Listen(&self, socket: u32, backlog: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			backlog: u32,
		}
		let req = Request::new(18)
			.args(InRaw {
				socket,
				backlog,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn Ioctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Fcntl(&self, unk0: u32, unk1: u32, unk2: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
		}
		let req = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn SetSockOpt(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Shutdown(&self, socket: u32, how: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			how: u32,
		}
		let req = Request::new(22)
			.args(InRaw {
				socket,
				how,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn ShutdownAllSockets(&self, how: u32) -> Result<(i32, u32)> {
		let req = Request::new(23)
			.args(how)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn Write(&self, socket: u32, message: &[i8]) -> Result<(i32, u32)> {
		let req = Request::new(24)
			.args(socket)
			.descriptor(IPCBuffer::from_slice(message, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn Read(&self, socket: u32, message: &mut [i8]) -> Result<(i32, u32)> {
		let req = Request::new(25)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_slice(message, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn Close(&self, socket: u32) -> Result<(i32, u32)> {
		let req = Request::new(26)
			.args(socket)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn DuplicateSocket(&self, unk0: u32, unk1: u64) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(27)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn GetResourceStatistics(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RecvMMsg(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SendMMsg(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IClient {
	unsafe fn from_kobject(obj: KObject) -> IClient {
		IClient(Session::from_kobject(obj))
	}
}
