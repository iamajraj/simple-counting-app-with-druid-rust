use druid::{
    widget::{Align, Button, Flex, Label},
    AppLauncher, Data, Env, Size, UnitPoint, Widget, WindowDesc,
};

#[derive(Clone, Data)]
struct MyData {
    num: i32,
}

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("Hello World")
        .window_size(Size::new(400.0, 400.0));
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(MyData { num: 0 });
}

fn ui_builder() -> impl Widget<MyData> {
    let label =
        Label::new(|data: &MyData, _: &Env| format!("Count -> {}", data.num)).with_text_size(40.0);
    let increment = Button::new("+").on_click(|_ctx, data: &mut MyData, _env| data.num += 1);
    let decrement = Button::new("-").on_click(|_ctx, data: &mut MyData, _env| data.num -= 1);
    return Align::new(
        UnitPoint::CENTER,
        Flex::column()
            .with_child(label)
            .with_child(Flex::row().with_child(increment).with_child(decrement)),
    );
}
