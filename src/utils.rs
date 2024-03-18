// src/utils.rs

pub fn format_click_message(clicks: i32) -> String {
    match clicks {
        0 => "Click here".to_string(), // For Button initial state
        1 => "Clicked 1 time".to_string(),
        _ => format!("Clicked {} times", clicks),
    }
}

