mod button_state;
mod traits;

pub use traits::MusicBox;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
mod raspberry_pi;

#[cfg(all(target_os = "linux", target_arch = "arm"))]
pub use self::raspberry_pi::RaspberryPI as UI;

#[cfg(target_os = "macos")]
mod mac_os;

#[cfg(target_os = "macos")]
pub use self::mac_os::MacOS as UI;
