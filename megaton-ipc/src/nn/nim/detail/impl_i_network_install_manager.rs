
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct INetworkInstallManager(Session);

impl INetworkInstallManager {
	pub fn new() -> Result<Arc<INetworkInstallManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<INetworkInstallManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"nim\0\0\0\0\0") {
			let ret = Arc::new(INetworkInstallManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"nim\0\0\0\0\0").map(|s| Arc::new(unsafe { INetworkInstallManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for INetworkInstallManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INetworkInstallManager {
	pub fn unknown1(&self, unk0: u128) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INetworkInstallManager {
	unsafe fn from_kobject(obj: KObject) -> INetworkInstallManager {
		INetworkInstallManager(Session::from_kobject(obj))
	}
}
