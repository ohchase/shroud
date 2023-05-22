use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use windows::Win32::Graphics::{
    Direct3D::{
        D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_11_1,
    },
    Direct3D11::{
        D3D11CreateDeviceAndSwapChain, ID3D11Device, ID3D11DeviceContext, D3D11_SDK_VERSION,
    },
    Dxgi::IDXGISwapChain,
};

use crate::{
    swapchain_util::{default_swapchain_descriptor, get_process_window},
    ShroudError, ShroudResult,
};

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX11SwapchainMethods {
    QueryInterface,
    AddRef,
    Release,
    SetPrivateData,
    SetPrivateDataInterface,
    GetPrivateData,
    GetParent,
    GetDevice,
    Present,
    GetBuffer,
    SetFullscreenState,
    GetFullscreenState,
    GetDesc,
    ResizeBuffers,
    ResizeTarget,
    GetContainingOutput,
    GetFrameStatistics,
    GetLastPresentCount,
}

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX11DeviceMethods {
    QueryInterface,
    AddRef,
    Release,
    CreateBuffer,
    CreateTexture1D,
    CreateTexture2D,
    CreateTexture3D,
    CreateShaderResourceView,
    CreateUnorderedAccessView,
    CreateRenderTargetView,
    CreateDepthStencilView,
    CreateInputLayout,
    CreateVertexShader,
    CreateGeometryShader,
    CreateGeometryShaderWithStreamOutput,
    CreatePixelShader,
    CreateHullShader,
    CreateDomainShader,
    CreateComputeShader,
    CreateClassLinkage,
    CreateBlendState,
    CreateDepthStencilState,
    CreateRasterizerState,
    CreateSamplerState,
    CreateQuery,
    CreatePredicate,
    CreateCounter,
    CreateDeferredContext,
    OpenSharedResource,
    CheckFormatSupport,
    CheckMultisampleQualityLevels,
    CheckCounterInfo,
    CheckCounter,
    CheckFeatureSupport,
    GetPrivateData,
    SetPrivateData,
    SetPrivateDataInterface,
    GetFeatureLevel,
    GetCreationFlags,
    GetDeviceRemovedReason,
    GetImmediateContext,
    SetExceptionMode,
    GetExceptionMode,
}

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX11ContextMethods {
    QueryInterface,
    AddRef,
    Release,
    GetDevice,
    GetPrivateData,
    SetPrivateData,
    SetPrivateDataInterface,
    SetConstantBuffers,
    SetShaderResources,
    SetShader,
    SetSamplers,
    SetShader1,
    DrawIndexed,
    Draw,
    Map,
    Unmap,
    SetConstantBuffers1,
    IASetInputLayout,
    IASetVertexBuffers,
    IASetIndexBuffer,
    DrawIndexedInstanced,
    DrawInstanced,
    SetConstantBuffers2,
    SetShader2,
    IASetPrimitiveTopology,
    SetShaderResources1,
    SetSamplers1,
    Begin,
    End,
    GetData,
    SetPredication,
    SetShaderResources2,
    SetSamplers2,
    OMSetRenderTargets,
    OMSetRenderTargetsAndUnorderedAccessViews,
    OMSetBlendState,
    OMSetDepthStencilState,
    SOSetTargets,
    DrawAuto,
    DrawIndexedInstancedIndirect,
    DrawInstancedIndirect,
    Dispatch,
    DispatchIndirect,
    RSSetState,
    RSSetViewports,
    RSSetScissorRects,
    CopySubresourceRegion,
    CopyResource,
    UpdateSubresource,
    CopyStructureCount,
    ClearRenderTargetView,
    ClearUnorderedAccessViewUint,
    ClearUnorderedAccessViewFloat,
    ClearDepthStencilView,
    GenerateMips,
    SetResourceMinLOD,
    GetResourceMinLOD,
    ResolveSubresource,
    ExecuteCommandList,
    SetShaderResources3,
    SetShader4,
    SetSamplers3,
    SetConstantBuffers3,
    SetShaderResources4,
    SetShader5,
    SetSamplers4,
    SetConstantBuffers4,
    SetShaderResources5,
    CSSetUnorderedAccessViews,
    SetShader6,
    SetSamplers5,
    SetConstantBuffers5,
    VSGetConstantBuffers,
    PSGetShaderResources,
    PSGetShader,
    PSGetSamplers,
    VSGetShader,
    PSGetConstantBuffers,
    IAGetInputLayout,
    IAGetVertexBuffers,
    IAGetIndexBuffer,
    GSGetConstantBuffers,
    GSGetShader,
    IAGetPrimitiveTopology,
    VSGetShaderResources,
    VSGetSamplers,
    GetPredication,
    GSGetShaderResources,
    GSGetSamplers,
    OMGetRenderTargets,
    OMGetRenderTargetsAndUnorderedAccessViews,
    OMGetBlendState,
    OMGetDepthStencilState,
    SOGetTargets,
    RSGetState,
    RSGetViewports,
    RSGetScissorRects,
    HSGetShaderResources,
    HSGetShader,
    HSGetSamplers,
    HSGetConstantBuffers,
    DSGetShaderResources,
    DSGetShader,
    DSGetSamplers,
    DSGetConstantBuffers,
    CSGetShaderResources,
    CSGetUnorderedAccessViews,
    CSGetShader,
    CSGetSamplers,
    CSGetConstantBuffers,
    ClearState,
    Flush,
    GetType,
    GetContextFlags,
    FinishCommandList,
    CopySubresourceRegion1,
    UpdateSubresource1,
    DiscardResource,
    DiscardView,
    SetConstantBuffers1_0,
    SetConstantBuffers1_1,
    SetConstantBuffers1_2,
    SetConstantBuffers1_3,
    SetConstantBuffers1_4,
    SetConstantBuffers1_5,
    VSGetConstantBuffers1,
    HSGetConstantBuffers1,
    DSGetConstantBuffers1,
    GSGetConstantBuffers1,
    PSGetConstantBuffers1,
    CSGetConstantBuffers1,
    SwapDeviceContextState,
    ClearView,
    DiscardView1,
    UpdateTileMappings,
    CopyTileMappings,
    CopyTiles,
    UpdateTiles,
    ResizeTilePool,
    TiledResourceBarrier,
    IsAnnotationEnabled,
    SetMarkerInt,
    BeginEventInt,
    EndEvent,
}

