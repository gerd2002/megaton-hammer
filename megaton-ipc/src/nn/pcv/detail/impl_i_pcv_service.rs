
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IPcvService(Session);

impl IPcvService {
	pub fn new() -> Result<Arc<IPcvService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IPcvService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"pcv\0\0\0\0\0") {
			let ret = Arc::new(IPcvService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"pcv\0\0\0\0\0").map(|s| Arc::new(unsafe { IPcvService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IPcvService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPcvService {
	pub fn set_power_enabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_clock_enabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_clock_rate(&self, unk0: i32, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u32,
		}
		let req = Request::new(2)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_clock_rate(&self, unk0: i32) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_state(&self, unk0: i32) -> Result<::nn::pcv::ModuleState> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let res : Response<::nn::pcv::ModuleState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_possible_clock_rates(&self, unk0: i32, unk1: i32, unk4: &mut [u32]) -> Result<(i32, i32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
		}
		let req = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk4, 0xa))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: i32,
			unk3: i32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn set_min_v_clock_rate(&self, unk0: i32, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u32,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_reset(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(7)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_voltage_enabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_voltage_enabled(&self, unk0: i32) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_voltage_range(&self, unk0: i32) -> Result<(i32, i32, i32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i32,
			unk2: i32,
			unk3: i32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn set_voltage_value(&self, unk0: i32, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
		}
		let req = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_voltage_value(&self, unk0: i32) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_temperature_thresholds(&self, unk0: i32, unk2: &mut [::nn::pcv::TemperatureThreshold]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_temperature(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn initialize(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_initialized(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn finalize(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn power_on(&self, unk0: ::nn::pcv::PowerControlTarget, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::pcv::PowerControlTarget,
			unk1: i32,
		}
		let req = Request::new(18)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn power_off(&self, unk0: ::nn::pcv::PowerControlTarget) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_voltage(&self, unk0: ::nn::pcv::PowerControlTarget, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::pcv::PowerControlTarget,
			unk1: i32,
		}
		let req = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_power_clock_info_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_oscillator_clock(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_dvfs_table(&self, unk0: i32, unk1: i32, unk3: &mut [u32], unk4: &mut [i32]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
		}
		let req = Request::new(23)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk3, 0xa))
			.descriptor(IPCBuffer::from_mut_slice(unk4, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_module_state_table(&self, unk0: i32, unk2: &mut [::nn::pcv::ModuleState]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(24)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_power_domain_state_table(&self, unk0: i32, unk2: &mut [::nn::pcv::PowerDomainState]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_fuse_info(&self, unk0: i32, unk2: &mut [u32]) -> Result<i32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(26)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IPcvService {
	unsafe fn from_kobject(obj: KObject) -> IPcvService {
		IPcvService(Session::from_kobject(obj))
	}
}
