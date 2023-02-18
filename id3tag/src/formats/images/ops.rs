//! Image processing operations

/// Check if the cover needs resizing and if the X:Y ratio is acceptable (i.e. not too wide or tall).
pub fn cover_needs_resizing(
    filename: &str,
    max_size: u32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let img = image::open(filename)?;

    // If image ratio is "reasonable", see if it needs resizing and return accordingly.
    // Otherwise return error.
    if aspect_ratio_ok(img.width(), img.height()) {
        if img.width() > max_size || img.height() > max_size {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Err("Image is not in the expected ratio.".into())
    }
}

/// Check if the image ratio is within acceptable limits
pub fn aspect_ratio_ok(x: u32, y: u32) -> bool {
    let min_ratio = 1.0 / 1.5; // 1:2 ratio
    let max_ratio = 1.5 / 1.0; // 2:1 ratio

    let ratio = f64::from(x) / f64::from(y);
    (min_ratio..=max_ratio).contains(&ratio)
}

#[cfg(test)]
///
mod tests {
    use super::*;

    #[test]
    ///
    fn test_cover_needs_resizing() {
        let res = cover_needs_resizing("../testdata/DSOTM_Back.jpeg", 500).unwrap_or_default();
        assert_eq!(res, true);

        let res = cover_needs_resizing("../testdata/DSOTM_Back.jpeg", 1500).unwrap_or_default();
        assert_eq!(res, false);
    }

    #[test]
    ///
    fn test_aspect_ratio_is_ok() {
        assert_eq!(aspect_ratio_ok(100, 100), true);
        assert_eq!(aspect_ratio_ok(500, 500), true);
        assert_eq!(aspect_ratio_ok(100, 150), true);
        assert_eq!(aspect_ratio_ok(150, 100), true);
        assert_eq!(aspect_ratio_ok(100, 151), false);
        assert_eq!(aspect_ratio_ok(151, 100), false);
    }
}