
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IManagerRootService(Session);

impl IManagerRootService {
	pub fn GetDisplayService(&self, unk0: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn GetDisplayServiceWithProxyNameExchange(&self, unk0: ::nn::vi::ProxyName, unk1: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::vi::ProxyName,
			unk1: u32,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
}

impl FromKObject for IManagerRootService {
	unsafe fn from_kobject(obj: KObject) -> IManagerRootService {
		IManagerRootService(Session::from_kobject(obj))
	}
}
