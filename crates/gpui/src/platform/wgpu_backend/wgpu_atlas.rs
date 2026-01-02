use std::sync::Arc;

use collections::FxHashMap;
use etagere::BucketedAtlasAllocator;
use parking_lot::Mutex;

use crate::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, PlatformAtlas,
    Size,
    platform::{AtlasTextureList, wgpu_backend::WgpuContext},
};

pub struct BladeTextureInfo {
    pub raw_view: wgpu::TextureView,
}

pub(crate) struct WgpuAtlas(Mutex<WgpuAtlasState>);

impl WgpuAtlas {
    pub(crate) fn new(context: Arc<WgpuContext>) -> Self {
        WgpuAtlas(Mutex::new(WgpuAtlasState {
            atlas_target: None,
            atlas_target_view: None,
            context,
            storage: WgpuAtlasStorage::default(),
            tiles_by_key: FxHashMap::default(),
            initializations: Vec::new(),
            uploads: Vec::new(),
        }))
    }

    pub fn before_frame(&self, encoder: &mut wgpu::CommandEncoder) {
        let mut state = self.0.lock();

        // Process any pending initializations
        for texture_id in state.initializations.drain(..) {
            todo!()
        }
    }

    pub fn after_frame(&self) {
        let mut state = self.0.lock();

        todo!()
    }

    pub fn get_texture_info(&self, texture_id: AtlasTextureId) -> WgpuTextureInfo {
        let state = self.0.lock();

        todo!()
    }
}

impl PlatformAtlas for WgpuAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<
            Option<(Size<DevicePixels>, std::borrow::Cow<'a, [u8]>)>,
        >,
    ) -> anyhow::Result<Option<AtlasTile>> {
        let mut atlas = self.0.lock();

        match atlas.tiles_by_key.get(key) {
            Some(tile) => Ok(Some(tile.clone())),
            None => Ok({
                profiling::scope!("new tile");

                match build()? {
                    Some((size, bytes)) => {
                        let tile = atlas.allocate(size, key.texture_kind());

                        atlas.upload_texture(tile.texture_id, tile.bounds, &bytes);
                        atlas.tiles_by_key.insert(key.clone(), tile.clone());

                        Some(tile)
                    }
                    None => None,
                }
            }),
        }
    }

    fn remove(&self, key: &AtlasKey) {
        let mut atlas = self.0.lock();

        let Some(id) = atlas.tiles_by_key.remove(key).map(|x| x.texture_id) else {
            return;
        };

        let Some(texture_slot) = atlas.storage[id.kind].textures.get_mut(id.index as usize) else {
            return;
        };

        if let Some(mut texture) = texture_slot.take() {
            texture.decrement_ref_count();

            if texture.is_unreferenced() {
                atlas.storage[id.kind]
                    .free_list
                    .push(texture.id.index as usize);

                texture.destroy(&atlas.context);
            } else {
                *texture_slot = Some(texture);
            }
        }

        todo!()
    }
}

struct WgpuAtlasState {
    atlas_target: Option<wgpu::Texture>,
    atlas_target_view: Option<wgpu::TextureView>,
    context: Arc<WgpuContext>,
    storage: WgpuAtlasStorage,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
    initializations: Vec<AtlasTextureId>,
    uploads: Vec<PendingUpload>,
}

impl WgpuAtlasState {
    fn allocate(&mut self, size: Size<DevicePixels>, texture_kind: AtlasTextureKind) -> AtlasTile {
        todo!()
    }

    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
    ) -> &mut WgpuAtlasTexture {
        todo!()
    }

    fn upload_texture(
        &mut self,
        texture_id: AtlasTextureId,
        bounds: Bounds<DevicePixels>,
        bytes: &[u8],
    ) {
        todo!()
    }

    fn flush_initializations(&mut self, encoder: &mut wgpu::CommandEncoder) {}

    fn flush(&mut self, encoder: &mut wgpu::CommandEncoder) {
        self.flush_initializations(encoder);

        for upload in self.uploads.drain(..) {
            let texture = &self.storage[upload.texture_id];

            encoder.copy_buffer_to_texture(
                wgpu::TexelCopyBufferInfo {
                    buffer: &upload.buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: upload.offset,
                        bytes_per_row: Some(
                            upload.bounds.size.width.to_bytes(texture.bytes_per_pixel()),
                        ),
                        rows_per_image: None,
                    },
                },
                wgpu::TexelCopyTextureInfo {
                    texture: &texture.raw,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: upload.bounds.origin.x.into(),
                        y: upload.bounds.origin.y.into(),
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::Extent3d {
                    width: upload.bounds.size.width.into(),
                    height: upload.bounds.size.height.into(),
                    depth_or_array_layers: 1,
                },
            );
        }
    }
}

struct WgpuAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    raw: wgpu::Texture,
    raw_view: wgpu::TextureView,
    format: wgpu::TextureFormat,
    live_atlas_keys: u32,
}

impl WgpuAtlasTexture {
    fn bytes_per_pixel(&self) -> u8 {
        // TODO(mdeand): There's probably a better way to do this

        match self.format {
            wgpu::TextureFormat::R8Unorm => 1,
            wgpu::TextureFormat::Rgba8Unorm => 4,
            _ => panic!("Unsupported texture format"),
        }
    }

    fn decrement_ref_count(&mut self) {
        self.live_atlas_keys = self.live_atlas_keys.saturating_sub(1);
    }

    fn is_unreferenced(&self) -> bool {
        self.live_atlas_keys == 0
    }

    fn destroy(self, _context: &WgpuContext) {
        // NOTE(mdeand): In wgpu, textures are automatically cleaned up when dropped.
        // NOTE(mdeand): If there were any additional resources to free, they would be handled here.
    }
}

impl std::ops::Index<AtlasTextureKind> for WgpuAtlasStorage {
    type Output = AtlasTextureList<WgpuAtlasTexture>;
    fn index(&self, kind: AtlasTextureKind) -> &Self::Output {
        match kind {
            crate::AtlasTextureKind::Monochrome => &self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &self.polychrome_textures,
        }
    }
}

impl std::ops::IndexMut<AtlasTextureKind> for WgpuAtlasStorage {
    fn index_mut(&mut self, kind: AtlasTextureKind) -> &mut Self::Output {
        match kind {
            crate::AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
        }
    }
}

impl std::ops::Index<AtlasTextureId> for WgpuAtlasStorage {
    type Output = WgpuAtlasTexture;
    fn index(&self, id: AtlasTextureId) -> &Self::Output {
        let textures = match id.kind {
            crate::AtlasTextureKind::Monochrome => &self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &self.polychrome_textures,
        };

        textures[id.index as usize].as_ref().unwrap()
    }
}

#[derive(Default)]
struct WgpuAtlasStorage {
    monochrome_textures: AtlasTextureList<WgpuAtlasTexture>,
    polychrome_textures: AtlasTextureList<WgpuAtlasTexture>,
}

struct WgpuTextureInfo {
    pub raw_view: wgpu::TextureView,
}

struct PendingUpload {
    texture_id: AtlasTextureId,
    bounds: Bounds<DevicePixels>,
    buffer: wgpu::Buffer,
    offset: u64,
}
