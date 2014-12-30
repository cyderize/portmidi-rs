//! A library for interfacing with MIDI devices.

extern crate "portmidi-sys" as ffi;
extern crate libc;

use std::ptr;
use self::pm::result::PortMidiResult;
use self::pm::devinfo::DeviceId;
use self::pm::devinfo::{DeviceInfo, DeviceType};
use self::pm::input::{InputDevice, new_input_device};
use self::pm::output::{OutputDevice, new_output_device};
use self::pm::util::{to_result, to_string};

pub use self::pm::devinfo;
pub use self::pm::message;
pub use self::pm::result;

pub use self::pm::message::{MidiMessage, MidiEvent};

/// Structs for dealing with MIDI input.
pub mod input {
	/// Flags for use with ```InputDevice.set_filter()```.
	pub use ffi::filters;
	/// Flags for use with ```InputDevice.set_channel_mask()```.
	pub use ffi::channel;
	
	pub use super::pm::input::InputDevice;
}

/// Structs for dealing with MIDI output
pub mod output {
	pub use super::pm::output::OutputDevice;
}

mod pm;

/// The main PortMidi object.
///
/// Only one PortMidi object should be instantiated in a program.
pub struct PortMidi;

impl PortMidi {
	/// Initialize the PortMidi interface
	pub fn new() -> PortMidiResult<PortMidi> {
		unsafe {
			try!(to_result(ptr::null(), ffi::Pm_Initialize()));
			Ok(PortMidi)
		}
	}
	/// Returns the number of MIDI devices
	pub fn count(&self) -> uint {
		unsafe { ffi::Pm_CountDevices() as uint }
	}
	/// Returns the ID of the default input device
	pub fn default_input(&self) -> DeviceId {
		unsafe { ffi::Pm_GetDefaultInputDeviceID() as DeviceId }
	}
	/// Returns the ID of the default output device
	pub fn default_output(&self) -> DeviceId {
		unsafe { ffi::Pm_GetDefaultOutputDeviceID() as DeviceId }
	}
	/// Returns the information associated with the device which has the given ID
	pub fn device_info(&self, id: DeviceId) -> DeviceInfo {
		unsafe {
			let info = ffi::Pm_GetDeviceInfo(id as ffi::PmDeviceID);
			DeviceInfo {
				interface: to_string((*info).interf),
				name: to_string((*info).name),
				kind:
					if (*info).input != 0 {
						DeviceType::Input
					}
					else if (*info).output != 0 {
						DeviceType::Output
					}
					else {
						DeviceType::Undefined
					}
			}
		}
	}
	/// Open the input device with the specified ID, ready to recieve MIDI data
	pub fn open_input(&self, id: DeviceId, buffer: i32) -> PortMidiResult<InputDevice> {
		unsafe { 
			let mut stream = ptr::null();
			try!(to_result(stream, ffi::Pm_OpenInput(&mut stream, id as ffi::PmDeviceID, ptr::null(), buffer, ptr::null(), ptr::null()))); 
			Ok(new_input_device(stream))
		}
	}/// Open the output device with the specified ID, ready to send MIDI data
	pub fn open_output(&self, id: DeviceId, buffer: i32, latency: i32) -> PortMidiResult<OutputDevice> {
		unsafe {
			let mut stream = ptr::null();
			try!(to_result(stream, ffi::Pm_OpenOutput(&mut stream, id as ffi::PmDeviceID, ptr::null(), buffer, ptr::null(), ptr::null(), latency))); 
			Ok(new_output_device(stream))
		}
	}
}

impl Drop for PortMidi {
	/// Free all resources associated with the PortMidi object
	fn drop(&mut self) {
		unsafe { let _ = ffi::Pm_Terminate(); }
	}
}
