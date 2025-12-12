# @hitarashi/gpui

Isomorphic GPUI bindings for TypeScript (Bun). Write high-performance GPU-accelerated UI applications in TypeScript, powered by Zed's GPUI.

## Features

- **Native Performance**: Uses Zed's Rust GPUI crate.
- **Isomorphic Architecture**: Writes UI logic in TypeScript, renders in Rust.
- **Cross-Platform**: Supports Linux, macOS, and Windows.
- **Asset Support**: Loads SVGs and Images.
- **Animations**: native high-performance animations.

## Usage

```typescript
import { App, div, svg } from "@hitarashi/gpui";

const ARROW_CIRCLE_SVG = "path/to/icon.svg";

App.run((appPtr) => {
    App.openWindow(appPtr, "My GPUI App", () => {
        return div()
            .flex()
            .sizeFull()
            .bg(0xffffff)
            .justifyCenter()
            .itemsCenter()
            .child(
                svg()
                    .path(ARROW_CIRCLE_SVG)
                    .size(48)
                    .textColor(0x000000)
                    .withAnimation("spin", 2, (s, delta) => s.rotate(delta))
            );
    });
});
```

## Installation

```bash
bun add @hitarashi/gpui
```
