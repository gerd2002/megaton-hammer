
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INvGemCoreDump(Session);

impl INvGemCoreDump {
	pub fn new() -> Result<Arc<INvGemCoreDump>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INvGemCoreDump>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nvgem:cd") {
			let ret = Arc::new(INvGemCoreDump(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nvgem:cd").map(|s| Arc::new(unsafe { INvGemCoreDump::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for INvGemCoreDump {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INvGemCoreDump {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INvGemCoreDump {
	unsafe fn from_kobject(obj: KObject) -> INvGemCoreDump {
		INvGemCoreDump(Session::from_kobject(obj))
	}
}
