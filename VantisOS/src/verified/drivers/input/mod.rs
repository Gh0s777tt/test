// Input Drivers Module - VantisOS
//
// This module provides input device drivers for VantisOS, including:
// - PS/2 mouse driver
// - USB HID driver
// - Touchscreen driver
// - Input event system

pub mod ps2_mouse;
pub mod usb_hid;
pub mod touchscreen;
pub mod input_event;

pub use ps2_mouse::{Ps2MouseDriver, Ps2MousePacket};
pub use usb_hid::{UsbHidDriver, UsbHidDevice, HidReport};
pub use touchscreen::{TouchscreenDriver, TouchEvent, TouchPoint};
pub use input_event::{InputEventManager, InputEvent, InputDevice, InputEventQueue};

/// Input drivers initialization
pub fn init() {
    ps2_mouse::init();
    usb_hid::init();
    touchscreen::init();
    input_event::init();
}