import { dlopen, FFIType, suffix } from "bun:ffi";


declare const IS_STANDALONE: boolean;

let libDir = 'target/debug';
if (typeof IS_STANDALONE !== "undefined" && IS_STANDALONE) {
    libDir = 'target/release';
}
const path = `${libDir}/libgpui.${suffix}`


export const lib = dlopen(path, {
    run_app: {
        args: [FFIType.function],
        returns: FFIType.void,
    },
    open_window: {
        // AppRef is a pointer
        // load_view is a function pointer
        args: [FFIType.ptr, FFIType.ptr, FFIType.function],
        returns: FFIType.void,
    },
    create_div: { args: [], returns: FFIType.ptr },
    div_flex: { args: [FFIType.ptr], returns: FFIType.void },
    div_flex_col: { args: [FFIType.ptr], returns: FFIType.void },
    div_size_full: { args: [FFIType.ptr], returns: FFIType.void },
    div_w_px: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_h_px: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_size_px: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_p_px: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_m_px: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_gap_px: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_justify_center: { args: [FFIType.ptr], returns: FFIType.void },
    div_justify_between: { args: [FFIType.ptr], returns: FFIType.void },
    div_items_center: { args: [FFIType.ptr], returns: FFIType.void },
    div_items_start: { args: [FFIType.ptr], returns: FFIType.void },
    div_items_end: { args: [FFIType.ptr], returns: FFIType.void },
    div_bg: { args: [FFIType.ptr, FFIType.u32], returns: FFIType.void },
    div_text_color: { args: [FFIType.ptr, FFIType.u32], returns: FFIType.void },
    // Text Styling
    div_text_size: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_line_height: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    div_font_family: { args: [FFIType.ptr, FFIType.ptr], returns: FFIType.void },
    div_font_weight: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },

    div_child_text: { args: [FFIType.ptr, FFIType.ptr], returns: FFIType.void },
    div_child: { args: [FFIType.ptr, FFIType.ptr], returns: FFIType.void },
    into_element: { args: [FFIType.ptr], returns: FFIType.ptr },
    create_text: { args: [FFIType.ptr], returns: FFIType.ptr },

    // SVG
    create_svg: { args: [], returns: FFIType.ptr },
    svg_path: { args: [FFIType.ptr, FFIType.ptr], returns: FFIType.void },
    svg_size: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    svg_text_color: { args: [FFIType.ptr, FFIType.u32], returns: FFIType.void },
    svg_into_element: { args: [FFIType.ptr], returns: FFIType.ptr },

    // Animation / Scroll
    svg_rotate: { args: [FFIType.ptr, FFIType.f32], returns: FFIType.void },
    svg_with_animation: {
        args: [
            FFIType.ptr,
            FFIType.ptr,
            FFIType.f32,
            FFIType.bool,
            FFIType.function
        ],
        returns: FFIType.void
    },
});
