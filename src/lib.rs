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
pub fn speaker_play_array(songs: Vec<&str>) {
    let mut songs_p: Vec<_> = songs.iter()
        .map(|song| convert_string(song))
        .collect();
    unsafe {
        speakerPlayArray(songs_p.as_mut_ptr())
    }
}
pub fn speaker_play_RTTTL(song: &str) {
    unsafe {
        speakerPlayRtttl(convert_string(song))
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
pub fn ime_get(addr: u8) -> Option<i32> {
    let mut v = 0;
    let r = unsafe {
        imeGet(addr, &mut v)
    };
    if r {
        Some(v)
    } else {
        None
    }
}
pub fn ime_get_velocity(addr: u8) -> Option<i32> {
    let mut v = 0;
    let r = unsafe {
        imeGetVelocity(addr, &mut v)
    };
    if r {
        Some(v)
    } else {
        None
    }
}
pub fn ime_reset(addr: u8) -> bool {
    unsafe {
        imeReset(addr)
    }
}
pub fn ime_shutdown() {
    unsafe {
        imeShutdown()
    }
}
pub fn gyro_get(gyro: Gyro) -> i32 {
    unsafe {
        gyroGet(gyro)
    }
}
pub fn gyro_init(port: u8, mult: u16) -> Gyro {
    unsafe {
        gyroInit(port, mult)
    }
}
pub fn gyro_reset(gyro: Gyro) {
    unsafe {
        gyroReset(gyro)
    }
}
pub fn gyro_shutdown(gyro: Gyro) {
    unsafe {
        gyroShutdown(gyro)
    }
}
pub fn encoder_get(enc: Encoder) -> i32 {
    unsafe {
        encoderGet(enc)
    }
}
pub fn encoder_init(port_top: u8, port_bot: u8, reverse: bool) -> Encoder {
    unsafe {
        encoderInit(port_top, port_bot, reverse)
    }
}
pub fn encoder_reset(enc: Encoder) {
    unsafe {
        encoderReset(enc)
    }
}
pub fn encoder_shutdown(enc: Encoder) {
    unsafe {
        encoderShutdown(enc)
    }
}
pub fn ultrasonic_get(ult: Ultrasonic) -> i32 {
    unsafe {
        ultrasonicGet(ult)
    }
}
pub fn ultrasonic_init(port_echo: u8, port_ping: u8) -> Ultrasonic {
    unsafe {
        ultrasonicInit(port_echo, port_ping)
    }
}
pub fn ultrasonic_shutdown(ult: Ultrasonic) {
    unsafe {
        ultrasonicShutdown(ult)
    }
}
pub fn i2c_read(addr: u8, count: u16) -> Option<Vec<u8>> {
    let mut data: Vec<u8> = Default::default();
    let r = unsafe {
        i2cRead(addr, data.as_mut_ptr(), count)
    };
    if r {
        Some(data)
    } else {
        None
    }
}
pub fn i2c_read_register(addr: u8, reg: u8, count: u16) -> Option<Vec<u8>> {
    let mut data: Vec<u8> = Default::default();
    let r = unsafe {
        i2cReadRegister(addr, reg, data.as_mut_ptr(), count)
    };
    if r {
        Some(data)
    } else {
        None
    }
}
pub fn i2c_write(addr: u8, mut data: Vec<u8>, count: u16) -> bool {
    unsafe {
        i2cWrite(addr, data.as_mut_ptr(), count)
    }
}
pub fn i2c_write_register(addr: u8, reg: u8, val: u16) -> bool {
    unsafe {
        i2cWriteRegister(addr, reg, val)
    }
}
pub fn usart_init(mut usart: FILE, baud: u32, flags: u32) {
    unsafe {
        usartInit(&mut usart, baud, flags)
    }
}
pub fn usart_shutdown(mut usart: FILE) {
    unsafe {
        usartShutdown(&mut usart)
    }
}
pub fn f_close(mut stream: FILE) {
    unsafe {
        fclose(&mut stream)
    }
}
pub fn f_count(mut stream: FILE) -> i32 {
    unsafe {
        fcount(&mut stream)
    }
}
pub fn f_delete(file: &str) -> bool {
    let r = unsafe {
        fdelete(convert_string(file))
    };
    r == 0
}
pub fn f_eof(mut stream: FILE) -> bool {
    let r = unsafe {
        feof(&mut stream)
    };
    r != 0
}
pub fn f_flush(mut stream: FILE) -> bool {
    let r = unsafe {
        fflush(&mut stream)
    };
    r == 0
}
pub fn f_getc(mut stream: FILE) -> Option<u8> {
    let r = unsafe {
        fgetc(&mut stream)
    };
    if r >= 0 {
        Some(r as u8)
    } else {
        None
    }
}
pub fn f_gets<'s>(len: i32, mut stream: FILE) -> &'s str {
    let string: &'s str = Default::default();
    unsafe {
        fgets(string.as_ptr() as *mut i8, len, &mut stream)
    };
    string
}
pub fn f_open(file: &str, mode: &str) -> *mut FILE {
    unsafe {
        fopen(convert_string(file), convert_string(mode))
    }
}
pub fn f_print(string: &str, mut stream: FILE) {
    unsafe {
        fprint(convert_string(string), &mut stream)
    }
}
pub fn f_putc(val: u8, mut stream: FILE) -> u8 {
    let r = unsafe {
        fputc(val as i32, &mut stream)
    };
    r as u8
}
pub fn f_puts(string: &str, mut stream: FILE) -> i32 {
    unsafe {
        fputs(convert_string(string), &mut stream)
    } 
}
pub fn f_read<'s>(count: usize, mut stream: FILE) -> (&'s str, usize) {
    let string: &'s str = Default::default();
    let r = unsafe {
        fread(string.as_ptr() as *mut std::os::raw::c_void, 1, count, &mut stream)
    };
    (string, r)
}
pub fn f_seek(mut stream: FILE, offset: i64, origin: i32) -> bool {
    let r = unsafe {
        fseek(&mut stream, offset, origin)
    };
    r == 0
}
pub fn f_tell(mut stream: FILE) -> i64 {
    unsafe {
        ftell(&mut stream)
    }
}
pub fn f_write(string: &str, mut stream: FILE) -> usize {
    unsafe {
        fwrite(convert_string(string) as *mut std::os::raw::c_void, 1, string.len(), &mut stream)
    }
}
pub fn get_char() -> Option<u8> {
    let r = unsafe {
        getchar()
    };
    if r >= 0 {
        Some(r as u8)
    } else {
        None
    }
}
pub fn prints(string: &str) {
    unsafe {
        print(convert_string(string))
    }
}
pub fn put_char(val: u8) -> u8 {
    let r = unsafe {
        putchar(val as i32)
    };
    r as u8
}
pub fn putss(string: &str) -> i32 {
    unsafe {
        puts(convert_string(string))
    }
}
macro_rules! f_printf(
    ($stream:expr, $fmt:expr $(, $arg:expr)*) => {
        unsafe { fprintf(mut $stream, convert_string($fmt) $(,$arg)*) }
    }
);

