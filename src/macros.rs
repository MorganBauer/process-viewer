#[macro_export]
macro_rules! auto_cast {
    ($t:expr, $cast:ty) => {{
        #[cfg(target_os = "windows")]
        {
            $t as $cast
        }
        #[cfg(not(target_os = "windows"))]
        {
            $t
        }
    }}
}