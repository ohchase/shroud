use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use winapi::{
    ctypes::c_void,
    shared::dxgi::{CreateDXGIFactory, IDXGIFactory, DXGI_SWAP_EFFECT_FLIP_DISCARD},
    um::{
        d3d12::{
            D3D12CreateDevice, ID3D12CommandAllocator, ID3D12CommandList, ID3D12CommandQueue,
            ID3D12Device, ID3D12GraphicsCommandList, D3D12_COMMAND_LIST_TYPE_DIRECT,
            D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE,
        },
        d3dcommon::D3D_FEATURE_LEVEL_11_0,
        unknwnbase::IUnknown,
    },
    Interface,
};

use crate::{directx::Window, ShroudError, ShroudResult};

use super::swapchain_util::default_swapchain_descriptor;

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX12DeviceMethods {
    QueryInterface,
    AddRef,
    Release,
    GetPrivateData,
    SetPrivateData,
    SetPrivateDataInterface,
    SetName,
    GetNodeCount,
    CreateCommandQueue,
    CreateCommandAllocator,
    CreateGraphicsPipelineState,
    CreateComputePipelineState,
    CreateCommandList,
    CheckFeatureSupport,
    CreateDescriptorHeap,
    GetDescriptorHandleIncrementSize,
    CreateRootSignature,
    CreateConstantBufferView,
    CreateShaderResourceView,
    CreateUnorderedAccessView,
    CreateRenderTargetView,
    CreateDepthStencilView,
    CreateSampler,
    CopyDescriptors,
    CopyDescriptorsSimple,
    GetResourceAllocationInfo,
    GetCustomHeapProperties,
    CreateCommittedResource,
    CreateHeap,
    CreatePlacedResource,
    CreateReservedResource,
    CreateSharedHandle,
    OpenSharedHandle,
    OpenSharedHandleByName,
    MakeResident,
    Evict,
    CreateFence,
    GetDeviceRemovedReason,
    GetCopyableFootprints,
    CreateQueryHeap,
    SetStablePowerState,
    CreateCommandSignature,
    GetResourceTiling,
    GetAdapterLuid,
}

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX12CommandQueueMethods {
    QueryInterface,
    AddRef,
    Release,
    GetPrivateData,
    SetPrivateData,
    SetPrivateDataInterface,
    SetName,
    GetDevice,
    UpdateTileMappings,
    CopyTileMappings,
    ExecuteCommandLists,
    SetMarker,
    BeginEvent,
    EndEvent,
    Signal,
    Wait,
    GetTimestampFrequency,
    GetClockCalibration,
    GetDesc,
}

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX12CommandAllocatorMethods {
    QueryInterface,
    AddRef,
    Release,
    GetPrivateData,
    SetPrivateData,
    SetPrivateDataInterface,
    SetName,
    GetDevice,
    Reset,
}

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX12CommandListMethods {
    QueryInterface,
    AddRef,
    Release,
    GetPrivateData,
    SetPrivateData,
    SetPrivateDataInterface,
    SetName,
    GetDevice,
    GetType,
    Close,
    Reset,
    ClearState,
    DrawInstanced,
    DrawIndexedInstanced,
    Dispatch,
    CopyBufferRegion,
    CopyTextureRegion,
    CopyResource,
    CopyTiles,
    ResolveSubresource,
    IASetPrimitiveTopology,
    RSSetViewports,
    RSSetScissorRects,
    OMSetBlendFactor,
    OMSetStencilRef,
    SetPipelineState,
    ResourceBarrier,
    ExecuteBundle,
    SetDescriptorHeaps,
    SetComputeRootSignature,
    SetGraphicsRootSignature,
    SetComputeRootDescriptorTable,
    SetGraphicsRootDescriptorTable,
    SetComputeRoot32BitConstant,
    SetGraphicsRoot32BitConstant,
    SetComputeRoot32BitConstants,
    SetGraphicsRoot32BitConstants,
    SetComputeRootConstantBufferView,
    SetGraphicsRootConstantBufferView,
    SetComputeRootShaderResourceView,
    SetGraphicsRootShaderResourceView,
    SetComputeRootUnorderedAccessView,
    SetGraphicsRootUnorderedAccessView,
    IASetIndexBuffer,
    IASetVertexBuffers,
    SOSetTargets,
    OMSetRenderTargets,
    ClearDepthStencilView,
    ClearRenderTargetView,
    ClearUnorderedAccessViewUint,
    ClearUnorderedAccessViewFloat,
    DiscardResource,
    BeginQuery,
    EndQuery,
    ResolveQueryData,
    SetPredication,
    SetMarker,
    BeginEvent,
    EndEvent,
    ExecuteIndirect,
}

