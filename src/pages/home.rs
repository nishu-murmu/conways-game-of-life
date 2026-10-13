use leptos::prelude::*;
use web_sys::{MouseEvent, WheelEvent};

#[component]
pub fn Home() -> impl IntoView {
    let (is_dragging, set_is_dragging) = signal(false);
    let (start_x, set_start_x) = signal(0.0);
    let (start_y, set_start_y) = signal(0.0);
    let (offset_x, set_offset_x) = signal(0.0);
    let (offset_y, set_offset_y) = signal(0.0);
    let (scale, set_scale) = signal(1.0);

    let (cursor_cell_x, set_cursor_cell_x) = signal(0i64);
    let (cursor_cell_y, set_cursor_cell_y) = signal(0i64);

    let base_grid_size = 40.0;

    let on_mousedown = move |e: MouseEvent| {
        set_is_dragging.set(true);
        set_start_x.set(e.client_x() as f64 - offset_x.get());
        set_start_y.set(e.client_y() as f64 - offset_y.get());
    };

    let on_mousemove = move |e: MouseEvent| {
        let mx = e.client_x() as f64;
        let my = e.client_y() as f64;

        let cell_size = base_grid_size * scale.get();
        let gx = ((mx - offset_x.get()) / cell_size).floor() as i64;
        let gy = ((my - offset_y.get()) / cell_size).floor() as i64;
        set_cursor_cell_x.set(gx);
        set_cursor_cell_y.set(gy);

        if is_dragging.get() {
            set_offset_x.set(mx - start_x.get());
            set_offset_y.set(my - start_y.get());
        }
    };

    let on_mouseup = move |_e: MouseEvent| {
        set_is_dragging.set(false);
    };

    let on_mouseleave = move |_e: MouseEvent| {
        set_is_dragging.set(false);
    };

    let on_wheel = move |e: WheelEvent| {
        e.prevent_default();

        let current_scale = scale.get();

        let zoom_factor = 1.1;
        let next_scale = if e.delta_y() < 0.0 {
            let s = current_scale * zoom_factor;
            if s > 5.0 { 5.0 } else { s }
        } else {
            let s = current_scale / zoom_factor;
            if s < 0.2 { 0.2 } else { s }
        };

        let mouse_x = e.client_x() as f64;
        let mouse_y = e.client_y() as f64;

        let dx = mouse_x - offset_x.get();
        let dy = mouse_y - offset_y.get();

        set_offset_x.set(mouse_x - dx * (next_scale / current_scale));
        set_offset_y.set(mouse_y - dy * (next_scale / current_scale));
        set_scale.set(next_scale);
    };

    let zoom_in = move |_| {
        let current_scale = scale.get();
        let next_scale = (current_scale * 1.2).min(5.0);
        set_scale.set(next_scale);
    };

    let zoom_out = move |_| {
        let current_scale = scale.get();
        let next_scale = (current_scale / 1.2).max(0.2);
        set_scale.set(next_scale);
    };

    let reset_view = move |_| {
        set_offset_x.set(0.0);
        set_offset_y.set(0.0);
        set_scale.set(1.0);
    };

    view! {
        <div class="canvas-container">
            // Infinite Grid Canvas
            <div
                id="canvas"
                on:mousedown=on_mousedown
                on:mouseup=on_mouseup
                on:mouseleave=on_mouseleave
                on:mousemove=on_mousemove
                on:wheel=on_wheel
                style=move || format!(
                    "background-position: {}px {}px; background-size: {}px {}px;",
                    offset_x.get(),
                    offset_y.get(),
                    base_grid_size * scale.get(),
                    base_grid_size * scale.get()
                )
            ></div>

            // Top HUD
            <div class="hud-top">
                <h1 class="app-title">"Conway's Game of Life"</h1>
                <div id="coords">
                    <span>{move || format!("Cell: ({}, {})", cursor_cell_x.get(), cursor_cell_y.get())}</span>
                    <span class="hud-divider">"•"</span>
                    <span>{move || format!("Zoom: {:.0}%", scale.get() * 100.0)}</span>
                </div>
            </div>

            // Instruction badge
            <div class="hud-instructions">
                "Drag to pan • Scroll to zoom"
            </div>

            // Controls
            <div class="hud-controls">
                <button class="hud-btn" on:click=zoom_in title="Zoom in">"+"</button>
                <button class="hud-btn" on:click=zoom_out title="Zoom out">"−"</button>
                <button class="hud-btn" on:click=reset_view title="Reset view">"Reset"</button>
            </div>
        </div>
    }
}
