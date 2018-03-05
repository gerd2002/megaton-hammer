
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IBluetoothDriver(Session);

impl IBluetoothDriver {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Init(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Enable(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Disable(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CleanupAndShutdown(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetAdapterProperties(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetAdapterProperty(&self, ) -> Result<()> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetAdapterProperty(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn StartDiscovery(&self, ) -> Result<()> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CancelDiscovery(&self, ) -> Result<()> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CreateBond(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn RemoveBond(&self, unk0: [u8; 6]) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CancelBond(&self, unk0: [u8; 6]) -> Result<()> {
		let req = Request::new(12)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn PinReply(&self, ) -> Result<()> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SspReply(&self, ) -> Result<()> {
		let req = Request::new(14)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn InitInterfaces(&self, ) -> Result<()> {
		let req = Request::new(16)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_Connect(&self, unk0: [u8; 6]) -> Result<()> {
		let req = Request::new(17)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_Disconnect(&self, unk0: [u8; 6]) -> Result<()> {
		let req = Request::new(18)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_SendData(&self, ) -> Result<()> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_SendData2(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_SetReport(&self, ) -> Result<()> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_GetReport(&self, ) -> Result<()> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_WakeController(&self, unk0: [u8; 6]) -> Result<()> {
		let req = Request::new(23)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_AddPairedDevice(&self, ) -> Result<()> {
		let req = Request::new(24)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_GetPairedDevice(&self, ) -> Result<()> {
		let req = Request::new(25)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_CleanupAndShutdown(&self, ) -> Result<()> {
		let req = Request::new(26)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown27(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ExtInterface_SetTSI(&self, ) -> Result<()> {
		let req = Request::new(28)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_SetBurstMode(&self, ) -> Result<()> {
		let req = Request::new(29)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_SetZeroRetran(&self, ) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_SetMcMode(&self, unk0: u8) -> Result<()> {
		let req = Request::new(31)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_StartLlrMode(&self, ) -> Result<()> {
		let req = Request::new(32)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_ExitLlrMode(&self, ) -> Result<()> {
		let req = Request::new(33)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_SetRadio(&self, unk0: u8) -> Result<()> {
		let req = Request::new(34)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_SetVisibility(&self, ) -> Result<()> {
		let req = Request::new(35)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown36(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown37(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn HidHostInterface_GetLatestPlr(&self, ) -> Result<()> {
		let req = Request::new(38)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ExtInterface_GetPendingConnections(&self, ) -> Result<()> {
		let req = Request::new(39)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn HidHostInterface_GetChannelMap(&self, ) -> Result<()> {
		let req = Request::new(40)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetIsBluetoothBoostEnabled(&self, unk0: u8) -> Result<()> {
		let req = Request::new(41)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetIsBluetoothBoostEnabled(&self, ) -> Result<u8> {
		let req = Request::new(42)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetIsBluetoothAfhEnabled(&self, unk0: u8) -> Result<()> {
		let req = Request::new(43)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetIsBluetoothAfhEnabled(&self, ) -> Result<u8> {
		let req = Request::new(44)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IBluetoothDriver {
	unsafe fn from_kobject(obj: KObject) -> IBluetoothDriver {
		IBluetoothDriver(Session::from_kobject(obj))
	}
}
