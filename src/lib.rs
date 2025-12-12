use gpui::prelude::*;
use gpui::{
    div, px, rgb, AnyElement, App, Application, Bounds, Context, Div,
    SharedString, Size, WindowBounds, WindowOptions,
};
use std::ffi::{c_char, c_void, CStr};

// --- FFI Types ---

#[repr(C)]
pub struct AppRef(*mut c_void);

// Wrappers to allow "in-place" mutation via standard FFI calls without returning new pointers
pub struct DivWrapper(Option<Div>);
pub struct TextWrapper(SharedString);

// --- Entry Points ---

#[no_mangle]
pub extern "C" fn run_app(on_init: extern "C" fn(AppRef)) {
    Application::new().run(move |cx: &mut App| {
        let app_ptr = cx as *mut App as *mut c_void;
        on_init(AppRef(app_ptr));
    });
}

#[no_mangle]
pub extern "C" fn open_window(app_ref: AppRef, load_view: extern "C" fn() -> *mut c_void) {
    let cx = unsafe { &mut *(app_ref.0 as *mut App) };

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
    Box::into_raw(Box::new(DivWrapper(Some(div())))) as *mut DivWrapper
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
