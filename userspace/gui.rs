use horizon::{compositor::Compositor, window::Window};

fn main() {
    let mut compositor = Compositor {
        windows: vec![
            Window { x: 50, y: 80, w: 400, h: 300, title: "Terminal" },
        ],
    };

    loop {
        // poll input
        // redraw
        // sleep
    }
}
