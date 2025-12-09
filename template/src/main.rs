use std::{
    io::{Cursor, Write, stdout},
    thread::sleep,
    time::{Duration, Instant},
};

use ascii_linker::embed_full;
use rodio::{Decoder, OutputStreamBuilder, Sink, Source};

const FILE: (&[&[u8]], &[u8], u64) = embed_full!("TEMPLATE_PWD");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (frames, audio, frametime) = FILE;
    let frametime = Duration::from_micros(frametime);

    let output_stream = OutputStreamBuilder::open_default_stream()?;
    let decoder = Decoder::new_mp3(Cursor::new(audio))?;
    let total = decoder.total_duration().unwrap();

    let source = decoder.track_position();

    let sink = Sink::connect_new(output_stream.mixer());

    sink.append(source);
    sink.play();

    let mut lock = stdout().lock();

    #[cfg(windows)]
    enable_virtual_terminal_processing();

    let len = frames.len();

    let mut counter = 0;

    lock.write_all(b"\r\x1b[2J\r\x1b[H")?;
    lock.write_all(b"\x1b[?25l")?;

    while counter < len {
        let task_time = Instant::now();
        let decompressed_frame = zstd::decode_all(frames[counter])?;

        lock.write_all(b"\r\x1b[H")?;
        lock.write_all(&decompressed_frame)?;
        lock.flush()?;

        if counter.is_multiple_of(15) {
            counter = get_pos(len, &sink, total);
        } else {
            counter += 1;
        }

        let elapsed = task_time.elapsed();
        if elapsed < frametime {
            sleep(frametime - elapsed);
        }
    }
    // unhide cursor
    lock.write_all(b"\x1b[?25h")?;

    Ok(())
}

fn get_pos(len: usize, sink: &Sink, total: Duration) -> usize {
    (sink.get_pos().div_duration_f64(total) * len as f64).round() as usize
}

#[cfg(windows)]
fn enable_virtual_terminal_processing() {
    use winapi::um::consoleapi::GetConsoleMode;
    use winapi::um::consoleapi::SetConsoleMode;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle != INVALID_HANDLE_VALUE {
            let mut mode = 0;
            if GetConsoleMode(handle, &mut mode) != 0 {
                SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
            }
        }
    }
}
