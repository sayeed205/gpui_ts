use gpui::prelude::*;
use gpui::{
    bounce, div, ease_in_out, percentage, px, rgb, svg, Animation, AnimationExt, AnyElement,
    App, Application, AssetSource, Bounds, Context, Div,
    SharedString, Size, Svg, TitlebarOptions, Transformation, WindowBounds, WindowDecorations,
    WindowOptions,
};
use std::ffi::{c_char, c_void, CStr};
use std::time::Duration;

// --- File Assets ---
struct FileAssets;

impl AssetSource for FileAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        Ok(Some(path.to_string().into_bytes().into()))
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

// --- FFI Types ---

#[repr(C)]
pub struct AppRef(*mut c_void);

// Wrappers to allow "in-place" mutation via standard FFI calls without returning new pointers
pub struct DivWrapper(Option<Div>);
pub struct SvgWrapper {
    inner: Option<Svg>,
    converted: Option<AnyElement>,
}
pub struct TextWrapper(#[allow(dead_code)] SharedString);

// --- Entry Points ---

#[no_mangle]
pub extern "C" fn run_app(on_init: extern "C" fn(AppRef)) {
    Application::new()
        .with_assets(FileAssets)
        .run(move |cx: &mut App| {
            let app_ptr = cx as *mut App as *mut c_void;
            on_init(AppRef(app_ptr));
        });
}

#[no_mangle]
pub extern "C" fn open_window(app_ref: AppRef, title: *const c_char, load_view: extern "C" fn() -> *mut c_void) {
    let cx = unsafe { &mut *(app_ref.0 as *mut App) };

    let title_str = if !title.is_null() {
        unsafe { CStr::from_ptr(title).to_string_lossy().into_owned() }
    } else {
        "GPUI App".to_string()
    };

    let bounds = Bounds::centered(
        None,
        Size {
            width: px(400.),
            height: px(300.),
        },
        cx,
    );
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::from(title_str)),
                ..Default::default()
            }),
            window_decorations: Some(WindowDecorations::Server),
            ..Default::default()
        },
        move |_, cx| {
            cx.new(move |_| FFIView { load_view })
        },
    )
        .unwrap();
    cx.activate(true);
}

struct FFIView {
    load_view: extern "C" fn() -> *mut c_void,
}

impl Render for FFIView {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let elem_ptr = (self.load_view)();

        if !elem_ptr.is_null() {
            unsafe { *Box::from_raw(elem_ptr as *mut AnyElement) }
        } else {
            div().child("Error: No element returned").into_any_element()
        }
    }
}

// --- Macros ---

macro_rules! impl_div_fn {
    ($name:ident, $method:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(ptr: *mut DivWrapper) {
            if ptr.is_null() { return; }
            let w = unsafe { &mut *ptr };
            if let Some(d) = w.0.take() {
                w.0 = Some(d.$method());
            }
        }
    };
}

macro_rules! impl_div_px {
    ($name:ident, $method:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(ptr: *mut DivWrapper, val: f32) {
            if ptr.is_null() { return; }
            let w = unsafe { &mut *ptr };
            if let Some(d) = w.0.take() {
                w.0 = Some(d.$method(px(val)));
            }
        }
    };
}

macro_rules! impl_div_color {
    ($name:ident, $method:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(ptr: *mut DivWrapper, hex: u32) {
            if ptr.is_null() { return; }
            let w = unsafe { &mut *ptr };
            if let Some(d) = w.0.take() {
                w.0 = Some(d.$method(rgb(hex)));
            }
        }
    };
}

// --- Element Builders ---

#[no_mangle]
pub extern "C" fn create_div() -> *mut DivWrapper {
    Box::into_raw(Box::new(DivWrapper(Some(div()))))
}

// Layout & Style
impl_div_fn!(div_flex, flex);
impl_div_fn!(div_flex_col, flex_col);
impl_div_fn!(div_size_full, size_full);
impl_div_fn!(div_justify_center, justify_center);
impl_div_fn!(div_justify_between, justify_between);
impl_div_fn!(div_items_center, items_center);
impl_div_fn!(div_items_start, items_start);
impl_div_fn!(div_items_end, items_end);

impl_div_px!(div_w_px, w);
impl_div_px!(div_h_px, h);
impl_div_px!(div_size_px, size);
impl_div_px!(div_p_px, p);
impl_div_px!(div_m_px, m);
impl_div_px!(div_gap_px, gap);

impl_div_color!(div_bg, bg);
impl_div_color!(div_text_color, text_color);


#[no_mangle]
pub extern "C" fn div_child_text(ptr: *mut DivWrapper, text: *const c_char) {
    let w = unsafe { &mut *ptr };
    let c_str = unsafe { CStr::from_ptr(text) };
    let s = c_str.to_string_lossy().to_string();

    if let Some(d) = w.0.take() {
        w.0 = Some(d.child(s));
    }
}

#[no_mangle]
pub extern "C" fn div_child(ptr: *mut DivWrapper, child_ptr: *mut c_void) {
    let w = unsafe { &mut *ptr };
    if !child_ptr.is_null() {
        let child = unsafe { *Box::from_raw(child_ptr as *mut AnyElement) };
        if let Some(d) = w.0.take() {
            w.0 = Some(d.child(child));
        }
    }
}

#[no_mangle]
pub extern "C" fn into_element(ptr: *mut DivWrapper) -> *mut c_void {
    if ptr.is_null() { return std::ptr::null_mut(); }
    let w = unsafe { Box::from_raw(ptr) };
    if let Some(d) = w.0 {
        let any: AnyElement = d.into_any_element();
        let ret = Box::into_raw(Box::new(any)) as *mut c_void;
        ret
    } else {
        std::ptr::null_mut()
    }
}


