use std::sync::atomic::{AtomicBool, Ordering};

/// Global flag to track JSON mode
static JSON_MODE: AtomicBool = AtomicBool::new(false);

/// Set the global JSON mode flag
pub fn set_json_mode(enabled: bool) {
    JSON_MODE.store(enabled, Ordering::SeqCst);
}

/// Check if JSON mode is enabled
pub fn is_json_mode() -> bool {
    JSON_MODE.load(Ordering::SeqCst)
}

/// Print to stdout only if not in JSON mode
pub fn log(message: &str) {
    if !is_json_mode() {
        println!("{}", message);
    }
}

/// Log an error to stderr - always prints even in JSON mode
pub fn error(message: &str) {
    eprintln!("Error: {}", message);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json_mode_flag() {
        set_json_mode(false);
        assert_eq!(is_json_mode(), false);
        
        set_json_mode(true);
        assert_eq!(is_json_mode(), true);
        
        set_json_mode(false);
        assert_eq!(is_json_mode(), false);
    }
    
    #[test]
    fn test_error_logging() {
        // Set JSON mode to true
        set_json_mode(true);
        
        // Error logging should still work in JSON mode
        // This test just ensures the functions don't panic
        error("Test error message");
        
        // Reset for other tests
        set_json_mode(false);
    }

    #[test]
    fn test_log_respects_json_mode() {
        // When JSON mode is off, log should print
        set_json_mode(false);
        log("This should print");
        
        // When JSON mode is on, log should not print
        set_json_mode(true);
        log("This should not print");
        
        // Reset for other tests
        set_json_mode(false);
    }
} 