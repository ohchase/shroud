use std::ffi::{IntoStringError, NulError};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;
use winapi::{shared::minwindef::HMODULE, um::libloaderapi::GetModuleHandleA};

#[cfg(not(target_os = "windows"))]
compile_error!("only windows is supported");

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

static DIRECTX_9_DLL_NAME: &str = concat!("d3d9.dll", "\0");
static DIRECTX_10_DLL_NAME: &str = concat!("d3d10.dll", "\0");
static DIRECTX_11_DLL_NAME: &str = concat!("d3d11.dll", "\0");
static DIRECTX_12_DLL_NAME: &str = concat!("d3d12.dll", "\0");
static OPENGL_DLL_NAME: &str = concat!("opengl32.dll", "\0");
static VULKAN_DLL_NAME: &str = concat!("vulkan-1.dll", "\0");

#[derive(Debug, EnumIter)]
pub enum RenderEngine {
    DirectX9,
    DirectX10,
    DirectX11,
    DirectX12,
    OpenGL,
    Vulkan,
}

pub fn detect_render_engine() -> Option<RenderEngine> {
    RenderEngine::iter()
        .find(|render_engine| RenderEngine::get_render_engine_handle(render_engine).is_ok())
}

impl RenderEngine {
    pub(crate) fn get_render_engine_handle(render_engine: &RenderEngine) -> ShroudResult<HMODULE> {
        let handle_name = RenderEngine::dll_name(render_engine);
        let handle = unsafe { GetModuleHandleA(handle_name.as_ptr() as *const i8) };
        if handle.is_null() {
            return Err(ShroudError::OpenHandleError(handle_name.to_string()));
        }
        Ok(handle)
    }

    pub fn dll_name(entry: &RenderEngine) -> &str {
        match entry {
            RenderEngine::DirectX9 => DIRECTX_9_DLL_NAME,
            RenderEngine::DirectX10 => DIRECTX_10_DLL_NAME,
            RenderEngine::DirectX11 => DIRECTX_11_DLL_NAME,
            RenderEngine::DirectX12 => DIRECTX_12_DLL_NAME,
            RenderEngine::OpenGL => OPENGL_DLL_NAME,
            RenderEngine::Vulkan => VULKAN_DLL_NAME,
        }
    }
}

#[derive(Error, Debug)]
pub enum ShroudError {
    #[error("IntoString Error `{0:#?}`")]
    IntoStringError(#[from] IntoStringError),

    #[error("Std Nul Error `{0:#?}`")]
    StdNulError(#[from] NulError),

    #[error("Error opening handle for dll: `{0:#?}`")]
    OpenHandleError(String),

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
