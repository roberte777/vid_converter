use std::{fs::File, io::Write, path::Path};
use vid2img::FileSource;

fn main() {
    let file_path = Path::new("./video.mp4");

    let video_source = FileSource::new(file_path, (1920, 1080)).unwrap();

    let mut number: usize = 0;
    video_source.into_iter().for_each(|frame| {
        if let Ok(Some(png_img_data)) = frame {
            let mut file =
                File::create(Path::new("out").join(format!("frame_{}.png", number))).unwrap();
            file.write_all(&png_img_data).unwrap();
            number += 1;
        }
    });
}
