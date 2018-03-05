
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDeliveryCacheProgressService(Session);

impl IDeliveryCacheProgressService {
	// fn GetEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetImpl(&self, unk0: &mut Option<::nn::bcat::detail::DeliveryCacheProgressImpl>) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IDeliveryCacheProgressService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheProgressService {
		IDeliveryCacheProgressService(Session::from_kobject(obj))
	}
}
