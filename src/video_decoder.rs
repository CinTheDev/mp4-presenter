use std::sync::mpsc;
use std::thread;

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;

const IMAGE_BUFFER_SIZE: usize = 1024;

pub struct VideoDecoder {
    _decoder_thread: thread::JoinHandle<()>,
    decoder_rx: mpsc::Receiver<Video>,
}

impl VideoDecoder {
    pub fn new(path: &str) -> Result<Self, ffmpeg_next::Error> {
        let (tx, rx) = mpsc::sync_channel(IMAGE_BUFFER_SIZE);

        let path = path.to_owned();
        let _decoder_thread = thread::spawn(move || {
            VideoDecoder::start_decoding(&path, tx).unwrap();
        });

        Ok(Self {
            _decoder_thread,
            decoder_rx: rx,
        })
    }

    fn start_decoding(path: &str, tx: mpsc::SyncSender<Video>) -> Result<(), ffmpeg_next::Error> {
        let mut video_file = input(path)?;
        let input = video_file
            .streams()
            .best(Type::Video)
            .unwrap();
        let video_stream_index = input.index();

        let context_decoder = ffmpeg_next::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        let mut receive_and_process_decoded_frames = 
            |decoder: &mut ffmpeg_next::decoder::Video| -> Result<(), ffmpeg_next::Error> {
                let mut decoded = Video::empty();

                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    let tx_response = tx.send(rgb_frame);

                    if let Err(_) = tx_response {
                        // Return immediatly if receiver has been dropped
                        return Ok(());
                    }
                }

                Ok(())
            };
        
        for (stream, packet) in video_file.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }

        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
        Ok(())
    }

    pub fn get_frame(&mut self) -> Result<Video, mpsc::RecvError> {
        self.decoder_rx.recv()
    }
}