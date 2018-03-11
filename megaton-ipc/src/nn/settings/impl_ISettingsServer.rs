
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct ISettingsServer(Session);

impl ISettingsServer {
	pub fn get_service() -> Result<ISettingsServer> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"set\0\0\0\0\0").map(|s| unsafe { ISettingsServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISettingsServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISettingsServer {
	pub fn GetLanguageCode(&self, ) -> Result<::nn::settings::LanguageCode> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::settings::LanguageCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetAvailableLanguageCodes(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-4.0.0")]
	pub fn MakeLanguageCode(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAvailableLanguageCodeCount(&self, ) -> Result<i32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetRegionCode(&self, ) -> Result<i32> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn GetAvailableLanguageCodes2(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn GetAvailableLanguageCodeCount2(&self, ) -> Result<()> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn GetKeyCodeMap(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISettingsServer {
	unsafe fn from_kobject(obj: KObject) -> ISettingsServer {
		ISettingsServer(Session::from_kobject(obj))
	}
}
