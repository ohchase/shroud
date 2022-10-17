pub mod directx10;
pub mod directx11;
pub mod directx12;
pub mod directx9;

use winapi::{
    shared::{
        dxgi::{
            DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_DISCARD,
        },
        dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
        dxgitype::{
            DXGI_MODE_DESC, DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
            DXGI_RATIONAL, DXGI_SAMPLE_DESC, DXGI_USAGE_RENDER_TARGET_OUTPUT,
        },
        windef::HWND,
    },
    um::{
        libloaderapi::GetModuleHandleA,
        winuser::{
            CreateWindowExA, DefWindowProcA, DestroyWindow, RegisterClassA, UnregisterClassA,
            WNDCLASSA, WS_OVERLAPPEDWINDOW,
        },
    },
};

struct WindowClass(WNDCLASSA);

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

impl Drop for WindowClass {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping window class");
            UnregisterClassA(self.0.lpszClassName, self.0.hInstance);
        }
    }
}

struct Window(HWND, WindowClass);

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

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            println!("Destroying window.");
            DestroyWindow(self.0);
        }
    }
}

fn default_swapchain_descriptor(window: &Window) -> DXGI_SWAP_CHAIN_DESC {
    let refresh_rate = DXGI_RATIONAL {
        Numerator: 60,
        Denominator: 1,
    };

    let buffer_desc = DXGI_MODE_DESC {
        Width: 100,
        Height: 100,
        RefreshRate: refresh_rate,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
        Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
    };

    let sample_desc = DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0,
    };

    DXGI_SWAP_CHAIN_DESC {
        BufferDesc: buffer_desc,
        SampleDesc: sample_desc,
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 1,
        OutputWindow: window.inner(),
        Windowed: 1,
        SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
        Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
    }
}
