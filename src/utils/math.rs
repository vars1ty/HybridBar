/// Ensures the `input` is within the range of `min` and `max`.
pub fn clamp_i32(input: i32, min: i32, max: i32) -> i32 {
    let mut result = input;
    if input < min {
        result = min;
    } else if input > max {
        result = max;
    }

    result
}
