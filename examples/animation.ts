import {App, div, svg} from "../ts/gpui";

console.log("Running Animation Example");

const ARROW_CIRCLE_SVG = "examples/image/arrow_circle.svg";

App.run((appPtr) => {
    App.openWindow(appPtr, () => {
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
                    .justifyBetween() // justify_around
                    .child(
                        div()
                            .flex()
                            .flexCol()
                            .h(150)
                            //.overflowYScroll() // removed
                            .sizeFull()
                            //.flex1() // not implemented
                            .justifyCenter()
                            .itemsCenter()
                            .textSize(24) // text_xl approx
                            .gap(16) // gap_4 approx
                            .child("Hello Animation")
                            .child(
                                svg()
                                    .size(20)
                                    .path(ARROW_CIRCLE_SVG)
                                    .textColor(0x000000)
                                    .withAnimation(
                                        "image_circle",
                                        2, // 2 seconds
                                        (s, delta) => {
                                            s.rotate(delta);
                                        }
                                    )
                            )
                    )
            );
    });
});
