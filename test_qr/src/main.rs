use image::*;
use bardecoder;
use bardecoder::prepare::BlockedMean;
fn main() {
    let img = image::open("E:\\rust_demo_projects\\rust_python_example\\back_end\\frame.png").unwrap();

    let mut db = bardecoder::default_builder();

    // Use some different arguments in one of the default components
    db.prepare(Box::new(BlockedMean::new(7, 9)));

    // Build the actual decoder
    let decoder = db.build();

    let results = decoder.decode(&img);
    for result in results {
        println!("{}", result.unwrap());
    }
}
