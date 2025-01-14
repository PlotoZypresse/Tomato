use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;

pub const POMODORO_FINISH: &[u8] = include_bytes!("./sounds/pomodoroFinish.mp3");
pub const BREAK_FINISH: &[u8] = include_bytes!("./sounds/breakDone.mp3");

pub fn play_sound(sound_data: Vec<u8>, duration: u64) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor).unwrap();

    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(duration));
}
