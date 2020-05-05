use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

fn create_quad_pipeline(device: &wgpu::Device) -> (wgpu::RenderPipeline, fine::graphic::Binding) {
    let mut binding = fine::graphic::Binding::new();
    binding.build(device, None);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[binding.get_layout()],
    });

    let source = &include_bytes!("./quad.vert.spv")[..];
    let vertex_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(source)).unwrap());
    let source = &include_bytes!("./quad.frag.spv")[..];
    let fragment_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(source)).unwrap());

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vertex_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fragment_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Front,
            depth_bias: 0,
            depth_bias_clamp: 0.,
            depth_bias_slope_scale: 0.,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: fine::graphic::DEFAULT_TEXTURE_FORMAT,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: Vertex::STRIDE,
                attributes: Vertex::ATTRIBUTES,
                step_mode: wgpu::InputStepMode::Vertex,
            }],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    (pipeline, binding)
}

fn create_quad(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) -> Quad {
    let vertices: Vec<Vertex> = vec![
        vertex(-1.0, -1.0, 0., 0.0, 0.0),
        vertex(-1.0, 1.0, 0., 0.0, 1.0),
        vertex(1.0, 1.0, 0., 1.0, 1.0),
        vertex(1.0, -1.0, 0., 1.0, 0.0),
    ];

    let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

    let vertex_buffer = device.create_buffer_with_data(
        fine::bytemuck::cast_slice(&vertices),
        wgpu::BufferUsage::VERTEX,
    );

    let index_buffer = device.create_buffer_with_data(
        fine::bytemuck::cast_slice(&indices),
        wgpu::BufferUsage::INDEX,
    );

    let (pipeline, binding) = create_quad_pipeline(device);

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: binding.get_layout(),
        bindings: &[],
    });

    Quad {
        pipeline,
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
        bind_group,
    }
}

pub struct Quad {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
}

impl Quad {
    pub fn new(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) -> Self {
        create_quad(device, encoder)
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut pass = encoder.begin_render_pass(&fine::graphic::wgpu::RenderPassDescriptor {
            color_attachments: &[fine::graphic::wgpu::RenderPassColorAttachmentDescriptor {
                attachment: view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                },
            }],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_index_buffer(&self.index_buffer, 0, 0);
        pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
