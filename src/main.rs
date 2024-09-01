use opencv::{Result, prelude::*, videoio, highgui};

fn main() -> Result<()> {
    // Open the default camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !cam.is_opened()? {
        panic!("Unable to open the default camera!");
    }

    // Create a window
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;

    let mut frame = Mat::default();

    loop {
        // Capture a frame
        cam.read(&mut frame)?;

        // Check if the frame is empty
        if frame.empty() {
            eprintln!("Warning: Empty frame captured!");
            continue;
        }

        // Show the frame in the window
        highgui::imshow("window", &frame)?;

        // Break the loop if 'q' is pressed (ASCII 113)
        let key = highgui::wait_key(1)?;
        if key == 113 {
            break;
        }
    }

    Ok(())
}
