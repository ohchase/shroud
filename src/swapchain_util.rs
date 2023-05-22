use std::process::id;

use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    Graphics::Dxgi::{
        Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_RATIONAL, DXGI_SAMPLE_DESC},
        IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
        DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
    },
    UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId},
};

pub(crate) fn get_process_window() -> Option<HWND> {
    extern "system" fn enum_windows_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {
        let mut wnd_proc_id: u32 = 0;
        unsafe {
            GetWindowThreadProcessId(hwnd, Some(&mut wnd_proc_id));
            if id() != wnd_proc_id {
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
    };

    match output.0 == 0 {
        true => None,
        false => Some(output),
    }
}

pub fn get_window_from_swapchain(swapchain: &IDXGISwapChain) -> Option<HWND> {
    let mut desc = default_swapchain_descriptor(HWND::default());
    match unsafe { (swapchain).GetDesc(&mut desc) } {
        Err(_e) => None,
        Ok(_) => Some(desc.OutputWindow),
    }
}

pub fn default_swapchain_descriptor(window: HWND) -> DXGI_SWAP_CHAIN_DESC {
    let buffer_desc = DXGI_MODE_DESC {
        Width: 0,
        Height: 0,
        RefreshRate: DXGI_RATIONAL {
            Numerator: 60,
            Denominator: 1,
        },
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        ScanlineOrdering: windows::Win32::Graphics::Dxgi::Common::DXGI_MODE_SCANLINE_ORDER(0),
        Scaling: windows::Win32::Graphics::Dxgi::Common::DXGI_MODE_SCALING(0),
    };

    let sample_desc = DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0,
    };

    DXGI_SWAP_CHAIN_DESC {
        BufferDesc: buffer_desc,
        SampleDesc: sample_desc,
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        OutputWindow: window,
        Windowed: true.into(),
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
        Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
    }
}
