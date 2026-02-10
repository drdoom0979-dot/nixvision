// Declara el archivo frame.rs como un módulo
pub mod frame;

// Re-exporta NixFrame para que puedas usarlo como 
// nixvision_lib::core::NixFrame en lugar de nixvision_lib::core::frame::NixFrame
pub use frame::NixFrame;