#[derive(Debug, EnumIter, EnumCount)]
pub enum DirectX12SwapchainMethods {
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

pub struct DirectX12Methods {
    device_vmt: Vec<*const usize>,
    command_queue_vmt: Vec<*const usize>,
    command_allocator_vmt: Vec<*const usize>,
    command_list_vmt: Vec<*const usize>,
    swapchain_vmt: Vec<*const usize>,
}

impl DirectX12Methods {
    fn device_vmt(&self) -> &Vec<*const usize> {
        &self.device_vmt
    }

    fn command_queue_vmt(&self) -> &Vec<*const usize> {
        &self.command_queue_vmt
    }

    fn command_allocator_vmt(&self) -> &Vec<*const usize> {
        &self.command_allocator_vmt
    }

    fn command_list_vmt(&self) -> &Vec<*const usize> {
        &self.command_list_vmt
    }

    fn swapchain_vmt(&self) -> &Vec<*const usize> {
        &self.swapchain_vmt
    }
}

impl std::fmt::Debug for DirectX12Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DirectX12 Method Table")?;

        let mut index = 0;
        writeln!(f, "Device Virtual Method Table")?;
        for (i, method) in DirectX12DeviceMethods::iter().enumerate() {
            writeln!(f, "\t[{}] {:?} {:#?}", index, method, self.device_vmt[i])?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Command Queue Virtual Method Table")?;
        for (i, method) in DirectX12CommandQueueMethods::iter().enumerate() {
            writeln!(
                f,
                "\t[{}] {:?} {:#?}",
                index, method, self.command_queue_vmt[i]
            )?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Command Allocator Virtual Method Table")?;
        for (i, method) in DirectX12CommandAllocatorMethods::iter().enumerate() {
            writeln!(
                f,
                "\t[{}] {:?} {:#?}",
                index, method, self.command_allocator_vmt[i]
            )?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Command List Virtual Method Table")?;
        for (i, method) in DirectX12CommandListMethods::iter().enumerate() {
            writeln!(
                f,
                "\t[{}] {:?} {:#?}",
                index, method, self.command_list_vmt[i]
            )?;

            index += 1;
        }
        writeln!(f)?;

        writeln!(f, "Swapchain Virtual Method Table")?;
        for (i, method) in DirectX12SwapchainMethods::iter().enumerate() {
            writeln!(f, "\t[{}] {:?} {:#?}", index, method, self.swapchain_vmt[i])?;

            index += 1;
        }
        writeln!(f)?;

        Ok(())
    }
}

pub fn methods() -> ShroudResult<DirectX12Methods> {
    // Initialize Factory
    let mut factory: *mut IDXGIFactory = std::ptr::null_mut();
    let result = unsafe {
        CreateDXGIFactory(
            &IDXGIFactory::uuidof(),
            &mut factory as *mut *mut _ as *mut *mut c_void,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX12CreateFactory(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX12 Factory has been released.");
            (*factory).Release();
        }
    }

    // Initialize adapter
    let mut adapter = std::ptr::null_mut();
    let result = unsafe { (*factory).EnumAdapters(0, &mut adapter) };
    if result < 0 {
        return Err(ShroudError::DirectX12EnumAdapters(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX12 Adapter has been released.");
            (*adapter).Release();
        }
    }

    // Initialize device
    let mut device: *mut ID3D12Device = std::ptr::null_mut();
    let result = unsafe {
        D3D12CreateDevice(
            adapter as *mut _ as *mut IUnknown,
            D3D_FEATURE_LEVEL_11_0,
            &ID3D12Device::uuidof(),
            &mut device as *mut *mut _ as *mut *mut c_void,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX12CreateDevice(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX12 Device has been released.");
            (*device).Release();
        }
    }

    // Create queue descriptor
    let queue_desc = D3D12_COMMAND_QUEUE_DESC {
        Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
        Priority: 0,
        Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
        NodeMask: 0,
    };

    // Initialize command queue
    let mut command_queue: *mut ID3D12CommandQueue = std::ptr::null_mut();
    let result = unsafe {
        (*device).CreateCommandQueue(
            &queue_desc,
            &ID3D12CommandQueue::uuidof(),
            &mut command_queue as *mut *mut _ as *mut *mut c_void,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX12CreateCommandQueue(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX12 Command Queue has been released.");
            (*command_queue).Release();
        }
    }

    // Initialize command allocator
    let mut command_allocator: *mut ID3D12CommandAllocator = std::ptr::null_mut();
    let result = unsafe {
        (*device).CreateCommandAllocator(
            D3D12_COMMAND_LIST_TYPE_DIRECT,
            &ID3D12CommandAllocator::uuidof(),
            &mut command_allocator as *mut *mut _ as *mut *mut c_void,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX12CreateCommandAllocator(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX12 Command Allocator has been released.");
            (*command_allocator).Release();
        }
    }

    // Initialize command list
    let mut command_list: *mut ID3D12CommandList = std::ptr::null_mut();
    let result = unsafe {
        (*device).CreateCommandList(
            0,
            D3D12_COMMAND_LIST_TYPE_DIRECT,
            command_allocator,
            std::ptr::null_mut(),
            &ID3D12GraphicsCommandList::uuidof(),
            &mut command_list as *mut *mut _ as *mut *mut c_void,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX12CreateCommandList(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX12 Command List has been released.");
            (*command_list).Release();
        }
    }

    // create default swap chain descriptor, and create d3d12 swapchain
    let window = Window::default();
    let mut swapchain_desc = default_swapchain_descriptor(&window);
    swapchain_desc.BufferCount = 2;
    swapchain_desc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;

    let mut swapchain = std::ptr::null_mut();
    let result = unsafe {
        (*factory).CreateSwapChain(
            command_queue as *mut _ as *mut IUnknown,
            &mut swapchain_desc,
            &mut swapchain,
        )
    };
    if result < 0 {
        return Err(ShroudError::DirectX12CreateSwapchain(result));
    }
    scopeguard::defer! {
        unsafe {
            log::info!("DirectX12 Swapchain has been released.");
            (*swapchain).Release();
        }
    }

    let device_vmt = unsafe {
        std::slice::from_raw_parts(
            (device as *const _ as *const *const *const usize).read(),
            DirectX12DeviceMethods::COUNT,
        )
        .to_vec()
    };
    let command_queue_vmt = unsafe {
        std::slice::from_raw_parts(
            (command_queue as *const _ as *const *const *const usize).read(),
            DirectX12CommandQueueMethods::COUNT,
        )
        .to_vec()
    };
    let command_allocator_vmt = unsafe {
        std::slice::from_raw_parts(
            (command_allocator as *const _ as *const *const *const usize).read(),
            DirectX12CommandAllocatorMethods::COUNT,
        )
        .to_vec()
    };

    let command_list_vmt = unsafe {
        std::slice::from_raw_parts(
            (command_list as *const _ as *const *const *const usize).read(),
            DirectX12CommandListMethods::COUNT,
        )
        .to_vec()
    };

    let swapchain_vmt = unsafe {
        std::slice::from_raw_parts(
            (swapchain as *const _ as *const *const *const usize).read(),
            DirectX12SwapchainMethods::COUNT,
        )
        .to_vec()
    };

    Ok(DirectX12Methods {
        device_vmt,
        command_queue_vmt,
        command_allocator_vmt,
        command_list_vmt,
        swapchain_vmt,
    })
}
