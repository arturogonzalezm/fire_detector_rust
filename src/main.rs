extern crate opencv;

use opencv::{
    core::{Scalar, Point, Vector, in_range},
    highgui,
    imgproc,
    prelude::*,
    videoio,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the default camera
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Unable to open the camera!");
    }

    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame)?;
        if frame.empty() {
            break;
        }

        // Convert to HSV color space
        let mut hsv_frame = Mat::default();
        imgproc::cvt_color(&frame, &mut hsv_frame, imgproc::COLOR_BGR2HSV, 0)?;

        // Define lower and upper bounds for the color of fire
        let lower_bound = Scalar::new(18.0, 50.0, 50.0, 0.0);
        let upper_bound = Scalar::new(35.0, 255.0, 255.0, 0.0);

        // Threshold the HSV image to get only fire colors
        let mut mask = Mat::default();
        in_range(&hsv_frame, &lower_bound, &upper_bound, &mut mask)?;

        // Find contours
        let mut contours: Vector<Vector<Point>> = Vector::new();
        imgproc::find_contours(
            &mask,
            &mut contours,
            imgproc::RETR_EXTERNAL,
            imgproc::CHAIN_APPROX_SIMPLE,
            Point::new(0, 0),
        )?;

        // Draw contours on the original frame
        for contour in contours.iter() {
            let area = imgproc::contour_area(&contour, false)?;
            if area > 500.0 {
                // Explicitly specify the type for the hierarchy vector
                let hierarchy: Vector<i32> = Vector::new();
                imgproc::draw_contours(
                    &mut frame,
                    &contours,
                    -1,
                    Scalar::new(0.0, 0.0, 255.0, 0.0),
                    2,
                    imgproc::LINE_8,
                    &hierarchy,
                    i32::MAX,
                    Point::new(0, 0),
                )?;
            }
        }

        // Display the result
        highgui::imshow("Fire Detector", &frame)?;
        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    Ok(())
}
