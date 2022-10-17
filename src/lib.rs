use thiserror::Error;

#[cfg(any(
    feature = "directx9",
    feature = "directx10",
    feature = "directx11",
    feature = "directx12"
))]
pub mod directx;

#[cfg(feature = "opengl")]
pub mod opengl;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[derive(Error, Debug)]
pub enum ShroudError {
    #[cfg(feature = "opengl")]
    #[error("Error getting handle to opengl32.dll")]
    OpenGlHandle(),

    #[cfg(feature = "vulkan")]
    #[error("Error getting handle to vulkan-1.dll")]
    VulkanHandle(),

    #[cfg(feature = "directx9")]
    #[error("Error creating directx9 instance `{0:#?}`")]
    DirectX9Create(i32),
    #[cfg(feature = "directx9")]
    #[error("Error creating directx9 device `{0:#?}`")]
    DirectX9CreateDevice(i32),

    #[cfg(feature = "directx11")]
    #[error("Error creating directx11 device `{0:#?}`")]
    DirectX11CreateDeviceAndSwapchain(i32),

    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 factory `{0:#?}`")]
    DirectX12CreateFactory(i32),
    #[cfg(feature = "directx12")]
    #[error("Error enumerating directx12 adapters `{0:#?}`")]
    DirectX12EnumAdapters(i32),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 device `{0:#?}`")]
    DirectX12CreateDevice(i32),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 command queue `{0:#?}`")]
    DirectX12CreateCommandQueue(i32),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 command allocator `{0:#?}`")]
    DirectX12CreateCommandAllocator(i32),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 command list `{0:#?}`")]
    DirectX12CreateCommandList(i32),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 swapchain `{0:#?}`")]
    DirectX12CreateSwapchain(i32),
}

pub type ShroudResult<T> = std::result::Result<T, ShroudError>;
