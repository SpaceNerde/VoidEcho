use std::{fs::File, time::Duration};
use std::io::BufReader;
use rodio::{source::SineWave, Decoder, OutputStream, Sink, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let file = BufReader::new(File::open("examples\\sample_1.flac").unwrap());    
    let source = Decoder::new(file).unwrap();
    
    sink.append(source);

    sink.sleep_until_end();
}