pub struct DirectX11Methods {
    swapchain_vmt: Vec<*const usize>,
    device_vmt: Vec<*const usize>,
    context_vmt: Vec<*const usize>,
}

impl DirectX11Methods {
    pub fn swapchain_vmt(&self) -> &Vec<*const usize> {
        &self.swapchain_vmt
    }

    pub fn device_vmt(&self) -> &Vec<*const usize> {
        &self.device_vmt
    }

    pub fn context_vmt(&self) -> &Vec<*const usize> {
        &self.context_vmt
    }
}

impl std::fmt::Debug for DirectX11Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DirectX11 Method Table")?;

        let mut index = 0;
        writeln!(f, "Swapchain Virtual Method Table")?;
        for (i, method) in DirectX11SwapchainMethods::iter().enumerate() {
            writeln!(f, "\t[{}] {:?} {:#?}", index, method, self.swapchain_vmt[i])?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Devices Virtual Method Table")?;
        for (i, method) in DirectX11SwapchainMethods::iter().enumerate() {
            writeln!(f, "\t[{}] {:?} {:#?}", index, method, self.device_vmt[i])?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Context Virtual Method Table")?;
        for (i, method) in DirectX11ContextMethods::iter().enumerate() {
            writeln!(f, "\t[{}] {:?} {:#?}", index, method, self.context_vmt[i])?;

            index += 1;
        }

        Ok(())
    }
}

pub fn methods() -> ShroudResult<DirectX11Methods> {
    let window = get_process_window().ok_or(ShroudError::Window)?;
    let swapchain_desc = default_swapchain_descriptor(window);
    let feature_level: *mut D3D_FEATURE_LEVEL = std::ptr::null_mut();

    let mut swapchain: Option<IDXGISwapChain> = None;
    let mut device: Option<ID3D11Device> = None;
    let mut device_context: Option<ID3D11DeviceContext> = None;

    unsafe {
        D3D11CreateDeviceAndSwapChain(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            windows::Win32::Graphics::Direct3D11::D3D11_CREATE_DEVICE_FLAG(0),
            Some(&[D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_11_1]),
            D3D11_SDK_VERSION,
            Some(&swapchain_desc),
            Some(&mut swapchain),
            Some(&mut device),
            Some(feature_level),
            Some(&mut device_context),
        )
    }
    .map_err(|e| ShroudError::DirectX11CreateDeviceAndSwapchain(e.code()))?;

    let swapchain = swapchain.ok_or(ShroudError::Expectation("Dx11 Swapchain created"))?;
    let swapchain_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(swapchain).read(),
            DirectX11SwapchainMethods::COUNT,
        )
    }
    .to_vec();

    let device = device.ok_or(ShroudError::Expectation("Dx11 Device created"))?;
    let device_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(device).read(),
            DirectX11DeviceMethods::COUNT,
        )
        .to_vec()
    };

    let device_context = device_context.ok_or(ShroudError::Expectation("Dx11 Context created"))?;
    let context_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(device_context).read(),
            DirectX11ContextMethods::COUNT,
        )
        .to_vec()
    };

    Ok(DirectX11Methods {
        swapchain_vmt,
        device_vmt,
        context_vmt,
    })
}
