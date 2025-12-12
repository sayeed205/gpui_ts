import {App, div, svg} from "../ts/gpui";
import {Heart} from 'lucide-static';

// Load from file (relative to project root)
const CHECK_ICON = "examples/image/arrow_circle.svg";

App.run((appPtr) => {
    App.openWindow(appPtr, () => {
        return div()
            .flex()
            .flexCol()
            .sizeFull()
            .bg(0x222222)
            .justifyCenter()
            .itemsCenter()
            .gap(20)
            .child(
                div().textColor(0xffffff).textSize(24).child("SVG Support")
            )
            .child(
                div().flex().child(
                    svg()
                        .path(Heart)
                        .size(48)
                        .textColor(0x00ff00)
                ).child(
                    svg().path(CHECK_ICON).size(48).textColor(0xfff)
                )
            )
            .child(
                div().textColor(0xaaaaaa).child("Green Checkmark should appear above")
            );
    });
});
