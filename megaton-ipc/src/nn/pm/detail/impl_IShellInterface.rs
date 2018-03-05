
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IShellInterface(Session);

impl IShellInterface {
	pub fn LaunchProcess(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn TerminateProcessByPid(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn TerminateProcessByTitleId(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetProcessEventWaiter(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetProcessEventType(&self, ) -> Result<u128> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn FinalizeDeadProcess(&self, unk0: u64) -> Result<()> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ClearProcessNotificationFlag(&self, unk0: u64) -> Result<()> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn NotifyBootFinished(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetApplicationPid(&self, ) -> Result<u64> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IShellInterface {
	unsafe fn from_kobject(obj: KObject) -> IShellInterface {
		IShellInterface(Session::from_kobject(obj))
	}
}
