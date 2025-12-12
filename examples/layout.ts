import {App, div} from "../ts/gpui";


function coloredBox(color: number, w: number = 50, h: number = 50) {
    return div().bg(color).w(w).h(h).m(4);
}

App.run((appPtr) => {
    App.openWindow(appPtr, () => {
        try {
            return div()
                .flex()
                .flexCol()
                .w(800)
                .h(600)
                .bg(0xff0000)
                .child(
                    // Header
                    div().w(600).h(60).bg(0x333333).flex().itemsCenter().justifyCenter().child(
                        div().textColor(0xffffff).textSize(24).child("Flexbox Layout Demo")
                    )
                )
                .child(
                    // Content
                    div().flex().sizeFull().gap(20).p(20).child(
                        // Sidebar
                        div().w(150).h(400).bg(0x444444).flex().flexCol().itemsCenter().p(10).gap(10).child(
                            div().textColor(0xdddddd).child("Sidebar Item 1")
                        )
                    )
                );
        } catch (e) {
            return div().child("Error Building View").textColor(0xff0000);
        }
    });
});
