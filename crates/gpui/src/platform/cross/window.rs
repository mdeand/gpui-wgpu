use crate::{
    Bounds, Capslock, DisplayId, ForegroundExecutor, Modifiers, Pixels, PlatformAtlas,
    PlatformDisplay, PlatformInputHandler, PlatformWindow, Point, PromptButton, PromptLevel,
    RequestFrameOptions, Size, WindowAppearance, WindowBackgroundAppearance, WindowBounds,
    platform::{cross::display::WinitDisplay, cross::renderer::WgpuRenderer},
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

struct Callbacks {
    on_request_frame: Cell<Option<Box<dyn FnMut(crate::RequestFrameOptions)>>>,
    on_input: Cell<Option<Box<dyn FnMut(crate::PlatformInput) -> crate::DispatchEventResult>>>,
    on_active_status_change: Cell<Option<Box<dyn FnMut(bool)>>>,
    on_hover_status_change: Cell<Option<Box<dyn FnMut(bool)>>>,
    on_resize: Cell<Option<Box<dyn FnMut(crate::Size<crate::Pixels>, f32)>>>,
    on_moved: Cell<Option<Box<dyn FnMut()>>>,
    on_should_close: Cell<Option<Box<dyn FnMut() -> bool>>>,
    on_hit_test_window_control: Cell<Option<Box<dyn FnMut() -> Option<crate::WindowControlArea>>>>,
    on_close: Cell<Option<Box<dyn FnOnce()>>>,
    on_appearance_changed: Cell<Option<Box<dyn FnMut()>>>,
}

struct Handlers {
    input_handler: Cell<Option<PlatformInputHandler>>,
}

pub struct WinitWindowCreationInfo {
    pub(crate) executor: ForegroundExecutor,
}

pub struct WinitWindowParams {
    display_id: Option<DisplayId>,
}

pub struct WinitWindow {
    window: winit::window::Window,
    display: RefCell<WinitDisplay>,
    renderer: WgpuRenderer,
    callbacks: Callbacks,
    handlers: Handlers,
}

impl WinitWindow {}

impl raw_window_handle::HasDisplayHandle for WinitWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.window.display_handle()
    }
}

impl raw_window_handle::HasWindowHandle for WinitWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.window.window_handle()
    }
}

impl PlatformWindow for WinitWindow {
    fn bounds(&self) -> Bounds<crate::Pixels> {
        let size = self.window.inner_size();

        Bounds {
            // TODO(mdeand): Should this be the outer size instead of the inner size?
            // TODO(mdeand): Should this be the position of the window instead of (0, 0)?
            origin: Point {
                x: Pixels(0.),
                y: Pixels(0.),
            },
            size: Size {
                width: Pixels(size.width as f32),
                height: Pixels(size.height as f32),
            },
        }
    }

    fn is_maximized(&self) -> bool {
        self.window.is_maximized()
    }

    fn window_bounds(&self) -> WindowBounds {
        let bounds = self.bounds();

        if let Some(_fullscreen) = self.window.fullscreen() {
            return WindowBounds::Fullscreen(bounds);
        }

        if self.window.is_maximized() {
            return WindowBounds::Maximized(bounds);
        }

        WindowBounds::Windowed(bounds)
    }

    fn content_size(&self) -> crate::Size<crate::Pixels> {
        let size = self.window.inner_size();

        crate::Size {
            width: Pixels(size.width as f32),
            height: Pixels(size.height as f32),
        }
    }

    fn resize(&mut self, size: crate::Size<crate::Pixels>) {
        let _ =
            self.window
                .request_inner_size(winit::dpi::Size::Logical(winit::dpi::LogicalSize {
                    width: size.width.0 as f64,
                    height: size.height.0 as f64,
                }));
    }

    fn scale_factor(&self) -> f32 {
        self.window.scale_factor() as f32
    }

