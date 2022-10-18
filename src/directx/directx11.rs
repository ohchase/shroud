use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use winapi::um::{
    d3d11::{D3D11CreateDeviceAndSwapChain, D3D11_SDK_VERSION},
    d3dcommon::{
        D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_0, D3D_FEATURE_LEVEL_11_0,
    },
};

use crate::{ShroudError, ShroudResult};

use super::{swapchain_util::default_swapchain_descriptor, Window, WindowClass};

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
    swapchain_methods: Vec<*const usize>,
    device_methods: Vec<*const usize>,
    context_methods: Vec<*const usize>,
}

impl std::fmt::Debug for DirectX11Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DirectX11 Method Table")?;

        let mut index = 0;
        writeln!(f, "Swapchain Methods")?;
        for (i, method) in DirectX11SwapchainMethods::iter().enumerate() {
            writeln!(
                f,
                "\t[{}] {:?} {:#?}",
                index, method, self.swapchain_methods[i]
            )?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Devices Methods")?;
        for (i, method) in DirectX11SwapchainMethods::iter().enumerate() {
            writeln!(
                f,
                "\t[{}] {:?} {:#?}",
                index, method, self.device_methods[i]
            )?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Context Methods")?;
        for (i, method) in DirectX11ContextMethods::iter().enumerate() {
            writeln!(
                f,
                "\t[{}] {:?} {:#?}",
                index, method, self.context_methods[i]
            )?;

            index += 1;
        }

        Ok(())
    }
}

pub fn methods() -> ShroudResult<DirectX11Methods> {
    let window_class = WindowClass::new("shroud\0");
    let window = Window::new("shroud\0", window_class);
    let swapchain_desc = default_swapchain_descriptor(&window);

    let mut feature_level: D3D_FEATURE_LEVEL = 0;
    let feature_levels: [D3D_FEATURE_LEVEL; 2] = [D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_10_0];

    let mut swapchain = std::ptr::null_mut();
    let mut device = std::ptr::null_mut();
    let mut context = std::ptr::null_mut();

    let result = unsafe {
        D3D11CreateDeviceAndSwapChain(
            std::ptr::null_mut(),
            D3D_DRIVER_TYPE_HARDWARE,
            std::ptr::null_mut(),
            0,
            feature_levels.as_ptr(),
            feature_levels.len() as u32,
            D3D11_SDK_VERSION,
            &swapchain_desc,
            &mut swapchain,
            &mut device,
            &mut feature_level,
            &mut context,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX11CreateDeviceAndSwapchain(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX11 Swapchain has been released.");
            (*swapchain).Release();

            log::info!("DirectX11 Device has been released.");
            (*device).Release();

            log::info!("DirectX11 Context has been released.");
            (*context).Release();
        }
    }

    let swapchain_methods = unsafe {
        std::slice::from_raw_parts(
            (swapchain as *const *const *const usize).read(),
            DirectX11SwapchainMethods::COUNT,
        )
        .to_vec()
    };

    let device_methods = unsafe {
        std::slice::from_raw_parts(
            (device as *const *const *const usize).read(),
            DirectX11DeviceMethods::COUNT,
        )
        .to_vec()
    };

    let context_methods = unsafe {
        std::slice::from_raw_parts(
            (context as *const *const *const usize).read(),
            DirectX11ContextMethods::COUNT,
        )
        .to_vec()
    };

    Ok(DirectX11Methods {
        swapchain_methods,
        device_methods,
        context_methods,
    })
}
