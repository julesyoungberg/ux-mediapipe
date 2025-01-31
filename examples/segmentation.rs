use mediapipe::*;
use opencv::prelude::*;
use opencv::{highgui, imgproc, videoio, Result};

fn segmentation() -> Result<()> {
    let window = "video capture";

    highgui::named_window(window, highgui::WINDOW_AUTOSIZE)?;

    let mut cap = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !cap.is_opened()? {
        panic!("Unable to open default cam")
    }

    cap.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
    cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
    cap.set(videoio::CAP_PROP_FPS, 30.0)?;

    let mut detector = segmentation::Segmentor::default();

    let mut raw_frame = Mat::default();
    let mut rgb_frame = Mat::default();
    let mut flip_frame = Mat::default();
    loop {
        cap.read(&mut raw_frame)?;

        let size = raw_frame.size()?;
        if size.width > 0 && !raw_frame.empty() {
            imgproc::cvt_color(&raw_frame, &mut rgb_frame, imgproc::COLOR_BGR2RGB, 0)?;
            opencv::core::flip(&rgb_frame, &mut flip_frame, 1)?; // horizontal

            println!("processing");
            let mut result = detector.process(&flip_frame);
            println!("processed");
            highgui::imshow(window, &mut result)?;
        } else {
            println!("WARN: Skip empty frame");
        }

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

fn main() {
    segmentation().unwrap()
}
