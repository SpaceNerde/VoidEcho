use eframe::egui;
use std::path::{Path, PathBuf};
use std::{fs::File, time::Duration};
use std::io::BufReader;
use rodio::{source::SineWave, Decoder, OutputStream, Sink, Source};

fn main() -> Result<(), eframe::Error>{
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Void Echo",
        options,
        Box::new(|cc| {
            Box::new(MusicApp::new(sink))
        }),
    )
}

struct MusicApp {
    sink: Sink,
    current_song_path: Option<PathBuf>,
}

impl MusicApp {
    fn new(sink: Sink) -> Self {
        Self {
            sink,
            current_song_path: None,
        }
    }

    fn load_file(&mut self, file_path: &PathBuf) {
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
        self.current_song_path = Some(file_path.to_path_buf());
    }
}

impl eframe::App for MusicApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle drag-and-drop
            if ctx.input(|i| i.raw.dropped_files.len() > 0) {
                let file_path = ctx.input(|i| i.raw.dropped_files[0].clone());
                self.load_file(&file_path.path.unwrap());
            }

            if ui.button("Load").clicked() {
                let mut default_file_path: PathBuf = PathBuf::new();
                default_file_path.push("examples\\sample_1.flac");
                self.load_file(&default_file_path);
            }

            if let Some(path) = &self.current_song_path {
                ui.label(format!("Current Song: {:?}", path.file_name().unwrap()));
            }

            if ui.button("Play").clicked() {
                self.sink.play();
            }
            if ui.button("Pause").clicked() {
                self.sink.pause();
            }
        });
    }
}