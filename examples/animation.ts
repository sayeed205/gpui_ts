import { App, div, svg } from "../ts/gpui";


const ARROW_CIRCLE_SVG = "examples/image/arrow_circle.svg";

App.run((appPtr) => {
    App.openWindow(appPtr, "Animation Example", () => {
        return div()
            .flex()
            .flexCol()
            .sizeFull()
            .bg(0xffffff)
            .textColor(0x000000)
            .justifyBetween()
            .child(
                div()
                    .flex()
                    .flexCol()
                    .sizeFull()
                    .justifyBetween()
                    .child(
                        div()
                            .flex()
                            .flexCol()
                            .h(150)
                            .sizeFull()
                            .justifyCenter()
                            .itemsCenter()
                            .textSize(24)
                            .gap(16)
                            .child("Hello Animation")
                            .child(
                                svg()
                                    .size(20)
                                    .path(ARROW_CIRCLE_SVG)
                                    .textColor(0x000000)
                                    .withAnimation(
                                        "image_circle",
                                        2,
                                        (s, delta) => {
                                            s.rotate(delta);
                                        }
                                    )
                            )
                    )
            );
    });
});
