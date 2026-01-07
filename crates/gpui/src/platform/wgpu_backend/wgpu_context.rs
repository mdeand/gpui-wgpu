pub struct WgpuContext {
    pub(super) adapter: wgpu::Adapter,
    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    pub(super) instance: wgpu::Instance,

    pub(super) globals_buffer: wgpu::Buffer,
    pub(super) quads_buffer: wgpu::Buffer,
    pub(super) mono_sprites_buffer: wgpu::Buffer,
    pub(super) color_adjustments_buffer: wgpu::Buffer,

    // pub(super) sprite_texture_view: wgpu::TextureView,
    // pub(super) sprite_sampler: wgpu::Sampler,
    // pub(super) poly_sprites_buffer: wgpu::Buffer,
}

impl WgpuContext {
    pub fn new() -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))?;

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::BUFFER_BINDING_ARRAY
                    | wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY
                    | wgpu::Features::TEXTURE_BINDING_ARRAY,
                required_limits: wgpu::Limits {
                    max_texture_dimension_1d: 2048, // *
                    max_texture_dimension_2d: 2048, // *
                    max_texture_dimension_3d: 256,  // *
                    max_texture_array_layers: 256,
                    max_bind_groups: 4,
                    max_bindings_per_bind_group: 1000,
                    max_dynamic_uniform_buffers_per_pipeline_layout: 8,
                    max_dynamic_storage_buffers_per_pipeline_layout: 4,
                    max_sampled_textures_per_shader_stage: 16,
                    max_samplers_per_shader_stage: 16,
                    max_storage_buffers_per_shader_stage: 4, // *
                    max_storage_textures_per_shader_stage: 4,
                    max_uniform_buffers_per_shader_stage: 12,
                    max_binding_array_elements_per_shader_stage: 2,
                    max_binding_array_sampler_elements_per_shader_stage: 2,
                    max_uniform_buffer_binding_size: 16 << 10, // * (16 KiB)
                    max_storage_buffer_binding_size: 128 << 20, // (128 MiB)
                    max_vertex_buffers: 8,
                    max_vertex_attributes: 32,
                    max_vertex_buffer_array_stride: 2048,
                    min_subgroup_size: 0,
                    max_subgroup_size: 0,
                    max_push_constant_size: 0,
                    min_uniform_buffer_offset_alignment: 256,
                    min_storage_buffer_offset_alignment: 256,
                    max_inter_stage_shader_components: 60,
                    max_color_attachments: 4,
                    max_color_attachment_bytes_per_sample: 32,
                    max_compute_workgroup_storage_size: 16352, // *
                    max_compute_invocations_per_workgroup: 256,
                    max_compute_workgroup_size_x: 256,
                    max_compute_workgroup_size_y: 256,
                    max_compute_workgroup_size_z: 64,
                    max_compute_workgroups_per_dimension: 65535,
                    max_buffer_size: 256 << 20, // (256 MiB)
                    max_non_sampler_bindings: 1_000_000,
                },
                ..Default::default()
            }))?;

        let globals_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Globals Buffer"),
            // FIXME(mdeand): Hack
            size: 16 as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let quads_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Quads Buffer"),
            // TODO(mdeand): Determine appropriate size
            size: 1024 * 1024, // 1 MB buffer for quads, for now. (:
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let mono_sprites_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Monosprites Buffer"),
            // TODO(mdeand): Determine appropriate size, or make resizable.
            size: 1024 * 1024,
            usage: wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        let color_adjustments_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Color Adjustments Buffer"),
            size: 1024 * 16, // TODO(mdeand): 16 KB buffer for color adjustments, for now. (:
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        Ok(Self {
            adapter,
            device,
            queue,
            instance,

            globals_buffer,
            quads_buffer,
            mono_sprites_buffer,
            color_adjustments_buffer,
        })
    }
}
