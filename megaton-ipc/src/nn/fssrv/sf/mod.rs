mod impl_IFile;
pub use self::impl_IFile::*;
mod impl_IEventNotifier;
pub use self::impl_IEventNotifier::*;
mod impl_IProgramRegistry;
pub use self::impl_IProgramRegistry::*;
mod impl_IStorage;
pub use self::impl_IStorage::*;
mod impl_ISaveDataInfoReader;
pub use self::impl_ISaveDataInfoReader::*;
mod impl_IDirectory;
pub use self::impl_IDirectory::*;
mod impl_IFileSystemProxyForLoader;
pub use self::impl_IFileSystemProxyForLoader::*;
mod impl_IDeviceOperator;
pub use self::impl_IDeviceOperator::*;
mod impl_IFileSystemProxy;
pub use self::impl_IFileSystemProxy::*;
mod impl_IFileSystem;
pub use self::impl_IFileSystem::*;
pub type SaveStruct = [u8; 0x40];
pub type SaveCreateStruct = [u8; 0x40];
pub type IDirectoryEntry = [u8; 0x310];
pub type Partition = u32;
