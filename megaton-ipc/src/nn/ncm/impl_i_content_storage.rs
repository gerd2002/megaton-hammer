
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IContentStorage(Session);

impl AsRef<Session> for IContentStorage {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IContentStorage {
	pub fn generate_place_holder_id(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn create_placeholder_entry_and_registered_directory_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_placeholder_entry(&self, unk0: u128) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn does_placeholder_entry_exist(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_placeholder_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn move_placeholder_to_registered(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_registered_entry(&self, unk0: u128) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn does_registered_entry_exist(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_placeholder_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clean_placeholder_directory(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_number_of_placholder_entries(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_number_of_registered_entries(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_registered_entries(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_registered_entry_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn close_and_flush_storage(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_placeholder_entry_registered_entry_and_registered_directory_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_placeholder_entry_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_registered_entry_raw(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_placeholder_entry_rights_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_registered_entry_rights_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_registered_path_for_debug(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_free_space(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_total_space(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn flush_storage(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(24)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IContentStorage {
	unsafe fn from_kobject(obj: KObject) -> IContentStorage {
		IContentStorage(Session::from_kobject(obj))
	}
}
