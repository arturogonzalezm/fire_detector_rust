use opencv::{
    core,
    prelude::*,
    imgproc,
    highgui,
    videoio,
    Result,
};

fn detect_fire(frame: &Mat) -> Result<Mat> {
    let mut hsv = Mat::default();
    imgproc::cvt_color(frame, &mut hsv, imgproc::COLOR_BGR2HSV, 0)?;

    // Define range of fire color in HSV
    let lower_fire = core::Scalar::new(0.0, 100.0, 100.0, 0.0);
    let upper_fire = core::Scalar::new(25.0, 255.0, 255.0, 0.0);

    let mut mask = Mat::default();
    core::in_range(&hsv, &lower_fire, &upper_fire, &mut mask)?;

    // Apply some morphological operations to remove noise
    let kernel = Mat::ones(5, 5, core::CV_8U)?.to_mat()?;
    let border_value = core::Scalar::all(0.0);

    let mut eroded = Mat::default();
    imgproc::erode(&mask, &mut eroded, &kernel, core::Point::new(-1, -1), 2, core::BORDER_CONSTANT, border_value)?;

    let mut dilated = Mat::default();
    imgproc::dilate(&eroded, &mut dilated, &kernel, core::Point::new(-1, -1), 2, core::BORDER_CONSTANT, border_value)?;

    // Find contours
    let mut contours = core::Vector::<core::Vector<core::Point>>::new();
    imgproc::find_contours(&dilated, &mut contours, imgproc::RETR_EXTERNAL, imgproc::CHAIN_APPROX_SIMPLE, core::Point::default())?;

    // Create a copy of the original frame for highlighting
    let result = frame.clone();
    let frame_size = frame.size()?;

    // Create an empty Mat with the same size and type as the original frame
    let mut overlay = Mat::new_size_with_default(frame_size, frame.typ(), core::Scalar::all(0.0))?;

    // Draw filled contours on the overlay
    for (i, contour) in contours.iter().enumerate() {
        let area = imgproc::contour_area(&contour, false)?;
        if area > 1000.0 {  // Increased threshold for less noise
            let color = core::Scalar::new(0.0, 0.0, 255.0, 0.0);  // Pure red color
            imgproc::draw_contours(&mut overlay, &contours, i as i32, color, -1, imgproc::LINE_8, &core::no_array(), 0, core::Point::default())?;
        }
    }

    // Blend the overlay with the original frame
    let mut blended = Mat::default();
    core::add_weighted(&result, 1.0, &overlay, 0.5, 0.0, &mut blended, -1)?;

    // Add text to indicate fire detection
    let font = imgproc::FONT_HERSHEY_SIMPLEX;
    let font_scale = 1.0;
    let font_color = core::Scalar::new(255.0, 255.0, 255.0, 0.0);  // White color
    let thickness = 2;
    imgproc::put_text(&mut blended, "Fire Detected!", core::Point::new(10, 30), font, font_scale, font_color, thickness, imgproc::LINE_AA, false)?;

    Ok(blended)
}


fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !cam.is_opened()? {
        return Err(opencv::Error::new(opencv::core::StsError, "Unable to open the default camera!"));
    }

    highgui::named_window("Fire Detection", highgui::WINDOW_NORMAL)?;

    let mut frame = Mat::default();

    loop {
        cam.read(&mut frame)?;

        if frame.empty() {
            eprintln!("Warning: Empty frame captured!");
            continue;
        }

        match detect_fire(&frame) {
            Ok(fire_detected_frame) => {
                highgui::imshow("Fire Detection", &fire_detected_frame)?;
            }
            Err(e) => {
                eprintln!("Error in fire detection: {:?}", e);
                highgui::imshow("Fire Detection", &frame)?;  // Show original frame on error
            }
        }

        if highgui::wait_key(1)? == 'q' as i32 {
            break;
        }
    }

    Ok(())
}