use thiserror::Error;

mod directx;

pub use directx::directx10;
pub use directx::directx11;
pub use directx::directx12;
pub use directx::directx9;

pub mod opengl;
pub mod vulkan;

#[derive(Error, Debug)]
pub enum ShroudError {
    #[error("Error creating directx9 instance `{0:#?}`")]
    DirectX9Create(i32),
    #[error("Error creating directx9 device `{0:#?}`")]
    DirectX9CreateDevice(i32),

    #[error("Error creating directx11 device `{0:#?}`")]
    DirectX11CreateDeviceAndSwapchain(i32),

    #[error("Error creating directx12 factory `{0:#?}`")]
    DirectX12CreateFactory(i32),
    #[error("Error enumerating directx12 adapters `{0:#?}`")]
    DirectX12EnumAdapters(i32),
    #[error("Error creating directx12 device `{0:#?}`")]
    DirectX12CreateDevice(i32),
    #[error("Error creating directx12 command queue `{0:#?}`")]
    DirectX12CreateCommandQueue(i32),
    #[error("Error creating directx12 command allocator `{0:#?}`")]
    DirectX12CreateCommandAllocator(i32),
    #[error("Error creating directx12 command list `{0:#?}`")]
    DirectX12CreateCommandList(i32),
    #[error("Error creating directx12 swapchain `{0:#?}`")]
    DirectX12CreateSwapchain(i32),
}

pub type ShroudResult<T> = std::result::Result<T, ShroudError>;
