use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;

pub struct VideoDecoder {
    //decoder: DecoderSplit,
    //reader: Reader,
    //stream_index: usize,

    out_buffer: Arc<Mutex<VecDeque<Box<[u8]>>>>,
}

impl VideoDecoder {
    pub fn new(path: &str) -> Result<Self, ffmpeg_next::Error> {
        let out_buffer = Arc::new(Mutex::new(VecDeque::new()));

        let mut s = Self {
            out_buffer
        };

        s.start_decoding(path)?;

        Ok(s)
    }

    fn start_decoding(&mut self, path: &str) -> Result<(), ffmpeg_next::Error> {
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

        // TODO: dedicated thread for this
        let mut receive_and_process_decoded_frames = 
            |decoder: &mut ffmpeg_next::decoder::Video| -> Result<(), ffmpeg_next::Error> {
                let mut decoded = Video::empty();

                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    // TODO: Enqueue decoded frame somewhere
                    //       or combine this with video player to immediatly display the frames
                }

                Ok(())
            };
        
        // TODO: dedicated thread for this as well
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

    pub fn get_frame(&mut self) -> Video {
        todo!();
    }
    /*
    pub fn new(path: PathBuf) -> Result<Self, video_rs::Error> {
        let file = Location::File(path);
        let decoder_reader = Decoder::new(file)?;

        let (decoder, reader, stream_index) = decoder_reader.into_parts();

        let out_buffer = Arc::new(Mutex::new(VecDeque::new()));

        Ok(Self {
            decoder,
            reader,
            stream_index,
            out_buffer,
        })
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let (width, height) = self.decoder.size_out();
        return (width as usize, height as usize);
    }

    pub fn start_decoding(&mut self) {
        // TODO: One thread to read stream
        //       other (multiple threads) to decode
        todo!();
    }

    pub fn get_frame(&mut self) -> Box<[u8]> {
        loop {
            let mut dequeue = self.out_buffer.lock().unwrap();
            let frame_response = dequeue.pop_front();

            if frame_response.is_none() {
                // Periodically release mutex lock to allow other thread to push data to the queue
                // We never want to get to this point btw
                drop(dequeue);
                println!("WARNING: Output buffer empty");
                thread::sleep(std::time::Duration::from_millis(10));
            }
            else {
                return frame_response.unwrap();
            }
        }
    }
    */
}
