import { lib } from "./ffi";
import { JSCallback, Pointer } from "bun:ffi";

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

    static openWindow(appPtr: Pointer | null, viewBuilder: () => Div) {
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

        lib.symbols.open_window(appPtr, cb.ptr);

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

    child(content: Div | string): this {
        if (typeof content === "string") {
            // Bun needs explicit CString handling or Buffer?
            // "cstring" type argument handles Utf8 string or Buffer.
            // But we used `ptr` in FFI definition.
            // We should use Buffer.from(content + "\0") to be safe.
            const buffer = Buffer.from(content + "\0");
            lib.symbols.div_child_text(this.ptr, buffer);
        } else if (content instanceof Div) {
            lib.symbols.div_child(this.ptr, content.intoElement());
        }
        return this;
    }

    intoElement(): Pointer | null {
        return lib.symbols.into_element(this.ptr);
    }
}

export function div(): Div {
    return new Div();
}
