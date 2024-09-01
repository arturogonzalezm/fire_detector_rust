use opencv::{
    core,
    prelude::*,
    imgproc,
    highgui,
    videoio,
    Result,
};

struct FireTracker {
    last_position: Option<core::Point>,
    frame_count: i32,
}

impl FireTracker {
    fn new() -> Self {
        FireTracker {
            last_position: None,
            frame_count: 0,
        }
    }

    fn update(&mut self, contours: &core::Vector<core::Vector<core::Point>>) -> Result<bool> {
        if let Some(largest_contour) = contours.iter().max_by_key(|c| imgproc::contour_area(c, false).unwrap_or(0.0) as i64) {
            let moments = imgproc::moments(&largest_contour, false)?;
            if moments.m00 != 0.0 {
                let cx = (moments.m10 / moments.m00) as i32;
                let cy = (moments.m01 / moments.m00) as i32;
                let current_position = core::Point::new(cx, cy);

                if let Some(last_pos) = self.last_position {
                    let distance = ((current_position.x - last_pos.x).pow(2) + (current_position.y - last_pos.y).pow(2)) as f64;
                    if distance < 50.0 {
                        self.frame_count += 1;
                    } else {
                        self.frame_count = 0;
                    }
                }

                self.last_position = Some(current_position);
                Ok(self.frame_count > 2)  // Lowered to make detection more sensitive
            } else {
                self.frame_count = 0;
                Ok(false)
            }
        } else {
            self.frame_count = 0;
            Ok(false)
        }
    }
}

fn detect_fire(frame: &Mat, tracker: &mut FireTracker) -> Result<Mat> {
    let mut hsv = Mat::default();
    imgproc::cvt_color(frame, &mut hsv, imgproc::COLOR_BGR2HSV, 0)?;

    // Apply histogram equalization to the V (brightness) channel
    let mut hsv_split: core::Vector<Mat> = core::Vector::new();  // Explicit type annotation
    core::split(&hsv, &mut hsv_split)?;

    // Extract the V channel (index 2) as a Mat
    let v_channel = hsv_split.get(2)?.try_clone()?;  // Clone the Mat properly

    // Create a new Mat to hold the equalized result
    let mut equalized_v_channel = Mat::default();
    imgproc::equalize_hist(&v_channel, &mut equalized_v_channel)?;

    // Update the V channel in the vector
    hsv_split.set(2, equalized_v_channel)?;

    // Merge the channels back into a single Mat
    core::merge(&hsv_split, &mut hsv)?;

    // Adjusted HSV range for flames from a lighter
    let lower_fire = core::Scalar::new(0.0, 50.0, 200.0, 0.0);
    let upper_fire = core::Scalar::new(35.0, 255.0, 255.0, 0.0);

    let mut mask = Mat::default();
    core::in_range(&hsv, &lower_fire, &upper_fire, &mut mask)?;

    // Apply adaptive thresholding to enhance detection under varying lighting conditions
    let mut adaptive_mask = Mat::default();
    imgproc::adaptive_threshold(
        &mask,
        &mut adaptive_mask,
        255.0,
        imgproc::ADAPTIVE_THRESH_GAUSSIAN_C,
        imgproc::THRESH_BINARY,
        11,
        2.0,
    )?;

    // Refined morphological operations to reduce noise
    let kernel = Mat::ones(3, 3, core::CV_8U)?.to_mat()?;

    // First morphological operation
    let mut opened_mask = Mat::default();
    imgproc::morphology_ex(&adaptive_mask, &mut opened_mask, imgproc::MORPH_OPEN, &kernel, core::Point::new(-1, -1), 1, core::BORDER_CONSTANT, core::Scalar::all(0.0))?;

    // Second morphological operation
    let mut cleaned = Mat::default();
    imgproc::morphology_ex(&opened_mask, &mut cleaned, imgproc::MORPH_CLOSE, &kernel, core::Point::new(-1, -1), 1, core::BORDER_CONSTANT, core::Scalar::all(0.0))?;

    // Find contours
    let mut contours = core::Vector::<core::Vector<core::Point>>::new();
    imgproc::find_contours(&cleaned, &mut contours, imgproc::RETR_EXTERNAL, imgproc::CHAIN_APPROX_SIMPLE, core::Point::default())?;

    let mut result = frame.clone();
    let is_fire_detected = tracker.update(&contours)?;

    if is_fire_detected {
        for contour in contours.iter() {
            let area = imgproc::contour_area(&contour, false)?;
            let bounding_rect = imgproc::bounding_rect(&contour)?;
            let aspect_ratio = bounding_rect.width as f32 / bounding_rect.height as f32;

            // Focus on smaller, intense areas typical of a lighter flame
            if area > 250.0 && area < 5000.0 && aspect_ratio < 1.5 {
                let roi_boxed = Mat::roi(frame, bounding_rect)?;
                let roi = roi_boxed.try_clone()?;

                let avg_intensity = core::mean(&roi, &core::no_array())?.0[2];
                if avg_intensity > 170.0 {
                    let color = core::Scalar::new(0.0, 0.0, 255.0, 0.0); // Red color
                    imgproc::rectangle(&mut result, bounding_rect, color, 2, imgproc::LINE_8, 0)?;
                }
            }
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    if !cam.is_opened()? {
        return Err(opencv::Error::new(opencv::core::StsError, "Unable to open the default camera!"));
    }

    println!("Camera opened successfully");
    println!("Frame width: {}", cam.get(videoio::CAP_PROP_FRAME_WIDTH)?);
    println!("Frame height: {}", cam.get(videoio::CAP_PROP_FRAME_HEIGHT)?);
    println!("FPS: {}", cam.get(videoio::CAP_PROP_FPS)?);

    cam.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
    cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;

    highgui::named_window("Fire Detection", highgui::WINDOW_NORMAL)?;
    highgui::resize_window("Fire Detection", 640, 480)?;

    let mut frame = Mat::default();
    let mut tracker = FireTracker::new();
    let mut empty_frame_count = 0;
    let max_empty_frames = 10;

    loop {
        if !cam.read(&mut frame)? {
            println!("Failed to read frame from camera");
            break;
        }

        if frame.empty() {
            empty_frame_count += 1;
            println!("Warning: Empty frame captured! Count: {}", empty_frame_count);
            if empty_frame_count > max_empty_frames {
                println!("Too many empty frames. Exiting...");
                break;
            }
            highgui::wait_key(100)?;
            continue;
        }

        empty_frame_count = 0;

        match detect_fire(&frame, &mut tracker) {
            Ok(fire_detected_frame) => {
                highgui::imshow("Fire Detection", &fire_detected_frame)?;
            }
            Err(e) => {
                eprintln!("Error in fire detection: {:?}", e);
                highgui::imshow("Fire Detection", &frame)?;
            }
        }

        if highgui::wait_key(1)? == 'q' as i32 {
            break;
        }
    }

    println!("Exiting program");
    Ok(())
}
