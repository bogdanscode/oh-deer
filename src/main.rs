use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, MixerDeviceSink, source::Source};

fn main() {
    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the sink_handle is dropped.
    let sink_handle = rodio::DeviceSinkBuilder::open_default_sink()
            .expect("open default audio stream");

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("~jbsound.mp3").unwrap());
    // Note that the playback stops when the player is dropped
    let player = rodio::play(&sink_handle.mixer(), file).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}