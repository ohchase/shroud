#[cfg(feature = "directx9")]
pub mod directx9;

#[cfg(feature = "directx10")]
pub mod directx10;

#[cfg(feature = "directx11")]
pub mod directx11;

#[cfg(feature = "directx12")]
pub mod directx12;

#[cfg(any(feature = "directx11", feature = "directx12"))]
mod swapchain_util;

use winapi::{
    shared::windef::HWND,
    um::{
        libloaderapi::GetModuleHandleA,
        winuser::{
            CreateWindowExA, DefWindowProcA, DestroyWindow, RegisterClassA, UnregisterClassA,
            WNDCLASSA, WS_OVERLAPPEDWINDOW,
        },
    },
};

pub(crate) struct WindowClass(WNDCLASSA);

impl WindowClass {
    fn new(class_name: &str) -> Self {
        let module_handle = unsafe { GetModuleHandleA(std::ptr::null()) };

        let window_class = WNDCLASSA {
            style: 0,
            lpfnWndProc: Some(DefWindowProcA),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: module_handle,
            hCursor: std::ptr::null_mut(),
            hIcon: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszClassName: class_name.as_ptr() as *const i8,
            lpszMenuName: class_name.as_ptr() as *const i8,
        };

        unsafe {
            RegisterClassA(&window_class);
        }
        Self(window_class)
    }

    fn inner(&self) -> &WNDCLASSA {
        &self.0
    }
}

impl Default for WindowClass {
    fn default() -> Self {
        WindowClass::new("Shroud\0")
    }
}

impl Drop for WindowClass {
    fn drop(&mut self) {
        unsafe {
            UnregisterClassA(self.0.lpszClassName, self.0.hInstance);
        }
    }
}

pub(crate) struct Window(HWND, WindowClass);

impl Window {
    fn new(window_name: &str, window_class: WindowClass) -> Self {
        let window = unsafe {
            CreateWindowExA(
                0,
                window_class.inner().lpszClassName,
                window_name.as_ptr() as *const i8,
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                100,
                100,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                window_class.inner().hInstance,
                std::ptr::null_mut(),
            )
        };

        Self(window, window_class)
    }

    fn inner(&self) -> HWND {
        self.0
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new("Shroud\0", WindowClass::default())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.0);
        }
    }
}
