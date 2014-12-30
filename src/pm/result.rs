//! Common structs for the result and error types

use std::error::Error;
use std::result::Result;

/// The result type used in portmidi-rs
pub type PortMidiResult<T> = Result<T, PortMidiError>;

/// A PortMidi error
#[deriving(Clone, Show)]
pub enum PortMidiError {
	HostError(String),
	InvalidDeviceId,
	InsufficientMemory,
	BufferTooSmall,
	BufferOverflow,
	BadPtr,
	BadData,
	InternalError,
	BufferMaxSize
}

impl Error for PortMidiError {
    fn description(&self) -> &str {
		match *self {
            PortMidiError::HostError(_) => "Host error",
            PortMidiError::InvalidDeviceId => "Invalid device ID",
			PortMidiError::InsufficientMemory => "Insufficient memory",
			PortMidiError::BufferTooSmall => "Buffer too small",
			PortMidiError::BufferOverflow => "Buffer overflow",
			PortMidiError::BadPtr => "Bad pointer",
			PortMidiError::BadData => "Bad data",
			PortMidiError::InternalError => "Internal error",
			PortMidiError::BufferMaxSize => "Maximum buffer size reached",
        }
	}

    fn detail(&self) -> Option<String> { None }
    fn cause(&self) -> Option<&Error> { None }
}