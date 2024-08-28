use log::info;
use pollster::FutureExt;
use wgpu::*;
use winit::{dpi::PhysicalSize, window::Window};

use std::marker::PhantomData;
use std::sync::Arc;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    point: [f32; 3],
    // color: [f32; 3],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            point: [x, y, 0.0],
            // color: [1.0, 0.0, 0.0],
        }
    }

    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                // VertexAttribute {
                //     offset: std::mem::size_of::<[f32; 3]>() as BufferAddress,
                //     shader_location: 1,
                //     format: VertexFormat::Float32x3,
                // },
            ],
        }
    }
}

struct Buffers<T> {
    vertex: Buffer,
    index: Buffer,
    _element: PhantomData<T>,
}

impl<T> Buffers<T> {
    pub fn new(device: &Device, elements: usize, indexes: u64) -> Self {
        let mask = COPY_BUFFER_ALIGNMENT - 1;

        // Vertex buffer.

        let vertex = device.create_buffer(&BufferDescriptor {
            label: Some("base vertex"),
            size: elements as u64 * std::mem::size_of::<Self>() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        // Index buffer.
        let size = std::mem::size_of::<u16>() as u64 * indexes;
        let size = ((size + mask) & !mask).max(COPY_BUFFER_ALIGNMENT);

        let index = device.create_buffer(&BufferDescriptor {
            label: Some("base index"),
            size,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });

        Buffers {
            vertex,
            index,
            _element: PhantomData,
        }
    }

    pub fn buffer_vertex(&self, queue: &Queue, data: &[Vertex]) {
        // self.vertex.slice(..).map_async(MapMode::Write, |result| {
        //     if result.is_ok() {

        //     }
        // });
        queue.write_buffer(&self.vertex, 0, bytemuck::cast_slice(data));
    }

    pub fn buffer_index(&self, queue: &Queue, data: &[u16]) {
        queue.write_buffer(&self.index, 0, bytemuck::cast_slice(data));
    }

    pub fn vertex(&self) -> &Buffer {
        &self.vertex
    }

    pub fn index(&self) -> &Buffer {
        &self.index
    }

    pub fn flush(&self, queue: &Queue) {
        queue.submit([]);
        self.vertex.unmap();
        self.index.unmap();
    }
}

struct Pipeline {
    pipeline: RenderPipeline,
    shader: ShaderModule,
    buffers: Option<Buffers<Vertex>>,
}

impl Pipeline {
    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    pub fn buffers(&self) -> &Buffers<Vertex> {
        self.buffers.as_ref().unwrap()
    }
}

impl Pipeline {
    fn color(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("color.wgsl"));

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor::default())),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(format.into())],
                compilation_options: Default::default(),
            }),
            label: None,
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            shader,
            pipeline,
            buffers: None,
        }
    }

    fn base(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("base.wgsl"));
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor::default())),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::buffer_layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(format.into())],
                compilation_options: Default::default(),
            }),
            label: None,
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            shader,
            pipeline,
            buffers: Some(Buffers::new(device, 3, 6)),
        }
    }
}

/// Render global state.
#[allow(dead_code)]
pub struct Renderer {
    /// Rendering surface.
    surface: Surface<'static>,
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    pipelines: Vec<Pipeline>,
    config: SurfaceConfiguration,

    /// The window reference.
    /// Make sure this is last so it gets dropped after the surface
    /// reference is.
    window: Arc<Window>,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        let window = Arc::new(window);
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window.clone())
            .expect("wgpu surface");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .block_on()
            .expect("wgpu adapter");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .block_on()
            .expect("wgpu device and queue");

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("default config");

        surface.configure(&device, &config);

        let capabilities = surface.get_capabilities(&adapter);

        let format = capabilities.formats.get(0).expect("swapchain_format");

        let pipeline = Pipeline::color(&device, format.clone());
        let base = Pipeline::base(&device, format.clone());

        base.buffers().buffer_vertex(
            &queue,
            &[
                Vertex::new(-1.0, -1.0),
                Vertex::new(0., 0.),
                Vertex::new(1., 1.),
            ],
        );

        // base.buffers().buffer_index(&queue,
        //     &[0, 1, 2, 0, 1, 2],
        // );

        base.buffers().flush(&queue);

        Self {
            surface,
            window,
            instance,
            adapter,
            device,
            queue,
            pipelines: vec![pipeline, base],
            config,
        }
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        let mut config = self.config.clone();
        config.width = size.width.max(1);
        config.height = size.height.max(1);

        self.surface.configure(&self.device, &config);
        self.window.request_redraw();
    }

    pub fn draw(&self) {
        let frame = self.surface.get_current_texture().expect("texture");
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                label: Some("render pass"),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let pipeline = &self.pipelines[1];
            let pipe = pipeline.pipeline();
            rpass.set_pipeline(pipe);
            // rpass.set_index_buffer(pipeline.buffers().index().slice(..), IndexFormat::Uint16);
            rpass.set_vertex_buffer(0, pipeline.buffers().vertex().slice(..));
            // rpass.draw_indexed(0..3, 0, 0..1);
            rpass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();
        if let Some(false) = self.window.is_visible() {
            self.window.set_visible(true);
            self.window.request_redraw();
        }
    }
}
