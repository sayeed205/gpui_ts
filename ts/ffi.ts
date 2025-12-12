import { dlopen, FFIType, suffix } from "bun:ffi";
import { join } from "node:path";
import { existsSync } from "node:fs";

function getLibPath(): string {
    const platform = process.platform;
    const arch = process.arch;

    // 1. Try Dist (Published)
    let osMap: Record<string, string> = {
        "linux": "linux",
        "darwin": "macos",
        "win32": "windows"
    };
    const os = osMap[platform] || platform;

    // We enforce 'libgpui-' prefix in our build workflow for consistency in dist/
    const distFile = `libgpui-${os}-${arch}.${suffix}`;
    const distPath = join(import.meta.dir, "..", "dist", distFile);

    if (existsSync(distPath)) return distPath;

    // 2. Try Local Target (Development)
    // Rust typically names: libgpui.so, libgpui.dylib, gpui.dll
    const names = [`libgpui.${suffix}`, `gpui.${suffix}`];
    const dirs = ["release", "debug"];

    for (const dir of dirs) {
        for (const name of names) {
            const candidate = join(import.meta.dir, "..", "target", dir, name);
            if (existsSync(candidate)) return candidate;
        }
    }

    throw new Error(`GPUI Library not found. Checked dist/ and target/ (debug/release). Platform: ${platform}-${arch}`);
}


export const lib = dlopen(getLibPath(), {
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
