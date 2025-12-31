use std::sync::Arc;

use parking_lot::Mutex;

use crate::platform::wgpu_backend::WgpuContext;

pub(crate) struct WgpuAtlas(Mutex<WgpuAtlasState>);

struct WgpuAtlasState {
    buffer: wgpu::Buffer,
    context: Arc<WgpuContext>,
}

impl WgpuAtlasState {
  
}

struct WgpuAtlasTexture {}
