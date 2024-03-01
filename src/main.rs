use eframe::egui;
use egui::ProgressBar;
use std::borrow::Borrow;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Instant, SystemTime};
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
    start_time: Option<Instant>,
    pause_time: Option<Instant>,
    pause_duration: Duration,
    total_duration: Option<f32>,
}

impl MusicApp {
    fn new(sink: Sink) -> Self {
        Self {
            sink,
            current_song_path: None,
            start_time: None,
            pause_time: None,
            pause_duration: Duration::from_secs_f32(0.),
            total_duration: None,
        }
    }

    fn load_file(&mut self, file_path: &PathBuf) {
        // load the file provided and convert it into a source
        let file = BufReader::new(File::open(file_path).unwrap());
        let source = Decoder::new(file).unwrap();

        // reset old times
        // this is the most disgusting thing i've ever done!
        self.start_time = None;
        self.pause_time = None;
        self.pause_duration = Duration::from_secs_f32(0.);

        // get times
        self.total_duration = Some(source.total_duration().unwrap_or_default().as_secs_f32());
        self.start_time = Some(Instant::now());

        // do weird sink stuff
        self.sink.append(source);
        
        // get the path to song
        self.current_song_path = Some(file_path.to_path_buf());
    }

    // credit to https://github.com/RustAudio/rodio/issues/272 (slmjkdbtl)
    fn time(&self) -> Duration {
        return match self.pause_time {
            Some(time) => self.start_time.unwrap().elapsed() - time.elapsed() - self.pause_duration,
            None => self.start_time.unwrap().elapsed() - self.pause_duration,
        };
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

            // Handle manuel load
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
                if let Some(time) = self.pause_time.take() {
                    self.pause_duration = self.pause_duration.add(time.elapsed());
                }
            }
            if ui.button("Pause").clicked() {
                self.sink.pause();
                self.pause_time = Some(Instant::now());
            }
            
            if !(self.sink.empty()) {
                let progress_bar = ProgressBar::new(
                    self.time().as_secs_f32() / self.total_duration.unwrap()
                ).show_percentage();
                ui.add(progress_bar);
            }
            ctx.request_repaint();
        });
    }
}