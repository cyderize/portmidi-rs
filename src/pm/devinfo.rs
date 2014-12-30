//! Structs for dealing with MIDI device information

/// Represents the ID number for a particular device
pub type DeviceId = int;

/// The information associated with a particular device
#[deriving(Clone, Show)]
pub struct DeviceInfo {
	/// The MIDI interface type used
	pub interface: String,
	/// The name of the device
	pub name: String,
	/// The kind of device
	pub kind: DeviceType,
}

/// Represents the kind of device
#[deriving(Copy, Clone, Show)]
pub enum DeviceType {
	/// An input device
	Input,
	/// An output device
	Output,
	/// An unknown device
	Undefined
}