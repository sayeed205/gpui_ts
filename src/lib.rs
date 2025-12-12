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
pub extern "C" fn create_text(copy: *const c_char) -> *mut c_void {
    // For simplicity, directly returning ElementRef for text,
    // as we don't have a separate builder for text properties yet (usually done on parent or via attributes).
    // GPUI text usually inherits or is just a string child?
    // Wait, `div().child("foo")` works.
    // If we want a standalone text element (like `div().child(text_element)`),
    // GPUI doesn't have a standalone `Text` element struct in the same way, mostly it's strings or params.
    // But we can wrap it in a div or just return a SharedString if we had a generic child adder.
    // `div_child` takes `ElementRef`.
    // We can make a "text element" be a `div` containing text or just use `div_child_text`.
    // Let's rely on `div_child_text` for now for simplicity,
    // OR create a wrapper around `SharedString` if we really need independent text nodes.
    // But `div().child(IntoElement)`... `SharedString` impls `IntoElement`.
    // `IntoElement::into_any_element(string)` works.

    if copy.is_null() { return std::ptr::null_mut(); }
    let c_str = unsafe { CStr::from_ptr(copy) };
    let s = c_str.to_string_lossy().to_string();
    let ss: SharedString = s.into();
    let any: AnyElement = ss.into_any_element();
    Box::into_raw(Box::new(any)) as *mut c_void
}