#[no_mangle]
pub extern "C" fn div_text_size(ptr: *mut DivWrapper, val: f32) {
    if ptr.is_null() { return; }
    let w = unsafe { &mut *ptr };
    if let Some(d) = w.0.take() {
        w.0 = Some(d.text_size(px(val)));
    }
}

#[no_mangle]
pub extern "C" fn div_line_height(ptr: *mut DivWrapper, val: f32) {
    if ptr.is_null() { return; }
    let w = unsafe { &mut *ptr };
    if let Some(d) = w.0.take() {
        w.0 = Some(d.line_height(px(val)));
    }
}

#[no_mangle]
pub extern "C" fn div_font_family(ptr: *mut DivWrapper, family: *const c_char) {
    if ptr.is_null() || family.is_null() { return; }
    let w = unsafe { &mut *ptr };
    let c_str = unsafe { CStr::from_ptr(family) };
    let s = c_str.to_string_lossy().to_string();
    if let Some(d) = w.0.take() {
        w.0 = Some(d.font_family(s));
    }
}

#[no_mangle]
pub extern "C" fn div_font_weight(ptr: *mut DivWrapper, weight: f32) {
    if ptr.is_null() { return; }
    let w = unsafe { &mut *ptr };
    if let Some(d) = w.0.take() {
        // gpui::FontWeight(f32)
        w.0 = Some(d.font_weight(gpui::FontWeight(weight)));
    }
}

#[no_mangle]
pub extern "C" fn create_text(copy: *const c_char) -> *mut c_void {
    if copy.is_null() { return std::ptr::null_mut(); }
    let c_str = unsafe { CStr::from_ptr(copy) };
    let s = c_str.to_string_lossy().to_string();
    let ss: SharedString = s.into();
    let any: AnyElement = ss.into_any_element();
    Box::into_raw(Box::new(any)) as *mut c_void
}

// --- SVG ---

#[no_mangle]
pub extern "C" fn create_svg() -> *mut SvgWrapper {
    Box::into_raw(Box::new(SvgWrapper {
        inner: Some(svg()),
        converted: None,
    }))
}

#[no_mangle]
pub extern "C" fn svg_path(ptr: *mut SvgWrapper, path: *const c_char) {
    if ptr.is_null() || path.is_null() { return; }
    let w = unsafe { &mut *ptr };
    let c_str = unsafe { CStr::from_ptr(path) };
    let s = c_str.to_string_lossy().to_string();
    if let Some(d) = w.inner.take() {
        w.inner = Some(d.path(s));
    }
}

#[no_mangle]
pub extern "C" fn svg_size(ptr: *mut SvgWrapper, size: f32) {
    if ptr.is_null() { return; }
    let w = unsafe { &mut *ptr };
    if let Some(d) = w.inner.take() {
        w.inner = Some(d.size(px(size)));
    }
}

#[no_mangle]
pub extern "C" fn svg_text_color(ptr: *mut SvgWrapper, color: u32) {
    if ptr.is_null() { return; }
    let w = unsafe { &mut *ptr };
    if let Some(d) = w.inner.take() {
        w.inner = Some(d.text_color(rgb(color)));
    }
}

#[no_mangle]
pub extern "C" fn svg_into_element(ptr: *mut SvgWrapper) -> *mut c_void {
    if ptr.is_null() { return std::ptr::null_mut(); }
    let w = unsafe { Box::from_raw(ptr) };
    if let Some(any) = w.converted {
        Box::into_raw(Box::new(any)) as *mut c_void
    } else if let Some(d) = w.inner {
        let any: AnyElement = d.into_any_element();
        Box::into_raw(Box::new(any)) as *mut c_void
    } else {
        std::ptr::null_mut()
    }
}

// div_overflow_y_scroll removed due to API mismatch

#[no_mangle]
pub extern "C" fn svg_rotate(ptr: *mut SvgWrapper, angle: f32) {
    if ptr.is_null() { return; }
    let w = unsafe { &mut *ptr };
    if let Some(d) = w.inner.take() {
        w.inner = Some(d.with_transformation(Transformation::rotate(percentage(angle))));
    }
}

#[no_mangle]
pub extern "C" fn svg_with_animation(
    ptr: *mut SvgWrapper,
    id: *const c_char,
    duration_secs: f32,
    repeat: bool,
    cb: extern "C" fn(*mut SvgWrapper, f32),
) {
    if ptr.is_null() || id.is_null() { return; }
    let w = unsafe { &mut *ptr };
    let c_str = unsafe { CStr::from_ptr(id) };
    let id_str = c_str.to_string_lossy().to_string();
    let id_ss = SharedString::from(id_str);

    if let Some(d) = w.inner.take() {
        let mut anim = Animation::new(Duration::from_secs_f32(duration_secs));
        if repeat {
            anim = anim.repeat();
        }
        // Hardcoded ease_in_out bounce for the example
        anim = anim.with_easing(bounce(ease_in_out));

        let animated = d.with_animation(
            id_ss,
            anim,
            move |svg, delta| {
                let mut wrapper = SvgWrapper { inner: Some(svg), converted: None };
                let wrapper_ptr = &mut wrapper as *mut SvgWrapper;
                cb(wrapper_ptr, delta);
                wrapper.inner.expect("Callback logic error")
            },
        );
        w.converted = Some(animated.into_any_element());
    }
}
