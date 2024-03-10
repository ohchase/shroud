use std::ffi::{IntoStringError, NulError};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;
use windows::{
    core::{HRESULT, PCSTR},
    Win32::{
        Foundation::{BOOL, HMODULE, HWND, LPARAM},
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId},
    },
};

#[cfg(not(target_os = "windows"))]
compile_error!("only windows is supported");

#[cfg(feature = "directx9")]
pub mod directx9;

#[cfg(feature = "directx10")]
pub mod directx10;

#[cfg(feature = "directx11")]
pub mod directx11;

#[cfg(feature = "directx12")]
pub mod directx12;

#[cfg(any(feature = "directx11", feature = "directx12"))]
pub mod swapchain_util;

pub(crate) fn get_process_window() -> Option<HWND> {
    extern "system" fn enum_windows_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {
        let mut wnd_proc_id: u32 = 0;
        unsafe {
            GetWindowThreadProcessId(hwnd, Some(&mut wnd_proc_id));
            if std::process::id() != wnd_proc_id {
                return true.into();
            }

            *(l_param.0 as *mut HWND) = hwnd;
        }
        false.into()
    }

    let mut output: HWND = HWND(0);
    unsafe {
        EnumWindows(
            Some(enum_windows_callback),
            std::mem::transmute::<_, LPARAM>(&mut output as *mut HWND),
        )
        .ok()?
    };

    match output.0 == 0 {
        true => None,
        false => Some(output),
    }
}

static DIRECTX_9_DLL_NAME: &str = concat!("d3d9.dll", "\0");
static DIRECTX_10_DLL_NAME: &str = concat!("d3d10.dll", "\0");
static DIRECTX_11_DLL_NAME: &str = concat!("d3d11.dll", "\0");
static DIRECTX_12_DLL_NAME: &str = concat!("d3d12.dll", "\0");

#[derive(Debug, EnumIter)]
pub enum RenderEngine {
    DirectX9,
    DirectX10,
    DirectX11,
    DirectX12,
}

pub fn detect_render_engine() -> Option<RenderEngine> {
    RenderEngine::iter()
        .find(|render_engine| RenderEngine::get_render_engine_handle(render_engine).is_ok())
}

impl RenderEngine {
    pub(crate) fn get_render_engine_handle(render_engine: &RenderEngine) -> ShroudResult<HMODULE> {
        let handle_name = RenderEngine::dll_name(render_engine);
        let handle = unsafe {
            GetModuleHandleA(PCSTR::from_raw(handle_name.as_ptr()))
                .map_err(|_| ShroudError::OpenHandleError(handle_name.to_owned()))?
        };
        Ok(handle)
    }

    pub fn dll_name(entry: &RenderEngine) -> &str {
        match entry {
            RenderEngine::DirectX9 => DIRECTX_9_DLL_NAME,
            RenderEngine::DirectX10 => DIRECTX_10_DLL_NAME,
            RenderEngine::DirectX11 => DIRECTX_11_DLL_NAME,
            RenderEngine::DirectX12 => DIRECTX_12_DLL_NAME,
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

    #[error("Error finding main process window!")]
    Window,

    #[error("General expectation failed `{0}`")]
    Expectation(&'static str),

    #[cfg(feature = "directx9")]
    #[error("Error creating directx9 instance `{0:#?}`")]
    DirectX9Create(HRESULT),
    #[cfg(feature = "directx9")]
    #[error("Error creating directx9 device `{0:#?}`")]
    DirectX9CreateDevice(HRESULT),

    #[cfg(feature = "directx11")]
    #[error("Error creating directx11 device `{0:#?}`")]
    DirectX11CreateDeviceAndSwapchain(HRESULT),

    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 factory `{0:#?}`")]
    DirectX12CreateFactory(HRESULT),
    #[cfg(feature = "directx12")]
    #[error("Error enumerating directx12 adapters `{0:#?}`")]
    DirectX12EnumAdapters(HRESULT),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 device `{0:#?}`")]
    DirectX12CreateDevice(HRESULT),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 command queue `{0:#?}`")]
    DirectX12CreateCommandQueue(HRESULT),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 command allocator `{0:#?}`")]
    DirectX12CreateCommandAllocator(HRESULT),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 command list `{0:#?}`")]
    DirectX12CreateCommandList(HRESULT),
    #[cfg(feature = "directx12")]
    #[error("Error creating directx12 swapchain `{0:#?}`")]
    DirectX12CreateSwapchain(HRESULT),
}

pub type ShroudResult<T> = std::result::Result<T, ShroudError>;
