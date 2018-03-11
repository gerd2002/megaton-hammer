
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct ISaveDataInfoReader(Session);

impl AsRef<Session> for ISaveDataInfoReader {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISaveDataInfoReader {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ISaveDataInfoReader {
	unsafe fn from_kobject(obj: KObject) -> ISaveDataInfoReader {
		ISaveDataInfoReader(Session::from_kobject(obj))
	}
}
