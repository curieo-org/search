mod err;
pub mod routing;
pub mod settings;
pub mod startup;

pub type Result<T> = std::result::Result<T, err::AppError>;
