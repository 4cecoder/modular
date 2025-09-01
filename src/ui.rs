//! UI system module
//!
//! A small, robust UI kit with Buttons and Labels suitable for demos.
//! It's intentionally lightweight and uses the engine's `renderer_2d` and
//! `input_window` types so it stays decoupled from platform-specific code.

use crate::{renderer_2d, Vec2};
use std::collections::HashMap;

/// Events emitted by the UI
#[derive(Debug, Clone)]
pub enum UiEvent {
    Click(String),
}

/// Simple label widget
#[derive(Debug, Clone)]
pub struct Label {
    pub id: String,
    pub text: String,
    pub position: Vec2,
}

impl Label {
    pub fn new(id: &str, text: &str, position: Vec2) -> Self {
        Self {
            id: id.to_string(),
            text: text.to_string(),
            position,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

/// Simple button widget
pub struct Button {
    pub id: String,
    pub text: String,
    pub position: Vec2,
    pub size: Vec2,
    pub enabled: bool,
    // click callback is optional and stored as FnMut
    on_click: Option<Box<dyn FnMut()>>,
    // transient UI state
    hovered: bool,
    pressed: bool,
}

impl std::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("id", &self.id)
            .field("text", &self.text)
            .field("position", &self.position)
            .field("size", &self.size)
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl Button {
    pub fn new(id: &str, text: &str, position: Vec2, size: Vec2) -> Self {
        Self {
            id: id.to_string(),
            text: text.to_string(),
            position,
            size,
            enabled: true,
            on_click: None,
            hovered: false,
            pressed: false,
        }
    }

    /// Set click callback (consumes and returns self to allow easy chaining)
    pub fn on_click(mut self, cb: Box<dyn FnMut()>) -> Self {
        self.on_click = Some(cb);
        self
    }

    fn contains_point(&self, x: i32, y: i32) -> bool {
        let px = self.position.x as i32;
        let py = self.position.y as i32;
        let w = self.size.x as i32;
        let h = self.size.y as i32;

        x >= px && x < px + w && y >= py && y < py + h
    }

    fn call_click(&mut self) {
        if let Some(cb) = &mut self.on_click {
            (cb)();
        }
    }
}

impl Clone for Button {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            text: self.text.clone(),
            position: self.position.clone(),
            size: self.size.clone(),
            enabled: self.enabled,
            on_click: None, // callbacks are not cloned
            hovered: false,
            pressed: false,
        }
    }
}

/// Toggle (checkbox) widget
pub struct Toggle {
    pub id: String,
    pub label: String,
    pub position: Vec2,
    pub checked: bool,
    pub enabled: bool,
    on_change: Option<Box<dyn FnMut(bool)>>,
}

impl std::fmt::Debug for Toggle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Toggle")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("position", &self.position)
            .field("checked", &self.checked)
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl Clone for Toggle {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            label: self.label.clone(),
            position: self.position.clone(),
            checked: self.checked,
            enabled: self.enabled,
            on_change: None,
        }
    }
}

impl Toggle {
    pub fn new(id: &str, label: &str, position: Vec2, initial: bool) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            position,
            checked: initial,
            enabled: true,
            on_change: None,
        }
    }

    pub fn on_change(mut self, cb: Box<dyn FnMut(bool)>) -> Self {
        self.on_change = Some(cb);
        self
    }

    fn call_change(&mut self, value: bool) {
        self.checked = value;
        if let Some(cb) = &mut self.on_change {
            (cb)(value);
        }
    }
}

/// Slider widget (horizontal)
pub struct Slider {
    pub id: String,
    pub position: Vec2,
    pub size: Vec2,
    pub min: f32,
    pub max: f32,
    pub value: f32,
    pub enabled: bool,
    on_change: Option<Box<dyn FnMut(f32)>>,
    // transient dragging state
    dragging: bool,
    // last emitted value used for coarse updates
    last_emitted: Option<f32>,
}

