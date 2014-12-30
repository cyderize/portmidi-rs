//#![no_std]
extern crate libc;

use libc::{c_char, c_void, c_int, c_uint};

pub mod filters {
	bitflags! {
		#[repr(C)] flags PmFilters: u32 {
			const PM_FILT_ACTIVE = (1 << 0x0E),
			const PM_FILT_SYSEX = (1 << 0x00),
			const PM_FILT_CLOCK = (1 << 0x08),
			const PM_FILT_PLAY = ((1 << 0x0A) | (1 << 0x0C) | (1 << 0x0B)),
			const PM_FILT_TICK = (1 << 0x09),
			const PM_FILT_FD = (1 << 0x0D),
			const PM_FILT_UNDEFINED = PM_FILT_FD.bits,
			const PM_FILT_RESET = (1 << 0x0F),
			const PM_FILT_REALTIME = (PM_FILT_ACTIVE.bits | PM_FILT_SYSEX.bits | PM_FILT_CLOCK.bits | PM_FILT_PLAY.bits | PM_FILT_UNDEFINED.bits | PM_FILT_RESET.bits | PM_FILT_TICK.bits),
			const PM_FILT_NOTE = ((1 << 0x19) | (1 << 0x18)),
			const PM_FILT_CHANNEL_AFTERTOUCH = (1 << 0x1D),
			const PM_FILT_POLY_AFTERTOUCH = (1 << 0x1A),
			const PM_FILT_AFTERTOUCH = (PM_FILT_CHANNEL_AFTERTOUCH.bits | PM_FILT_POLY_AFTERTOUCH.bits),
			const PM_FILT_PROGRAM = (1 << 0x1C),
			const PM_FILT_CONTROL = (1 << 0x1B),
			const PM_FILT_PITCHBEND = (1 << 0x1E),
			const PM_FILT_MTC = (1 << 0x01),
			const PM_FILT_SONG_POSITION = (1 << 0x02),
			const PM_FILT_SONG_SELECT = (1 << 0x03),
			const PM_FILT_TUNE = (1 << 0x06),
			const PM_FILT_SYSTEMCOMMON = (PM_FILT_MTC.bits | PM_FILT_SONG_POSITION.bits | PM_FILT_SONG_SELECT.bits | PM_FILT_TUNE.bits)
		}
	}
}

pub mod channel {
	use libc::c_int;
	bitflags! {
		#[repr(C)] flags PmChannelMask: c_int {
			const PM_CHANNEL_1 = (1 << 0),
			const PM_CHANNEL_2 = (1 << 1),
			const PM_CHANNEL_3 = (1 << 2),
			const PM_CHANNEL_4 = (1 << 3),
			const PM_CHANNEL_5 = (1 << 4),
			const PM_CHANNEL_6 = (1 << 5),
			const PM_CHANNEL_7 = (1 << 6),
			const PM_CHANNEL_8 = (1 << 7),
			const PM_CHANNEL_9 = (1 << 8),
			const PM_CHANNEL_10 = (1 << 9),
			const PM_CHANNEL_11 = (1 << 10),
			const PM_CHANNEL_12 = (1 << 11),
			const PM_CHANNEL_13 = (1 << 12),
			const PM_CHANNEL_14 = (1 << 13),
			const PM_CHANNEL_15 = (1 << 14),
			const PM_CHANNEL_16 = (1 << 15)
		}
	}
}

pub type PortMidiStream = c_void;
pub type PmMessage = i32;
pub type PmTimestamp = c_int;
pub type PmDeviceID = c_int;

#[repr(C)]
#[deriving(Clone, Copy, Show)]
pub struct PmEvent {
	pub message: PmMessage,
	pub timestamp: PmTimestamp,
}

#[repr(C)]
#[deriving(Clone, Copy, Show, FromPrimitive)]
pub enum PmError {
	PmNoError = 0,
	PmGotData = 1,
	PmHostError = -10000,
	PmInvalidDeviceId = -9999,
	PmInsufficientMemory = -9998,
	PmBufferTooSmall = -9997,
	PmBufferOverflow = -9996,
	PmBadPtr = -9995,
	PmBadData = -9994,
	PmInternalError = -9993,
	PmBufferMaxSize = -9992,
}

#[repr(C)]
pub struct PmDeviceInfo {
	pub struct_version: i32,
	pub interf: *const c_char,
	pub name: *const c_char,
	pub input: i32,
	pub output: i32,
	pub opened: i32,
}

impl Copy for PmDeviceInfo {}

#[link(name = "portmidi")]
extern "C" {
	pub fn Pm_Initialize() -> PmError;
	pub fn Pm_Terminate()-> PmError;
	pub fn Pm_HasHostError(stream: *const PortMidiStream) -> c_uint;
	pub fn Pm_GetErrorText(errorCode: PmError) -> *const c_char;
	pub fn Pm_GetHostErrorText(msg: *mut c_char, len: c_uint);
	pub fn Pm_CountDevices() -> c_uint;
	pub fn Pm_GetDefaultInputDeviceID() -> PmDeviceID;
	pub fn Pm_GetDefaultOutputDeviceID() -> PmDeviceID;
	pub fn Pm_GetDeviceInfo(id: PmDeviceID) -> *const PmDeviceInfo;
	pub fn Pm_OpenInput(stream: *mut *const PortMidiStream, inputDevice: PmDeviceID, inputDriverInfo: *const c_void, bufferSize: i32, time_proc: *const c_void, time_info: *const c_void) -> PmError;
	pub fn Pm_OpenOutput(stream: *mut *const PortMidiStream, outputDevice: PmDeviceID, inputDriverInfo: *const c_void, bufferSize: i32, time_proc: *const c_void, time_info: *const c_void, latency: i32) -> PmError;
	pub fn Pm_SetFilter(stream: *const PortMidiStream, filters: filters::PmFilters) -> PmError;
	pub fn Pm_SetChannelMask(stream: *const PortMidiStream, mask: channel::PmChannelMask) -> PmError;
	pub fn Pm_Abort(stream: *const PortMidiStream) -> PmError;
	pub fn Pm_Close(stream: *const PortMidiStream) -> PmError;
	pub fn Pm_Read(stream: *const PortMidiStream, buffer: *mut PmEvent, length: i32) -> c_int;
	pub fn Pm_Poll(stream: *const PortMidiStream) -> PmError;
	pub fn Pm_Write(stream: *const PortMidiStream, buffer: *const PmEvent, length: i32) -> PmError;
	pub fn Pm_WriteShort(stream: *const PortMidiStream, when: PmTimestamp, message: PmMessage) -> PmError;
	pub fn Pm_WriteSysEx(stream: *const PortMidiStream, when: PmTimestamp, msg: *const c_char) -> PmError;
}
