use winapi::shared::{
    dxgi::{
        DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_EFFECT_DISCARD,
    },
    dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
    dxgitype::{
        DXGI_MODE_DESC, DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
        DXGI_RATIONAL, DXGI_SAMPLE_DESC, DXGI_USAGE_RENDER_TARGET_OUTPUT,
    },
};

use super::Window;

pub(crate) fn default_swapchain_descriptor(window: &Window) -> DXGI_SWAP_CHAIN_DESC {
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
