use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct VideoDecoder {
    //decoder: DecoderSplit,
    //reader: Reader,
    //stream_index: usize,

    out_buffer: Arc<Mutex<VecDeque<Box<[u8]>>>>,
}

impl VideoDecoder {
    pub fn new(path: &str) -> Result<Self, ffmpeg_next::Error> {
        let out_buffer = Arc::new(Mutex::new(VecDeque::new()));

        Ok(Self {
            out_buffer,
        })
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