    fn appearance(&self) -> WindowAppearance {
        match self.window.theme() {
            Some(winit::window::Theme::Light) => WindowAppearance::Light,
            Some(winit::window::Theme::Dark) => WindowAppearance::Dark,
            // TODO(mdeand): This is *probably* bad.
            None => WindowAppearance::default(),
        }
    }

    fn display(&self) -> Option<std::rc::Rc<dyn PlatformDisplay>> {
        Some(Rc::new(self.display.borrow().clone()))
    }

    fn mouse_position(&self) -> crate::Point<crate::Pixels> {
        todo!()
    }

    fn modifiers(&self) -> Modifiers {
        todo!()
    }

    fn capslock(&self) -> Capslock {
        todo!()
    }

    fn set_input_handler(&mut self, input_handler: PlatformInputHandler) {
        self.handlers.input_handler.set(Some(input_handler));
    }

    fn take_input_handler(&mut self) -> Option<PlatformInputHandler> {
        self.handlers.input_handler.take()
    }

    fn prompt(
        &self,
        level: PromptLevel,
        msg: &str,
        detail: Option<&str>,
        answers: &[PromptButton],
    ) -> Option<futures::channel::oneshot::Receiver<usize>> {
        todo!()
    }

    fn activate(&self) {
        self.window.focus_window();
    }

    fn is_active(&self) -> bool {
        self.window.has_focus()
    }

    fn is_hovered(&self) -> bool {
        todo!()
    }

    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    fn set_background_appearance(&self, background_appearance: WindowBackgroundAppearance) {
        todo!()
    }

    fn minimize(&self) {
        self.window.set_minimized(true);
    }

    fn zoom(&self) {
        self.window.set_maximized(!self.window.is_maximized());
    }

    fn toggle_fullscreen(&self) {
        self.window
            .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
    }

    fn is_fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    fn on_request_frame(&self, callback: Box<dyn FnMut(RequestFrameOptions)>) {
        self.callbacks.on_request_frame.set(Some(callback));
    }

    fn on_input(
        &self,
        callback: Box<dyn FnMut(crate::PlatformInput) -> crate::DispatchEventResult>,
    ) {
        self.callbacks.on_input.set(Some(callback));
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.on_active_status_change.set(Some(callback));
    }

    fn on_hover_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.callbacks.on_hover_status_change.set(Some(callback));
    }

    fn on_resize(&self, callback: Box<dyn FnMut(crate::Size<crate::Pixels>, f32)>) {
        self.callbacks.on_resize.set(Some(callback));
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.on_moved.set(Some(callback));
    }

    fn on_should_close(&self, callback: Box<dyn FnMut() -> bool>) {
        self.callbacks.on_should_close.set(Some(callback));
    }

    fn on_hit_test_window_control(
        &self,
        callback: Box<dyn FnMut() -> Option<crate::WindowControlArea>>,
    ) {
        self.callbacks
            .on_hit_test_window_control
            .set(Some(callback));
    }

    fn on_close(&self, callback: Box<dyn FnOnce()>) {
        self.callbacks.on_close.set(Some(callback));
    }

    fn on_appearance_changed(&self, callback: Box<dyn FnMut()>) {
        self.callbacks.on_appearance_changed.set(Some(callback));
    }

    fn draw(&self, scene: &crate::Scene) {
        self.renderer.draw(scene);
    }

    fn sprite_atlas(&self) -> std::sync::Arc<dyn PlatformAtlas> {
        self.renderer.sprite_atlas()
    }

    #[cfg(target_os = "windows")]
    fn get_raw_handle(&self) -> gpui::platform::windows::HWND {
        unimplemented!()
    }

    fn gpu_specs(&self) -> Option<crate::GpuSpecs> {
        // TODO(mdeand): Implement this properly.
        None
    }

    fn update_ime_position(&self, _bounds: crate::Bounds<crate::Pixels>) {
        todo!()
    }
}
