
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IFileSystemProxyForLoader(Session);

impl IFileSystemProxyForLoader {
	pub fn get_service() -> Result<IFileSystemProxyForLoader> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"fsp-ldr\0").map(|s| unsafe { IFileSystemProxyForLoader::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFileSystemProxyForLoader {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFileSystemProxyForLoader {
	// fn MountCode(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn IsCodeMounted(&self, TID: ::nn::ApplicationId) -> Result<u8> {
		let req = Request::new(1)
			.args(TID)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IFileSystemProxyForLoader {
	unsafe fn from_kobject(obj: KObject) -> IFileSystemProxyForLoader {
		IFileSystemProxyForLoader(Session::from_kobject(obj))
	}
}
