use std::{fs::File, io::BufReader};

fn main() {
    let f = File::open("vid/OpeningManim.mp4").unwrap();
    let size = f.metadata().unwrap().len();
    let reader = BufReader::new(f);

    let mp4 = mp4::Mp4Reader::read_header(reader, size).unwrap();

    println!("Timescale: {}", mp4.moov.mvhd.timescale);
    println!("Size: {}", mp4.size());
    println!("Duration: {:?}", mp4.duration());
}
