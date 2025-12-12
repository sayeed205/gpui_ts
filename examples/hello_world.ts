import { App, div } from "../ts/gpui";

console.log("Running Hello World Example");

App.run((appPtr) => {
    App.openWindow(appPtr, "Hello World", () => {
        return div()
            .flex()
            .flexCol()
            .sizeFull()
            .justifyCenter()
            .itemsCenter()
            .bg(0x1e1e1e)
            .child(
                div()
                    .flex()
                    .flexCol()
                    .itemsCenter()
                    .gap(16)
                    .child(
                        div()
                            .textColor(0xffffff)
                            .textSize(32)
                            .fontWeight(700)
                            .child("Hello, World!")
                    )
                    .child(
                        div()
                            .textColor(0xaaaaaa)
                            .textSize(16)
                            .child("Powered by GPUI + Bun")
                    )
            );
    });
});
