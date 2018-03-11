
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IHardwareOpusDecoder(Session);

impl AsRef<Session> for IHardwareOpusDecoder {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHardwareOpusDecoder {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IHardwareOpusDecoder {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoder {
		IHardwareOpusDecoder(Session::from_kobject(obj))
	}
}
