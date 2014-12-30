use ffi;
use super::message::MidiEvent;
use super::result::PortMidiResult;
use super::util::{to_result, to_event};

pub use ffi::filters::PmFilters as Filters;
pub use ffi::channel::PmChannelMask as ChannelMask;

pub fn new_input_device(stream: *const ffi::PortMidiStream) -> InputDevice {
	InputDevice {
		stream: stream
	}
}

/// Represents a MIDI input device.
pub struct InputDevice {
	stream: *const ffi::PortMidiStream
}

impl InputDevice {
	/// Reads MIDI events from the input device.
	///
	/// Returns the number of events read if successfull.
	pub fn read(&self, buffer: &mut [MidiEvent]) -> PortMidiResult<int> {
		unsafe {
			let mut fill = Vec::from_elem(buffer.len(), ffi::PmEvent { message: 0, timestamp: 0 });
			let ptr = fill.as_mut_ptr();
			let read = ffi::Pm_Read(self.stream, ptr, buffer.len() as i32);
			if read >= 0 {
				for ((event_in, event_out), _) in fill.iter().zip(buffer.iter_mut()).zip(range(0, read)) {
					*event_out = to_event(*event_in);
				}
				Ok(read as int)
			}
			else {
				let error: ffi::PmError = FromPrimitive::from_int(read as int).unwrap();
				try!(to_result(self.stream, error));
				panic!("Unreachable code reached!");
			}
		}
	}
	/// Whether or not data is available.
	pub fn poll(&self) -> PortMidiResult<bool> {
		unsafe {
			let poll = ffi::Pm_Poll(self.stream);
			match poll {
				ffi::PmError::PmNoError => Ok(false),
				ffi::PmError::PmGotData => Ok(true),
				_ => {
					try!(to_result(self.stream, poll));
					panic!("Unreachable code reached!");
				}
			}
		 }
	}
	/// Filter out the selected message types
	pub fn set_filter(&self, filter: Filters) -> PortMidiResult<()> {
		unsafe { to_result(self.stream, ffi::Pm_SetFilter(self.stream, filter)) }
	}
	/// Listen for messages on the selected channel(s)
	pub fn set_channel_mask(&self, mask: ChannelMask) -> PortMidiResult<()> {
		unsafe { to_result(self.stream, ffi::Pm_SetChannelMask(self.stream, mask)) }
	}
}

impl Drop for InputDevice {
	/// Free all resources associated with this input device.
	///
	/// Closes the underlying stream
	fn drop(&mut self) {
		unsafe { let _ = ffi::Pm_Close(self.stream); }
	}
}