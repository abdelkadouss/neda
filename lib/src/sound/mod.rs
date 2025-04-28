use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct Adhan {
    pub file: String,
}

impl Adhan {
    pub fn new(file: String) -> Self {
        Self { file }
    }

    pub fn play(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.set_volume(1_f32);

        let file = BufReader::new(File::open(&self.file).unwrap());
        let source = Decoder::new(file).unwrap();

        sink.append(source);
        sink.sleep_until_end();
    }
}
