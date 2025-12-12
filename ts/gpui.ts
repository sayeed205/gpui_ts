import { lib } from "./ffi";
import { readFileSync, existsSync } from "node:fs";
import { JSCallback, Pointer } from "bun:ffi";

export interface Element {
    intoElement(): Pointer | null;
}

export class Window {
    // Placeholder
}

export class App {
    static run(onInit: (app: Pointer | null) => void) {
        // Bun callback
        const cb = new JSCallback(
            (appPtr: Pointer | null) => {
                onInit(appPtr);
            },
            {
                args: ["ptr"],
                returns: "void",
            }
        );

        lib.symbols.run_app(cb.ptr);
        // Bun callbacks must be kept alive if multiple calls expected. 
        // run_app blocks so it might be fine, but safe to close after returns.
        cb.close();
    }

    static openWindow(appPtr: Pointer | null, title: string, viewBuilder: () => Div) {
        const cb = new JSCallback(
            () => {
                const div = viewBuilder();
                // Bun return pointers directly
                return div.intoElement();
            },
            {
                args: [],
                returns: "ptr",
            }
        );

        const titlePtr = Buffer.from(title + "\0");
        lib.symbols.open_window(appPtr, titlePtr, cb.ptr);

        // Keep alive for Bun too?
        // JSCallback in Bun is automatically GC'd if not held.
        // We must hold it.
        // @ts-ignore: global
        (globalThis as any)._keep_alive = (globalThis as any)._keep_alive || [];
        // @ts-ignore: global
        (globalThis as any)._keep_alive.push(cb);
    }
}

export class Div {
    ptr: Pointer | null;

    constructor() {
        this.ptr = lib.symbols.create_div();
    }

    flex(): this {
        lib.symbols.div_flex(this.ptr);
        return this;
    }

    flexCol(): this {
        lib.symbols.div_flex_col(this.ptr);
        return this;
    }

    sizeFull(): this {
        lib.symbols.div_size_full(this.ptr);
        return this;
    }

    w(px: number): this {
        lib.symbols.div_w_px(this.ptr, px);
        return this;
    }

    h(px: number): this {
        lib.symbols.div_h_px(this.ptr, px);
        return this;
    }

    size(px: number): this {
        lib.symbols.div_size_px(this.ptr, px);
        return this;
    }

    p(px: number): this {
        lib.symbols.div_p_px(this.ptr, px);
        return this;
    }

    m(px: number): this {
        lib.symbols.div_m_px(this.ptr, px);
        return this;
    }

    gap(px: number): this {
        lib.symbols.div_gap_px(this.ptr, px);
        return this;
    }

    justifyCenter(): this {
        lib.symbols.div_justify_center(this.ptr);
        return this;
    }

    justifyBetween(): this {
        lib.symbols.div_justify_between(this.ptr);
        return this;
    }

    itemsCenter(): this {
        lib.symbols.div_items_center(this.ptr);
        return this;
    }

    itemsStart(): this {
        lib.symbols.div_items_start(this.ptr);
        return this;
    }

    itemsEnd(): this {
        lib.symbols.div_items_end(this.ptr);
        return this;
    }

    bg(color: number): this {
        lib.symbols.div_bg(this.ptr, color);
        return this;
    }

    textColor(color: number): this {
        lib.symbols.div_text_color(this.ptr, color);
        return this;
    }

    textSize(px: number): this {
        lib.symbols.div_text_size(this.ptr, px);
        return this;
    }

    lineHeight(px: number): this {
        lib.symbols.div_line_height(this.ptr, px);
        return this;
    }

    fontFamily(name: string): this {
        const buffer = Buffer.from(name + "\0");
        lib.symbols.div_font_family(this.ptr, buffer);
        return this;
    }

    fontWeight(weight: number): this {
        lib.symbols.div_font_weight(this.ptr, weight);
        return this;
    }

    child(content: Element | string): this {
        if (typeof content === "string") {
            // Bun needs explicit CString handling or Buffer?
            // "cstring" type argument handles Utf8 string or Buffer.
            // But we used `ptr` in FFI definition.
            // We should use Buffer.from(content + "\0") to be safe.
            const buffer = Buffer.from(content + "\0");
            lib.symbols.div_child_text(this.ptr, buffer);
        } else if (content && typeof (content as any).intoElement === 'function') {
            lib.symbols.div_child(this.ptr, (content as Element).intoElement());
        }
        return this;
    }

    intoElement(): Pointer | null {
        return lib.symbols.into_element(this.ptr);
    }
}

export class Svg implements Element {
    ptr: Pointer | null;

    constructor(ptr?: Pointer) {
        if (ptr) {
            this.ptr = ptr;
        } else {
            this.ptr = lib.symbols.create_svg();
        }
    }

    path(p: string): this {
        let content = p;
        if (!p.trim().startsWith("<")) {
            try {
                if (existsSync(p)) {
                    content = readFileSync(p, "utf-8");
                } else {
                    console.warn(`SVG path not found: ${p}`);
                }
            } catch (e) {
                console.error(`Error reading SVG file ${p}:`, e);
            }
        }

        const buffer = Buffer.from(content + "\0");
        lib.symbols.svg_path(this.ptr, buffer);
        return this;
    }

    size(px: number): this {
        lib.symbols.svg_size(this.ptr, px);
        return this;
    }

    textColor(color: number): this {
        lib.symbols.svg_text_color(this.ptr, color);
        return this;
    }

    rotate(angle: number): this {
        lib.symbols.svg_rotate(this.ptr, angle);
        return this;
    }

    withAnimation(id: string, duration: number, callback: (svg: Svg, delta: number) => void): this {
        const buffer = Buffer.from(id + "\0");
        const cb = new JSCallback(
            (svgPtr: any, delta: number) => {
                const svg = new Svg(svgPtr);
                callback(svg, delta);
            },
            {
                args: ["ptr", "f32"],
                returns: "void",
            }
        );
        // @ts-ignore
        (globalThis as any)._keep_alive = (globalThis as any)._keep_alive || [];
        // @ts-ignore
        (globalThis as any)._keep_alive.push(cb);

        lib.symbols.svg_with_animation(this.ptr, buffer, duration, true, cb.ptr);
        return this;
    }

    intoElement(): Pointer | null {
        return lib.symbols.svg_into_element(this.ptr);
    }
}

export function div(): Div {
    return new Div();
}

export function svg(): Svg {
    return new Svg();
}
