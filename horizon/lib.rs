#[path = "mod.rs"]
pub mod horizon;

#[cfg(test)]
mod tests {
    use super::horizon::compositor::Compositor;
    use super::horizon::framebuffer::Framebuffer;
    use super::horizon::window::Window;

    #[test]
    fn test_compositor_draws_window_pixels() {
        let storage = vec![0u32; 64].into_boxed_slice();
        let fb_mem: &'static mut [u32] = Box::leak(storage);
        let mut fb = Framebuffer {
            width: 8,
            height: 8,
            buffer: fb_mem,
        };

        let comp = Compositor {
            windows: vec![Window {
                x: 1,
                y: 1,
                w: 2,
                h: 2,
                title: "demo",
            }],
        };
        comp.draw(&mut fb);

        assert_eq!(fb.buffer[0], 0xFF1E222A);
        assert_eq!(fb.buffer[1 + 8], 0xFF2E3440);
    }
}
