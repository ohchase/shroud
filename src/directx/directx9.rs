use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use winapi::shared::{
    d3d9::{
        Direct3DCreate9Ex, D3DADAPTER_DEFAULT, D3DCREATE_DISABLE_DRIVER_MANAGEMENT,
        D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3D_SDK_VERSION,
    },
    d3d9types::{
        D3DDEVTYPE_NULLREF, D3DFMT_UNKNOWN, D3DMULTISAMPLE_NONE, D3DPRESENT_PARAMETERS,
        D3DSWAPEFFECT_DISCARD,
    },
};

use crate::{ShroudError, ShroudResult};

use super::Window;

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX9DeviceMethods {
    QueryInterface,
    AddRef,
    Release,
    TestCooperativeLevel,
    GetAvailableTextureMem,
    EvictManagedResources,
    GetDirect3D,
    GetDeviceCaps,
    GetDisplayMode,
    GetCreationParameters,
    SetCursorProperties,
    SetCursorPosition,
    ShowCursor,
    CreateAdditionalSwapChain,
    GetSwapChain,
    GetNumberOfSwapChains,
    Reset,
    Present,
    GetBackBuffer,
    GetRasterStatus,
    SetDialogBoxMode,
    SetGammaRamp,
    GetGammaRamp,
    CreateTexture,
    CreateVolumeTexture,
    CreateCubeTexture,
    CreateVertexBuffer,
    CreateIndexBuffer,
    CreateRenderTarget,
    CreateDepthStencilSurface,
    UpdateSurface,
    UpdateTexture,
    GetRenderTargetData,
    GetFrontBufferData,
    StretchRect,
    ColorFill,
    CreateOffscreenPlainSurface,
    SetRenderTarget,
    GetRenderTarget,
    SetDepthStencilSurface,
    GetDepthStencilSurface,
    BeginScene,
    EndScene,
    Clear,
    SetTransform,
    GetTransform,
    MultiplyTransform,
    SetViewport,
    GetViewport,
    SetMaterial,
    GetMaterial,
    SetLight,
    GetLight,
    LightEnable,
    GetLightEnable,
    SetClipPlane,
    GetClipPlane,
    SetRenderState,
    GetRenderState,
    CreateStateBlock,
    BeginStateBlock,
    EndStateBlock,
    SetClipStatus,
    GetClipStatus,
    GetTexture,
    SetTexture,
    GetTextureStageState,
    SetTextureStageState,
    GetSamplerState,
    SetSamplerState,
    ValidateDevice,
    SetPaletteEntries,
    GetPaletteEntries,
    SetCurrentTexturePalette,
    GetCurrentTexturePalette,
    SetScissorRect,
    GetScissorRect,
    SetSoftwareVertexProcessing,
    GetSoftwareVertexProcessing,
    SetNPatchMode,
    GetNPatchMode,
    DrawPrimitive,
    DrawIndexedPrimitive,
    DrawPrimitiveUP,
    DrawIndexedPrimitiveUP,
    ProcessVertices,
    CreateVertexDeclaration,
    SetVertexDeclaration,
    GetVertexDeclaration,
    SetFVF,
    GetFVF,
    CreateVertexShader,
    SetVertexShader,
    GetVertexShader,
    SetVertexShaderConstantF,
    GetVertexShaderConstantF,
    SetVertexShaderConstantI,
    GetVertexShaderConstantI,
    SetVertexShaderConstantB,
    GetVertexShaderConstantB,
    SetStreamSource,
    GetStreamSource,
    SetStreamSourceFreq,
    GetStreamSourceFreq,
    SetIndices,
    GetIndices,
    CreatePixelShader,
    SetPixelShader,
    GetPixelShader,
    SetPixelShaderConstantF,
    GetPixelShaderConstantF,
    SetPixelShaderConstantI,
    GetPixelShaderConstantI,
    SetPixelShaderConstantB,
    GetPixelShaderConstantB,
    DrawRectPatch,
    DrawTriPatch,
    DeletePatch,
    CreateQuery,
}

pub struct DirectX9Methods {
    device_vmt: Vec<*const usize>,
}

impl DirectX9Methods {
    pub fn device_vmt(&self) -> &Vec<*const usize> {
        &self.device_vmt
    }
}

impl std::fmt::Debug for DirectX9Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DirectX9 Method Table")?;
        writeln!(f, "Devices Virtual Method Table")?;
        for (i, method) in DirectX9DeviceMethods::iter().enumerate() {
            writeln!(f, "\t[{}] {:?} {:#?}", i, method, self.device_vmt[i])?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub fn methods() -> ShroudResult<DirectX9Methods> {
    let window = Window::default();

    let mut direct3d_9 = std::ptr::null_mut();
    let result = unsafe { Direct3DCreate9Ex(D3D_SDK_VERSION, &mut direct3d_9) };
    if result < 0 {
        return Err(ShroudError::DirectX9Create(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("Direct3D_9 has been released.");
            (*direct3d_9).Release();
        }
    }

    let mut present_params = D3DPRESENT_PARAMETERS {
        BackBufferWidth: 0,
        BackBufferHeight: 0,
        BackBufferFormat: D3DFMT_UNKNOWN,
        BackBufferCount: 0,
        MultiSampleType: D3DMULTISAMPLE_NONE,
        MultiSampleQuality: 0,
        SwapEffect: D3DSWAPEFFECT_DISCARD,
        hDeviceWindow: window.inner(),
        Windowed: 1,
        EnableAutoDepthStencil: 0,
        AutoDepthStencilFormat: D3DFMT_UNKNOWN,
        Flags: 0,
        FullScreen_RefreshRateInHz: 0,
        PresentationInterval: 0,
    };

    let mut device = std::ptr::null_mut();
    let result = unsafe {
        (*direct3d_9).CreateDevice(
            D3DADAPTER_DEFAULT,
            D3DDEVTYPE_NULLREF,
            window.inner(),
            D3DCREATE_SOFTWARE_VERTEXPROCESSING | D3DCREATE_DISABLE_DRIVER_MANAGEMENT,
            &mut present_params,
            &mut device,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX9CreateDevice(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX9Device has been released.");
            (*device).Release();
        }
    }

    let device_vmt = unsafe {
        std::slice::from_raw_parts(
            (device as *const *const *const usize).read(),
            DirectX9DeviceMethods::COUNT,
        )
        .to_vec()
    };

    Ok(DirectX9Methods { device_vmt })
}
