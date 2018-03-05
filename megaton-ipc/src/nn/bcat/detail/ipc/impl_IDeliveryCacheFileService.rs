
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDeliveryCacheFileService(Session);

impl IDeliveryCacheFileService {
	pub fn Open(&self, unk0: ::nn::bcat::DirectoryName, unk1: ::nn::bcat::FileName) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::bcat::DirectoryName,
			unk1: ::nn::bcat::FileName,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetSize(&self, ) -> Result<i64> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetDigest(&self, ) -> Result<::nn::bcat::Digest> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<::nn::bcat::Digest> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IDeliveryCacheFileService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheFileService {
		IDeliveryCacheFileService(Session::from_kobject(obj))
	}
}
