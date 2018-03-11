mod impl_i_applet_accessor;
pub use self::impl_i_applet_accessor::*;
mod impl_i_library_applet_accessor;
pub use self::impl_i_library_applet_accessor::*;
mod impl_i_self_controller;
pub use self::impl_i_self_controller::*;
mod impl_i_window;
pub use self::impl_i_window::*;
mod impl_i_transfer_storage_accessor;
pub use self::impl_i_transfer_storage_accessor::*;
mod impl_i_all_system_applet_proxies_service;
pub use self::impl_i_all_system_applet_proxies_service::*;
mod impl_i_library_applet_creator;
pub use self::impl_i_library_applet_creator::*;
mod impl_i_application_proxy_service;
pub use self::impl_i_application_proxy_service::*;
mod impl_i_system_applet_proxy;
pub use self::impl_i_system_applet_proxy::*;
mod impl_i_application_proxy;
pub use self::impl_i_application_proxy::*;
mod impl_i_lock_accessor;
pub use self::impl_i_lock_accessor::*;
mod impl_i_global_state_controller;
pub use self::impl_i_global_state_controller::*;
mod impl_i_application_functions;
pub use self::impl_i_application_functions::*;
mod impl_i_overlay_functions;
pub use self::impl_i_overlay_functions::*;
mod impl_i_display_controller;
pub use self::impl_i_display_controller::*;
mod impl_i_debug_functions;
pub use self::impl_i_debug_functions::*;
mod impl_i_overlay_applet_proxy;
pub use self::impl_i_overlay_applet_proxy::*;
mod impl_i_audio_controller;
pub use self::impl_i_audio_controller::*;
mod impl_i_window_controller;
pub use self::impl_i_window_controller::*;
mod impl_i_library_applet_proxy;
pub use self::impl_i_library_applet_proxy::*;
mod impl_i_application_accessor;
pub use self::impl_i_application_accessor::*;
mod impl_i_home_menu_functions;
pub use self::impl_i_home_menu_functions::*;
mod impl_i_process_winding_controller;
pub use self::impl_i_process_winding_controller::*;
mod impl_i_storage_accessor;
pub use self::impl_i_storage_accessor::*;
mod impl_i_common_state_getter;
pub use self::impl_i_common_state_getter::*;
mod impl_i_library_applet_self_accessor;
pub use self::impl_i_library_applet_self_accessor::*;
mod impl_i_storage;
pub use self::impl_i_storage::*;
mod impl_i_application_creator;
pub use self::impl_i_application_creator::*;
pub type AppletIdentityInfo = u128;
pub type AppletProcessLaunchReason = u32;
pub type LibraryAppletInfo = u64;
pub type WindowCreationOption = u32;
pub type AppletKind = u64;
pub type EmulatedButtonEvent = u32;
