
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILocalManager(Session);

impl ILocalManager {
	pub fn new() -> Result<Arc<ILocalManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ILocalManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"wlan:lcl") {
			let ret = Arc::new(ILocalManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"wlan:lcl").map(|s| Arc::new(unsafe { ILocalManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ILocalManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILocalManager {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown3(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown5(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown8(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown10(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown12(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown14(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown16(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown17(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown18(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown19(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown20(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown21(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown22(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown23(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown24(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown25(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown26(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown27(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(27)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown28(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown29(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(29)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown30(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown31(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown32(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown33(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown34(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown35(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown36(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown37(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(37)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown38(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown39(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(39)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown40(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown41(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown42(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(42)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown43(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(43)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown44(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(44)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILocalManager {
	unsafe fn from_kobject(obj: KObject) -> ILocalManager {
		ILocalManager(Session::from_kobject(obj))
	}
}
