use ffi;
use super::message::{MidiMessage, MidiEvent, TimeStamp};
use super::result::PortMidiResult;
use super::util::{to_result, to_pm_event};

pub fn new_output_device(stream: *const ffi::PortMidiStream) -> OutputDevice {
	OutputDevice {
		stream: stream
	}
}

/// Represents a MIDI output device.
pub struct OutputDevice {
	stream: *const ffi::PortMidiStream
}

impl OutputDevice {
	/// Write these MIDI events to the output device.
	pub fn write(&self, buffer: &[MidiEvent]) -> PortMidiResult<()> {
		let mut fill = Vec::from_elem(buffer.len(), ffi::PmEvent { message: 0, timestamp: 0 });
		for (event_in, event_out) in buffer.iter().zip(fill.iter_mut()) {
			*event_out = to_pm_event(*event_in);
		}
		unsafe { try!(to_result(self.stream, ffi::Pm_Write(self.stream, fill.as_ptr(), fill.len() as i32))); }
		Ok(())
	}
	/// Write this message to the output device.
	pub fn write_short(&self, time: TimeStamp, msg: MidiMessage) -> PortMidiResult<()> {
		unsafe { to_result(self.stream, ffi::Pm_WriteShort(self.stream, time, msg.to_i32())) }
	}
	/// Write this system exclusive buffer to the output device.
	pub fn write_sysex(&self, time: TimeStamp, msg: &[u8]) -> PortMidiResult<()> {
		let buffer = msg.to_c_str();
		let ptr = buffer.as_ptr();
		unsafe { to_result(self.stream, ffi::Pm_WriteSysEx(self.stream, time, ptr)) }
	}
	/// Abort writing.
	///
	/// Consumes the device.
	pub fn abort(self) -> PortMidiResult<()> {
		unsafe { to_result(self.stream, ffi::Pm_Abort(self.stream)) }
	}
}

impl Drop for OutputDevice {
	/// Free all resources associated with this output device.
	///
	/// Closes the underlying stream
	fn drop(&mut self) {
		unsafe { let _ = ffi::Pm_Close(self.stream); }
	}
}