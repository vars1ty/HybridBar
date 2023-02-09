/// Ensures the `input` is within the range of `min` and `max`.
pub const fn clamp_i32(input: i32, min: i32, max: i32) -> i32 {
    if input < min {
        return min;
    } else if input > max {
        return max;
    }

    input
}
