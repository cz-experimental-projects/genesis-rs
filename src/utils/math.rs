pub fn clamp(value: f32, min_value: f32, max_value: f32) -> f32 {
    if value > max_value {
        return max_value;
    }

    if value < min_value {
        return min_value;
    }

    value
}
