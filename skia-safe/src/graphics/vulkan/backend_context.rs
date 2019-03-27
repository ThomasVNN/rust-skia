use std::ffi;
use crate::prelude::*;
use super::{Instance, PhysicalDevice, Device, Queue, GetProc };
use skia_bindings::{C_GrVkBackendContext_New, C_GrVkBackendContext_Delete, GrVkExtensionFlags, GrVkFeatureFlags};

bitflags! {
    pub struct ExtensionFlags : u32 {
        const EXT_DEBUG_REPORT = skia_bindings::GrVkExtensionFlags_kEXT_debug_report_GrVkExtensionFlag as _;
        const NV_GLSL_SHADER = skia_bindings::GrVkExtensionFlags_kNV_glsl_shader_GrVkExtensionFlag as _;
        const KHR_SURFACE = skia_bindings::GrVkExtensionFlags_kKHR_surface_GrVkExtensionFlag as _;
        const KHR_SWAPCHAIN = skia_bindings::GrVkExtensionFlags_kKHR_swapchain_GrVkExtensionFlag as _;
        const KHR_WIN32_SURFACE = skia_bindings::GrVkExtensionFlags_kKHR_win32_surface_GrVkExtensionFlag as _;
        const KHR_ANDROID_SURFACE = skia_bindings::GrVkExtensionFlags_kKHR_android_surface_GrVkExtensionFlag as _;
        const KHR_XCB_SURFACE = skia_bindings::GrVkExtensionFlags_kKHR_xcb_surface_GrVkExtensionFlag as _;
    }
}

impl NativeTransmutable<GrVkExtensionFlags> for ExtensionFlags {}
#[test]
fn test_extension_flags_layout() {
    ExtensionFlags::test_layout();
}

bitflags! {
    pub struct FeatureFlags: u32 {
        const GEOMETRY_SHADER = skia_bindings::GrVkFeatureFlags_kGeometryShader_GrVkFeatureFlag as _;
        const DUAL_SRC_BLEND = skia_bindings::GrVkFeatureFlags_kDualSrcBlend_GrVkFeatureFlag as _;
        const SAMPLE_RATE_SHADING = skia_bindings::GrVkFeatureFlags_kSampleRateShading_GrVkFeatureFlag as _;
    }
}

impl NativeTransmutable<GrVkFeatureFlags> for FeatureFlags {}
#[test]
fn test_feature_flags_layout() {
    FeatureFlags::test_layout();
}

// Note: the GrBackendContext's layout generated by bindgen does not match in size,
// so we do need to use a pointer here for now.
#[derive(Debug)]
pub struct BackendContext {
    pub (crate) native: *mut ffi::c_void
}

impl Drop for BackendContext {
    fn drop(&mut self) {
        unsafe { C_GrVkBackendContext_Delete(self.native) }
    }
}

// TODO: add some accessor functions to the public fields.
// TODO: may support Clone (note the original structure holds a smartpointer!)

impl BackendContext {

    pub unsafe fn new(
        instance: Instance,
        physical_device: PhysicalDevice,
        device: Device,
        queue: Queue,
        graphics_queue_index: u32,
        get_proc: GetProc
        ) -> BackendContext {

        BackendContext {
            native: C_GrVkBackendContext_New(
                instance as _,
                physical_device as _,
                device as _,
                queue as _,
                graphics_queue_index,
                get_proc) }
    }
}