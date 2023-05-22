use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};
use windows::Win32::Graphics::{
    Direct3D::D3D_FEATURE_LEVEL_11_0,
    Direct3D12::{
        D3D12CreateDevice, ID3D12CommandAllocator, ID3D12CommandList, ID3D12CommandQueue,
        ID3D12Device, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC,
        D3D12_COMMAND_QUEUE_FLAG_NONE,
    },
    Dxgi::{CreateDXGIFactory, IDXGIFactory, IDXGISwapChain, DXGI_SWAP_EFFECT_FLIP_DISCARD},
};

use crate::{
    swapchain_util::{default_swapchain_descriptor, get_process_window},
    ShroudError, ShroudResult,
};

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
    pub fn device_vmt(&self) -> &Vec<*const usize> {
        &self.device_vmt
    }

    pub fn command_queue_vmt(&self) -> &Vec<*const usize> {
        &self.command_queue_vmt
    }

    pub fn command_allocator_vmt(&self) -> &Vec<*const usize> {
        &self.command_allocator_vmt
    }

    pub fn command_list_vmt(&self) -> &Vec<*const usize> {
        &self.command_list_vmt
    }

    pub fn swapchain_vmt(&self) -> &Vec<*const usize> {
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
    let factory: IDXGIFactory =
        unsafe { CreateDXGIFactory().map_err(|e| ShroudError::DirectX12CreateFactory(e.code()))? };

    // Initialize adapter
    let adapter = unsafe {
        factory
            .EnumAdapters(0)
            .map_err(|e| ShroudError::DirectX12EnumAdapters(e.code()))?
    };

    // Initialize device
    let mut device = None;
    unsafe {
        D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device)
            .map_err(|e| ShroudError::DirectX12CreateFactory(e.code()))?
    };
    let device: ID3D12Device =
        device.ok_or(ShroudError::Expectation("DirectX12 device populated"))?;

    // Create queue descriptor
    let queue_desc = D3D12_COMMAND_QUEUE_DESC {
        Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
        Priority: 0,
        Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
        NodeMask: 0,
    };

    // Initialize command queue
    let command_queue: ID3D12CommandQueue = unsafe {
        device
            .CreateCommandQueue(&queue_desc)
            .map_err(|e| ShroudError::DirectX12CreateCommandQueue(e.code()))
    }?;

    // Initialize command allocator
    let command_allocator: ID3D12CommandAllocator = unsafe {
        device
            .CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)
            .map_err(|e| ShroudError::DirectX12CreateCommandAllocator(e.code()))
    }?;

    // Initialize command list
    let command_list: ID3D12CommandList = unsafe {
        device
            .CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &command_allocator, None)
            .map_err(|e| ShroudError::DirectX12CreateCommandList(e.code()))
    }?;

    // create default swap chain descriptor, and create d3d12 swapchain
    let window = get_process_window().ok_or(ShroudError::Window)?;
    let mut swapchain_desc = default_swapchain_descriptor(window);
    swapchain_desc.BufferCount = 2;
    swapchain_desc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;

    let mut swapchain = None;
    unsafe {
        let res = factory.CreateSwapChain(&device, &swapchain_desc, &mut swapchain);
        if res.is_err() {
            return Err(ShroudError::DirectX12CreateSwapchain(res));
        }
    };
    let swapchain: IDXGISwapChain =
        swapchain.ok_or(ShroudError::Expectation("DirectX12 device populated"))?;

    let device_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(device).read(),
            DirectX12DeviceMethods::COUNT,
        )
        .to_vec()
    };
    let command_queue_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(command_queue).read(),
            DirectX12CommandQueueMethods::COUNT,
        )
        .to_vec()
    };
    let command_allocator_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(command_allocator).read(),
            DirectX12CommandAllocatorMethods::COUNT,
        )
        .to_vec()
    };

    let command_list_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(command_list).read(),
            DirectX12CommandListMethods::COUNT,
        )
        .to_vec()
    };

    let swapchain_vmt = unsafe {
        std::slice::from_raw_parts(
            std::mem::transmute::<_, *const *const *const usize>(swapchain).read(),
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
