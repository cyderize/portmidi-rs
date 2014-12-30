use ffi;
use std::c_str::CString;
use std::num::Int;
use libc::c_char;
use super::message::{MidiEvent, MidiMessage};
use super::result::{PortMidiResult, PortMidiError};

pub fn to_pm_event(event: MidiEvent) -> ffi::PmEvent {
	ffi::PmEvent {
		message: Int::to_be(event.message.to_i32()),
		timestamp: event.timestamp
	}
}

pub fn to_event(event: ffi::PmEvent) -> MidiEvent {
	MidiEvent {
		message: MidiMessage::new(Int::from_be(event.message)),
		timestamp: event.timestamp,
	}
}

pub fn to_result(stream: *const ffi::PortMidiStream, error: ffi::PmError) -> PortMidiResult<()> {
	match error {
		ffi::PmError::PmNoError => Ok(()),
		ffi::PmError::PmGotData => Ok(()),
		ffi::PmError::PmHostError => {
			Err(PortMidiError::HostError(
				unsafe {
					if ffi::Pm_HasHostError(stream) != 0 {
						let chars = [1u8, ..256];
						let mut c_str = chars.to_c_str();
						let ptr = c_str.as_mut_ptr();
						ffi::Pm_GetHostErrorText(ptr, 256);
						c_str.to_string()
					}
					else { "".to_string() }
				}
			))
		}
		ffi::PmError::PmInvalidDeviceId => Err(PortMidiError::InvalidDeviceId),
		ffi::PmError::PmInsufficientMemory => Err(PortMidiError::InsufficientMemory),
		ffi::PmError::PmBufferTooSmall => Err(PortMidiError::BufferTooSmall),
		ffi::PmError::PmBufferOverflow => Err(PortMidiError::BufferOverflow),
		ffi::PmError::PmBadPtr =>  Err(PortMidiError::BadPtr),
		ffi::PmError::PmBadData =>  Err(PortMidiError::BadData),
		ffi::PmError::PmInternalError => Err(PortMidiError::InternalError),
		ffi::PmError::PmBufferMaxSize => Err(PortMidiError::BufferMaxSize),
	}
}

pub fn to_string(buf: *const c_char) -> String {
	unsafe {
		CString::new(buf, false).to_string()
	}
}