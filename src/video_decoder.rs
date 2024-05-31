use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;

pub struct VideoDecoder {
    // TODO
}

impl VideoDecoder {
    pub fn new(path: &str) -> Result<Self, ffmpeg_next::Error> {
        // TODO: Threading

        let mut s = Self {
            // TODO
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
}
