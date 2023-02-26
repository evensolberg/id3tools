//! Image processing operations

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
    fn test_aspect_ratio_is_ok() {
        assert_eq!(aspect_ratio_ok(100, 100), true);
        assert_eq!(aspect_ratio_ok(500, 500), true);
        assert_eq!(aspect_ratio_ok(100, 150), true);
        assert_eq!(aspect_ratio_ok(150, 100), true);
        assert_eq!(aspect_ratio_ok(100, 151), false);
        assert_eq!(aspect_ratio_ok(151, 100), false);
    }
}
