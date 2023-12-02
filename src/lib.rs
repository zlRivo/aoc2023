#[macro_export]
macro_rules! read_file {
    ($str: expr) => {
        {
            std::fs::read_to_string($str).unwrap_or_else(|_| "".to_string())
        }
    }
}