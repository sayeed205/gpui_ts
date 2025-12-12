import { App, div } from "./ts/gpui";

console.log("Starting GPUI App via TS FFI");

App.run((appPtr) => {
    App.openWindow(appPtr, () => {
        return div()
            .flex()
            .flexCol()
            .sizeFull()
            .bg(0x111111)
            .justifyCenter()
            .itemsCenter()
            .child(
                div()
                    .w(400)
                    .h(300)
                    .bg(0x2a2a2a)
                    .p(20)
                    .gap(10)
                    .flex()
                    .flexCol()
                    .child(
                        div()
                            .textColor(0xff0000)
                            .textSize(32)
                            .child("Hello Styled Text!")
                    )
                    .child(
                        div()
                            .w(100)
                            .h(40)
                            .bg(0x4a90e2)
                            .justifyCenter()
                            .itemsCenter()
                            .flex()
                            .child(
                                div().child("Button?")
                            )
                    )
            );
    });
});

console.log("App exited");
