use std::path::PathBuf;

use video_rs::{decode::DecoderSplit, Reader};
use video_rs::location::Location;
use video_rs::Decoder;

struct VideoDecoder {
    decoder: DecoderSplit,
    reader: Reader,
    stream_index: usize,
}

impl VideoDecoder {
    fn new(path: PathBuf) -> Result<Self, video_rs::Error> {
        let file = Location::File(path);
        let decoder_reader = Decoder::new(file)?;

        let (decoder, reader, stream_index) = decoder_reader.into_parts();

        Ok(Self {
            decoder,
            reader,
            stream_index,
        })
    }
}
