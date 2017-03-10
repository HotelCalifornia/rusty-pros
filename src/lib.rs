#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/pros.rs"));
fn convert_string(s: &str) -> *const ::std::os::raw::c_char {
    CString::new(s).unwrap().as_ptr()
}


pub fn is_autonomous() -> bool {
    unsafe {
        isAutonomous()
    }
}
pub fn is_enabled() -> bool {
    unsafe {
        isEnabled()
    }
}
pub fn is_joystick_connected(joystick: u8) -> bool {
    unsafe {
        isJoystickConnected(joystick)
    }
}
pub fn is_online() -> bool {
    unsafe {
        isOnline()
    }
}
pub fn joystick_get_digital(joystick: u8, btn_group: u8, btn: u8) -> bool {
    unsafe {
        joystickGetDigital(joystick, btn_group, btn)
    }
}
pub fn power_level_backup() -> u32 {
    unsafe {
        powerLevelBackup()
    }
}
pub fn power_level_main() -> u32 {
    unsafe {
        powerLevelMain()
    }
}
pub fn set_team_name(name: &str) {
    unsafe {
        setTeamName(convert_string(name))
    }
}
pub fn analog_calibrate(channel: u8) -> i32 {
    unsafe {
        analogCalibrate(channel)
    }
}
pub fn analog_read(channel: u8) -> i32 {
    unsafe {
        analogRead(channel)
    }
}
pub fn analog_read_calibrated(channel: u8) -> i32 {
    unsafe {
        analogReadCalibrated(channel)
    }
}
pub fn analog_read_calibrated_HR(channel: u8) -> i32 {
    unsafe {
        analogReadCalibratedHR(channel)
    }
}
pub fn digital_read(pin: u8) -> bool {
    unsafe {
        digitalRead(pin)
    }
}
pub fn digital_write(pin: u8, value: bool) {
    unsafe {
        digitalWrite(pin, value)
    }
}
pub fn io_clear_interrupt(pin: u8) {
    unsafe {
        ioClearInterrupt(pin)
    }
}
pub fn io_set_interrupt(pin: u8, edges: u8, handler: InterruptHandler) {
    unsafe {
        ioSetInterrupt(pin, edges, handler)
    }
}
pub fn motor_get(channel: u8) -> i32 {
    unsafe {
        motorGet(channel)
    }
}
pub fn motor_set(channel: u8, speed: i32) {
    unsafe {
        motorSet(channel, speed)
    }
}
pub fn motor_stop(channel: u8) {
    unsafe {
        motorStop(channel)
    }
}
pub fn motor_stop_all() {
    unsafe {
        motorStopAll()
    }
}
pub fn speaker_init() {
    unsafe {
        speakerInit()
    }
}
// pub fn speaker_play_array(songs: &[&str, 3]) {
    // TODO: fuckkkk
// }
pub fn speaker_play_RTTTL(song: &str) {
    unsafe {
        speakerPlayRTTTL(convert_string(song))
    }
}
pub fn speaker_shutdown() {
    unsafe {
        speakerShutdown()
    }
}
pub fn ime_initialize_all() -> u32 {
    unsafe {
        imeInitializeAll()
    }
}
//pub fn ime_get(addr: u8, )
