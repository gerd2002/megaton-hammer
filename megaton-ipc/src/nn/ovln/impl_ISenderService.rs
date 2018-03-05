
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISenderService(Session);

impl ISenderService {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for ISenderService {
	unsafe fn from_kobject(obj: KObject) -> ISenderService {
		ISenderService(Session::from_kobject(obj))
	}
}
