//! Structs for dealing with MIDI events and messages

/// The timestamp type
pub type TimeStamp = i32;

/// The key (note) ranging from 0-127
pub type Key = u8;
/// The velocity (0-127)
pub type Velocity = u8;
/// The channel (0-15)
pub type Channel = u8;

/// Represents a MIDI Event
#[deriving(Copy, Clone, Show)]
pub struct MidiEvent {
	/// The message associated with this event
	pub message: MidiMessage,
	/// THe timestamp of the event
	pub timestamp: TimeStamp,
}

impl MidiEvent {
	/// Create a new MIDI event
	pub fn new() -> MidiEvent {
		MidiEvent {
			message: MidiMessage::new(0),
			timestamp: 0,
		}
	}
}

/// Represents a MIDI message
#[deriving(Copy, Clone, Show)]
pub enum MidiMessage {
	/// A note off message
	NoteOff(Channel, Key, Velocity),
	/// A note on message
	NoteOn(Channel, Key, Velocity),
	/// An unknown message
	Unknown(i32)
}

impl MidiMessage {
	/// Create a MidiMessage from a 4-byte word.
	///
	/// The first 3 bytes contain the MIDI data, and the last is unused.
	pub fn new(word: i32) -> MidiMessage {
		let left_nibble = (word >> 28) as u8 & 0xF;
		let right_nibble = (word >> 24) as u8 & 0xF;
		
		let key = (word >> 16) as u8 & 0x7F;
		let velocity = (word >> 8) as u8 & 0x7F;
		
		match left_nibble {
			0x8 => MidiMessage::NoteOff(right_nibble, key, velocity),
			0x9 => MidiMessage::NoteOn(right_nibble, key, velocity),
			_ => MidiMessage::Unknown(word)
		}
	}
	/// Return a 4-bit word from this MidiMessage
	///
	/// The first 3 bytes contain the MIDI data, and the last is unused.
	pub fn to_i32(self) -> i32 {
		match self {
			MidiMessage::NoteOff(channel, key, velocity) => {
				-2147483648 |
				(channel as i32 << 24) |
				(key as i32 << 16) |
				(velocity as i32 << 8)
			},
			MidiMessage::NoteOn(channel, key, velocity) => {
				-1879048192 |
				(channel as i32 << 24) |
				(key as i32 << 16) |
				(velocity as i32 << 8)
			},
			MidiMessage::Unknown(word) => word,
		}
	}
}