impl std::fmt::Debug for Slider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slider")
            .field("id", &self.id)
            .field("position", &self.position)
            .field("size", &self.size)
            .field("min", &self.min)
            .field("max", &self.max)
            .field("value", &self.value)
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl Clone for Slider {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            position: self.position.clone(),
            size: self.size.clone(),
            min: self.min,
            max: self.max,
            value: self.value,
            enabled: self.enabled,
            on_change: None,
            dragging: false,
            last_emitted: Some(self.value),
        }
    }
}

impl Slider {
    pub fn new(id: &str, position: Vec2, size: Vec2, min: f32, max: f32, initial: f32) -> Self {
        Self {
            id: id.to_string(),
            position,
            size,
            min,
            max,
            value: initial.clamp(min, max),
            enabled: true,
            on_change: None,
            dragging: false,
            last_emitted: None,
        }
    }

    pub fn on_change(mut self, cb: Box<dyn FnMut(f32)>) -> Self {
        self.on_change = Some(cb);
        self
    }

    fn set_value(&mut self, v: f32) {
        let v = v.clamp(self.min, self.max);
        self.value = v;
        // emit only when change is significant (coarse) to reduce spam
        let emit_delta = 0.01; // 1%
        let should_emit = match self.last_emitted {
            Some(prev) => (prev - v).abs() >= emit_delta,
            None => true,
        };
        if should_emit {
            if let Some(cb) = &mut self.on_change {
                (cb)(v);
            }
            self.last_emitted = Some(v);
        }
    }

    fn knob_rect(&self) -> (i32, i32, i32, i32) {
        let x = self.position.x as i32;
        let y = self.position.y as i32;
        let w = self.size.x as i32;
        let h = self.size.y as i32;
        let t = ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0);
        let knob_x = x + (t * (w as f32)) as i32 - 6;
        (knob_x, y, 12, h)
    }
}

/// Widget enum stores possible widget types
#[derive(Debug, Clone)]
pub enum Widget {
    Button(Button),
    Label(Label),
    Toggle(Toggle),
    Slider(Slider),
}

/// UIManager manages widgets, input handling, layout and rendering
use minifb::Key;

pub struct UIManager {
    widgets: Vec<Widget>,
    // fast lookup by id -> index in widgets
    index_by_id: HashMap<String, usize>,
    pub theme: Theme,
    /// index of focused widget (if any)
    focus_index: Option<usize>,
}
impl UIManager {
    /// Bring widget with id to front (render and hit-test order)
    pub fn bring_to_front(&mut self, id: &str) {
        if let Some(&idx) = self.index_by_id.get(id) {
            if idx + 1 == self.widgets.len() {
                return; // already front
            }
            let widget = self.widgets.remove(idx);
            self.widgets.push(widget);

            // rebuild index map
            self.index_by_id.clear();
            for (i, w) in self.widgets.iter().enumerate() {
                let wid = match w {
                    Widget::Button(b) => b.id.clone(),
                    Widget::Label(l) => l.id.clone(),
                    Widget::Toggle(t) => t.id.clone(),
                    Widget::Slider(s) => s.id.clone(),
                };
                self.index_by_id.insert(wid, i);
            }
        }
    }
}

impl UIManager {
    // ...existing methods above remain; we'll add new behavior in handle_input/render below
}

/// Simple UI theme for colors and sizes
#[derive(Debug, Clone)]
pub struct Theme {
    pub button_bg: renderer_2d::Color,
    pub button_bg_disabled: renderer_2d::Color,
    pub button_hover: renderer_2d::Color,
    pub button_pressed: renderer_2d::Color,
    pub text_color: renderer_2d::Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            button_bg: renderer_2d::Color::rgb(40, 40, 80),
            button_bg_disabled: renderer_2d::Color::rgb(60, 60, 60),
            button_hover: renderer_2d::Color::rgb(60, 60, 120),
            button_pressed: renderer_2d::Color::rgb(20, 20, 60),
            text_color: renderer_2d::Color::WHITE,
        }
    }
}

