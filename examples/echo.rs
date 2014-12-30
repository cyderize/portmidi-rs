extern crate portmidi;
extern crate "portmidi-sys" as ffi;

use portmidi::{PortMidi, MidiEvent};

fn main() {
	let pm = PortMidi::new().unwrap();
	println!("In: {}", pm.device_info(pm.default_input()));
	println!("Out: {}", pm.device_info(pm.default_output()));
	let input = pm.open_input(pm.default_input(), 4).unwrap();
	let output = pm.open_output(pm.default_output(), 4, 25).unwrap();
	println!("");
	loop {
		if !input.poll().unwrap() { continue; }
		let mut buffer = Vec::from_elem(4, MidiEvent::new());
		let read = input.read(buffer.as_mut_slice()).unwrap();
		for (_, event) in range(0, read).zip(buffer.iter()) {
			println!("Event: {}", (*event).message);
		}
		output.write(buffer.slice_to_or_fail(&(read as uint))).unwrap();
	}
}