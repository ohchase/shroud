use windows::Win32::{
    Foundation::HWND,
    Graphics::Dxgi::{
        Common::{DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_RATIONAL, DXGI_SAMPLE_DESC},
        IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
        DXGI_SWAP_EFFECT_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT,
    },
};

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
        SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
        Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH.0 as u32,
    }
}
