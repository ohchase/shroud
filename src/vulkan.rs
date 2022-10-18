use strum::{IntoEnumIterator, VariantNames};
use strum_macros::{EnumCount, EnumIter, EnumVariantNames};

use crate::{RenderEngine, ShroudResult};
use winapi::um::libloaderapi::GetProcAddress;

#[allow(non_camel_case_types)]
#[derive(Debug, EnumIter, EnumCount, EnumVariantNames)]
pub enum VulkanStaticMethods {
    vkCreateInstance,
    vkDestroyInstance,
    vkEnumeratePhysicalDevices,
    vkGetPhysicalDeviceFeatures,
    vkGetPhysicalDeviceFormatProperties,
    vkGetPhysicalDeviceImageFormatProperties,
    vkGetPhysicalDeviceProperties,
    vkGetPhysicalDeviceQueueFamilyProperties,
    vkGetPhysicalDeviceMemoryProperties,
    vkGetInstanceProcAddr,
    vkGetDeviceProcAddr,
    vkCreateDevice,
    vkDestroyDevice,
    vkEnumerateInstanceExtensionProperties,
    vkEnumerateDeviceExtensionProperties,
    vkEnumerateDeviceLayerProperties,
    vkGetDeviceQueue,
    vkQueueSubmit,
    vkQueueWaitIdle,
    vkDeviceWaitIdle,
    vkAllocateMemory,
    vkFreeMemory,
    vkMapMemory,
    vkUnmapMemory,
    vkFlushMappedMemoryRanges,
    vkInvalidateMappedMemoryRanges,
    vkGetDeviceMemoryCommitment,
    vkBindBufferMemory,
    vkBindImageMemory,
    vkGetBufferMemoryRequirements,
    vkGetImageMemoryRequirements,
    vkGetImageSparseMemoryRequirements,
    vkGetPhysicalDeviceSparseImageFormatProperties,
    vkQueueBindSparse,
    vkCreateFence,
    vkDestroyFence,
    vkResetFences,
    vkGetFenceStatus,
    vkWaitForFences,
    vkCreateSemaphore,
    vkDestroySemaphore,
    vkCreateEvent,
    vkDestroyEvent,
    vkGetEventStatus,
    vkSetEvent,
    vkResetEvent,
    vkCreateQueryPool,
    vkDestroyQueryPool,
    vkGetQueryPoolResults,
    vkCreateBuffer,
    vkDestroyBuffer,
    vkCreateBufferView,
    vkDestroyBufferView,
    vkCreateImage,
    vkDestroyImage,
    vkGetImageSubresourceLayout,
    vkCreateImageView,
    vkDestroyImageView,
    vkCreateShaderModule,
    vkDestroyShaderModule,
    vkCreatePipelineCache,
    vkDestroyPipelineCache,
    vkGetPipelineCacheData,
    vkMergePipelineCaches,
    vkCreateGraphicsPipelines,
    vkCreateComputePipelines,
    vkDestroyPipeline,
    vkCreatePipelineLayout,
    vkDestroyPipelineLayout,
    vkCreateSampler,
    vkDestroySampler,
    vkCreateDescriptorSetLayout,
    vkDestroyDescriptorSetLayout,
    vkCreateDescriptorPool,
    vkDestroyDescriptorPool,
    vkResetDescriptorPool,
    vkAllocateDescriptorSets,
    vkFreeDescriptorSets,
    vkUpdateDescriptorSets,
    vkCreateFramebuffer,
    vkDestroyFramebuffer,
    vkCreateRenderPass,
    vkDestroyRenderPass,
    vkGetRenderAreaGranularity,
    vkCreateCommandPool,
    vkDestroyCommandPool,
    vkResetCommandPool,
    vkAllocateCommandBuffers,
    vkFreeCommandBuffers,
    vkBeginCommandBuffer,
    vkEndCommandBuffer,
    vkResetCommandBuffer,
    vkCmdBindPipeline,
    vkCmdSetViewport,
    vkCmdSetScissor,
    vkCmdSetLineWidth,
    vkCmdSetDepthBias,
    vkCmdSetBlendConstants,
    vkCmdSetDepthBounds,
    vkCmdSetStencilCompareMask,
    vkCmdSetStencilWriteMask,
    vkCmdSetStencilReference,
    vkCmdBindDescriptorSets,
    vkCmdBindIndexBuffer,
    vkCmdBindVertexBuffers,
    vkCmdDraw,
    vkCmdDrawIndexed,
    vkCmdDrawIndirect,
    vkCmdDrawIndexedIndirect,
    vkCmdDispatch,
    vkCmdDispatchIndirect,
    vkCmdCopyBuffer,
    vkCmdCopyImage,
    vkCmdBlitImage,
    vkCmdCopyBufferToImage,
    vkCmdCopyImageToBuffer,
    vkCmdUpdateBuffer,
    vkCmdFillBuffer,
    vkCmdClearColorImage,
    vkCmdClearDepthStencilImage,
    vkCmdClearAttachments,
    vkCmdResolveImage,
    vkCmdSetEvent,
    vkCmdResetEvent,
    vkCmdWaitEvents,
    vkCmdPipelineBarrier,
    vkCmdBeginQuery,
    vkCmdEndQuery,
    vkCmdResetQueryPool,
    vkCmdWriteTimestamp,
    vkCmdCopyQueryPoolResults,
    vkCmdPushConstants,
    vkCmdBeginRenderPass,
    vkCmdNextSubpass,
    vkCmdEndRenderPass,
    vkCmdExecuteCommands,
}

pub struct VulkanMethods {
    static_methods: Vec<*const usize>,
}

impl std::fmt::Debug for VulkanMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Vulkan Method Table")?;
        writeln!(f, "Static Methods")?;
        for (i, method) in VulkanStaticMethods::iter().enumerate() {
            writeln!(f, "\t[{}] {:?} {:#?}", i, method, self.static_methods[i])?;
        }
        writeln!(f)?;
        Ok(())
    }
}

pub fn methods() -> ShroudResult<VulkanMethods> {
    let handle = RenderEngine::get_render_engine_handle(&RenderEngine::Vulkan)?;

    let mut static_methods: Vec<*const usize> = Vec::new();
    for method_name in VulkanStaticMethods::VARIANTS {
        let method_address =
            unsafe { GetProcAddress(handle, [method_name, "\0"].join("").as_ptr() as *const i8) };
        static_methods.push(method_address as *const usize);
    }

    Ok(VulkanMethods { static_methods })
}
