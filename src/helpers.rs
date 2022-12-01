pub(crate)

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
