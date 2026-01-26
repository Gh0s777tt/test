use wgpu::*;
use super::{context::GpuContext, pipeline::create_pipeline};

pub fn render(ctx: &GpuContext) {
    let frame = ctx.surface.get_current_texture().unwrap();
    let view = frame.texture.create_view(&TextureViewDescriptor::default());

    let pipeline = create_pipeline(&ctx.device, ctx.config.format);

    let mut encoder = ctx.device.create_command_encoder(&CommandEncoderDescriptor::default());

    {
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&pipeline);
        pass.draw(0..3, 0..1);
    }

    ctx.queue.submit(Some(encoder.finish()));
    frame.present();
}
