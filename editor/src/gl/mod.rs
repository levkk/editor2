use pollster::FutureExt;
use wgpu::*;
use winit::window::Window;

struct Pipeline {
    pipeline: RenderPipeline,
    shader: ShaderModule,
}

impl Pipeline {
    pub fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }
}

impl Pipeline {
    fn color(device: &Device, format: TextureFormat) -> Self {
        let shader = device.create_shader_module(include_wgsl!("color.wgsl"));

        Self::new(device, shader, format)
    }

    fn new(device: &Device, shader: ShaderModule, format: TextureFormat) -> Self {
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

        Self { shader, pipeline }
    }
}

/// Render global state.
pub struct Renderer<'window> {
    /// Rendering surface.
    surface: Surface<'window>,
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    pipelines: Vec<Pipeline>,

    /// The window reference.
    /// Make sure this is last so it gets dropped after the surface
    /// reference is.
    window: &'window Window,
}

impl<'window> Renderer<'window> {
    pub fn new(window: &'window Window) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).expect("wgpu surface");

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

        Self {
            surface,
            window,
            instance,
            adapter,
            device,
            queue,
            pipelines: vec![pipeline],
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
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
                        load: LoadOp::Clear(Color::GREEN),
                        store: StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            let pipeline = &self.pipelines[0];
            let pipe = pipeline.pipeline();
            rpass.set_pipeline(pipe);
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
