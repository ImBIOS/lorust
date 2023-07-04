/// Computes `number` rounded to `precision`.
///
/// # Examples
///
/// ```rust
/// use lorust::round;
///
/// let rounded = round(4.006, 2);
/// assert_eq!(rounded, 4.01);
/// ```
pub fn round(number: f64, precision: i32) -> f64 {
    let factor = 10.0_f64.powi(precision);
    (number * factor).round() / factor
}