// (WidgetState removed; buttons track hovered/pressed directly)

impl Default for UIManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            index_by_id: HashMap::new(),
            theme: Theme::default(),
            focus_index: None,
        }
    }

    /// Add a generic widget
    pub fn add_widget(&mut self, widget: Widget) {
        let id = match &widget {
            Widget::Button(b) => b.id.clone(),
            Widget::Label(l) => l.id.clone(),
            Widget::Toggle(t) => t.id.clone(),
            Widget::Slider(s) => s.id.clone(),
        };
        let idx = self.widgets.len();
        self.widgets.push(widget);
        self.index_by_id.insert(id, idx);
        // If no widget focused yet, give focus to the first focusable widget
        if self.focus_index.is_none() {
            // find first button index
            for (i, w) in self.widgets.iter().enumerate() {
                match w {
                    Widget::Button(_) | Widget::Toggle(_) | Widget::Slider(_) => {
                        self.focus_index = Some(i);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
    /// Handle input and emit UI events. This will also call widget callbacks
    /// for clicks (Button.on_click). Provides hover & focus management and
    /// keyboard navigation (Tab / Shift+Tab + Enter/Space to activate).
    pub fn handle_input(&mut self, input: &crate::input_window::WindowInputState) -> Vec<UiEvent> {
        let mut events = Vec::new();

        let (mx, my) = input.mouse_pos();

        // Update hover/pressed/drag state for widgets
        for w in &mut self.widgets {
            match w {
                Widget::Button(btn) => {
                    let hover = btn.enabled && btn.contains_point(mx, my);
                    btn.hovered = hover;
                    // pressed state while left mouse button held
                    btn.pressed = hover
                        && input.is_mouse_button_pressed(crate::input_window::MouseButton::Left);
                }
                Widget::Label(_l) => {
                    // labels don't track hover
                }
                Widget::Toggle(_t) => {
                    // toggles are simple; no per-frame pressed state tracked here
                }
                Widget::Slider(s) => {
                    // if dragging, update value from mouse while left button held
                    if s.dragging
                        && input.is_mouse_button_pressed(crate::input_window::MouseButton::Left)
                    {
                        let x = s.position.x as f32;
                        let w = s.size.x as f32;
                        let ratio = ((mx as f32) - x) / w;
                        let val = s.min + ratio.clamp(0.0, 1.0) * (s.max - s.min);
                        s.set_value(val);
                    }
                }
            }
        }

        // Mouse click handling (top-most button)
        if input.is_mouse_button_just_pressed(crate::input_window::MouseButton::Left) {
            for i in (0..self.widgets.len()).rev() {
                match &mut self.widgets[i] {
                    Widget::Button(btn) => {
                        if btn.enabled && btn.contains_point(mx, my) {
                            btn.call_click();
                            events.push(UiEvent::Click(btn.id.clone()));
                            // set focus to clicked widget
                            self.focus_index = Some(i);
                            // bring to front so it's rendered on top
                            let id = btn.id.clone();
                            self.bring_to_front(&id);
                            break;
                        }
                    }
                    Widget::Toggle(t) => {
                        // toggle if clicked on box or label area
                        let bx = t.position.x as i32;
                        let by = t.position.y as i32;
                        let bw = 12;
                        let bh = 12;
                        let in_box = mx >= bx && mx < bx + bw && my >= by && my < by + bh;
                        let in_label = mx >= bx && mx < bx + 200 && my >= by && my < by + bh;
                        if t.enabled && (in_box || in_label) {
                            t.call_change(!t.checked);
                            events.push(UiEvent::Click(t.id.clone()));
                            self.focus_index = Some(i);
                            let id = t.id.clone();
                            self.bring_to_front(&id);
                            break;
                        }
                    }
                    Widget::Slider(s) => {
                        let sx = s.position.x as i32;
                        let sy = s.position.y as i32;
                        let sw = s.size.x as i32;
                        let sh = s.size.y as i32;
                        let (kx, ky, kw, kh) = s.knob_rect();
                        let in_bar = mx >= sx && mx < sx + sw && my >= sy && my < sy + sh;
                        let in_knob = mx >= kx && mx < kx + kw && my >= ky && my < ky + kh;
                        if s.enabled && (in_bar || in_knob) {
                            s.dragging = true;
                            // set value immediately
                            let x = s.position.x as f32;
                            let w = s.size.x as f32;
                            let ratio = ((mx as f32) - x) / w;
                            let val = s.min + ratio.clamp(0.0, 1.0) * (s.max - s.min);
                            s.set_value(val);
                            // focus and bring to front
                            self.focus_index = Some(i);
                            let id = s.id.clone();
                            self.bring_to_front(&id);
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }

        // If left mouse released this frame, stop dragging sliders
        if !input.is_mouse_button_pressed(crate::input_window::MouseButton::Left) {
            for w in &mut self.widgets {
                if let Widget::Slider(s) = w {
                    if s.dragging {
                        s.dragging = false;
                        // ensure final value emits even if below delta threshold
                        if let Some(cb) = &mut s.on_change {
                            // emit final regardless
                            (cb)(s.value);
                            s.last_emitted = Some(s.value);
                        }
                    }
                }
            }
        }

        // Keyboard navigation: Tab / Shift+Tab
        if input.is_key_just_pressed(Key::Tab) {
            let backwards =
                input.is_key_pressed(Key::LeftShift) || input.is_key_pressed(Key::RightShift);
            let mut start = self.focus_index.unwrap_or(0);
            let len = self.widgets.len();
            if len == 0 {
                return events;
            }
            // find next focusable widget (Button/Toggle/Slider)
            for _ in 0..len {
                start = if backwards {
                    if start == 0 {
                        len - 1
                    } else {
                        start - 1
                    }
                } else {
                    (start + 1) % len
                };

                match &self.widgets[start] {
                    Widget::Button(_) | Widget::Toggle(_) | Widget::Slider(_) => {
                        self.focus_index = Some(start);
                        break;
                    }
                    _ => {}
                }
            }
        }

        // Activation via keyboard
        if input.is_key_just_pressed(Key::Enter) || input.is_key_just_pressed(Key::Space) {
            if let Some(fi) = self.focus_index {
                if fi < self.widgets.len() {
                    match &mut self.widgets[fi] {
                        Widget::Button(btn) => {
                            if btn.enabled {
                                btn.call_click();
                                events.push(UiEvent::Click(btn.id.clone()));
                            }
                        }
                        Widget::Toggle(t) => {
                            if t.enabled {
                                t.call_change(!t.checked);
                                events.push(UiEvent::Click(t.id.clone()));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        events
    }

    /// Update UI (animations, etc). For now it's a no-op but kept for API completeness.
    pub fn update(&mut self, _delta_time: f32) {
        // placeholder for transitions/animations
    }

    /// Render all widgets using the provided renderer
    pub fn render(&self, renderer: &mut renderer_2d::Renderer2D) {
        for (i, widget) in self.widgets.iter().enumerate() {
            match widget {
                Widget::Button(btn) => {
                    let x = btn.position.x as i32;
                    let y = btn.position.y as i32;
                    let w = btn.size.x as i32;
                    let h = btn.size.y as i32;

                    let bg = if !btn.enabled {
                        self.theme.button_bg_disabled
                    } else if btn.pressed {
                        self.theme.button_pressed
                    } else if btn.hovered {
                        self.theme.button_hover
                    } else {
                        self.theme.button_bg
                    };

                    renderer.draw_rect(x, y, w, h, bg);
                    // border
                    renderer.draw_rect_outline(x, y, w, h, renderer_2d::Color::WHITE);

                    // focus outline if focused
                    if let Some(fi) = self.focus_index {
                        if fi == i {
                            renderer.draw_rect_outline(
                                x - 2,
                                y - 2,
                                w + 4,
                                h + 4,
                                renderer_2d::Color::YELLOW,
                            );
                        }
                    }

                    // text centered
                    let center_x = (x + w / 2) as usize;
                    let text_y = (y + h / 2 - 8) as usize;
                    renderer.draw_text_centered(
                        &btn.text,
                        center_x,
                        text_y,
                        self.theme.text_color,
                        1,
                    );
                }
                Widget::Label(lbl) => {
                    let x = lbl.position.x as usize;
                    let y = lbl.position.y as usize;
                    renderer.draw_text(&lbl.text, x, y, self.theme.text_color, 1);
                }
                Widget::Toggle(t) => {
                    // draw a box and label
                    let box_x = t.position.x as i32;
                    let box_y = t.position.y as i32;
                    let box_size = 12;
                    let bg = if !t.enabled {
                        self.theme.button_bg_disabled
                    } else if t.checked {
                        self.theme.button_pressed
                    } else {
                        self.theme.button_bg
                    };
                    renderer.draw_rect(box_x, box_y, box_size, box_size, bg);
                    renderer.draw_rect_outline(
                        box_x,
                        box_y,
                        box_size,
                        box_size,
                        renderer_2d::Color::WHITE,
                    );
                    // checkmark when checked
                    if t.checked {
                        // simple X mark
                        renderer.draw_text(
                            "X",
                            (box_x + 3) as usize,
                            box_y as usize,
                            renderer_2d::Color::WHITE,
                            1,
                        );
                    }
                    // label text to the right
                    renderer.draw_text(
                        &t.label,
                        (box_x + box_size + 4) as usize,
                        box_y as usize,
                        self.theme.text_color,
                        1,
                    );
                    // focus outline
                    if let Some(fi) = self.focus_index {
                        if fi == i {
                            renderer.draw_rect_outline(
                                box_x - 2,
                                box_y - 2,
                                box_size + 4,
                                box_size + 4,
                                renderer_2d::Color::YELLOW,
                            );
                        }
                    }
                }
                Widget::Slider(s) => {
                    let x = s.position.x as i32;
                    let y = s.position.y as i32;
                    let w = s.size.x as i32;
                    let h = s.size.y as i32;
                    // track background
                    renderer.draw_rect(x, y + h / 3, w, h / 3, self.theme.button_bg);
                    // knob
                    let (kx, ky, kw, kh) = s.knob_rect();
                    renderer.draw_rect(kx, ky, kw, kh, self.theme.button_hover);
                    renderer.draw_rect_outline(kx, ky, kw, kh, renderer_2d::Color::WHITE);
                    // focus outline for slider
                    if let Some(fi) = self.focus_index {
                        if fi == i {
                            renderer.draw_rect_outline(
                                x - 2,
                                y - 2,
                                w + 4,
                                h + 4,
                                renderer_2d::Color::YELLOW,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Mutable access to a label by id
    pub fn get_label_mut(&mut self, id: &str) -> Option<&mut Label> {
        if let Some(&idx) = self.index_by_id.get(id) {
            if let Widget::Label(lbl) = &mut self.widgets[idx] {
                return Some(lbl);
            }
        }
        None
    }

    /// Mutable access to a button by id
    pub fn get_button_mut(&mut self, id: &str) -> Option<&mut Button> {
        if let Some(&idx) = self.index_by_id.get(id) {
            if let Widget::Button(btn) = &mut self.widgets[idx] {
                return Some(btn);
            }
        }
        None
    }

    /// Immutable access to a toggle by id
    pub fn get_toggle(&self, id: &str) -> Option<&Toggle> {
        if let Some(&idx) = self.index_by_id.get(id) {
            if let Widget::Toggle(t) = &self.widgets[idx] {
                return Some(t);
            }
        }
        None
    }

    /// Immutable access to a slider by id
    pub fn get_slider(&self, id: &str) -> Option<&Slider> {
        if let Some(&idx) = self.index_by_id.get(id) {
            if let Widget::Slider(s) = &self.widgets[idx] {
                return Some(s);
            }
        }
        None
    }
}
