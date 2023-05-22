use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use windows::Win32::{
    Foundation::{FALSE, TRUE},
    Graphics::Direct3D9::{
        Direct3DCreate9Ex, D3DADAPTER_DEFAULT, D3DCREATE_DISABLE_DRIVER_MANAGEMENT,
        D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3DDEVTYPE_NULLREF, D3DFMT_UNKNOWN,
        D3DMULTISAMPLE_NONE, D3DPRESENT_PARAMETERS, D3DSWAPEFFECT_DISCARD, D3D_SDK_VERSION,
    },
};

use crate::{swapchain_util::get_process_window, ShroudError, ShroudResult};

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
    let window = get_process_window().ok_or(ShroudError::Window)?;
    // let swapchain_desc = default_swapchain_descriptor(window);
    // let feature_level: *mut D3D_FEATURE_LEVEL = std::ptr::null_mut();

    let direct3d_9 = unsafe { Direct3DCreate9Ex(D3D_SDK_VERSION) }
        .map_err(|e| ShroudError::DirectX9Create(e.code()))?;

    let mut present_params = D3DPRESENT_PARAMETERS {
        BackBufferWidth: 0,
        BackBufferHeight: 0,
        BackBufferFormat: D3DFMT_UNKNOWN,
        BackBufferCount: 0,
        MultiSampleType: D3DMULTISAMPLE_NONE,
        MultiSampleQuality: 0,
        SwapEffect: D3DSWAPEFFECT_DISCARD,
        hDeviceWindow: window,
        Windowed: TRUE,
        EnableAutoDepthStencil: FALSE,
        AutoDepthStencilFormat: D3DFMT_UNKNOWN,
        Flags: 0,
        FullScreen_RefreshRateInHz: 0,
        PresentationInterval: 0,
    };

    let mut device = None;
    unsafe {
        direct3d_9
            .CreateDevice(
                D3DADAPTER_DEFAULT,
                D3DDEVTYPE_NULLREF,
                window,
                (D3DCREATE_SOFTWARE_VERTEXPROCESSING | D3DCREATE_DISABLE_DRIVER_MANAGEMENT) as u32,
                &mut present_params,
                &mut device,
            )
            .map_err(|e| ShroudError::DirectX9CreateDevice(e.code()))?
    };

    let device = device.ok_or(ShroudError::Expectation("Dx11 Context created"))?;
    let device_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(device).read(),
            DirectX9DeviceMethods::COUNT,
        )
        .to_vec()
    };

    Ok(DirectX9Methods { device_vmt })
}
