use electron_rust_terminal_emulator_backend::*;
use neon::prelude::*;

fn marshalling_example_add(mut cx: FunctionContext) -> JsResult<JsNumber> {
 let a = cx.argument::<JsNumber>(0)?.value();
 let b = cx.argument::<JsNumber>(1)?.value();
 let r = electron_rust_terminal_emulator_backend::example_add(a as f32, b as f32);
 Ok(cx.number(r))
}

fn marshalling_example_concat(mut cx: FunctionContext) -> JsResult<JsString> {
 let a = cx.argument::<JsString>(0)?.value();
 let b = cx.argument::<JsString>(1)?.value();
 let r = example_concat(&a, &b);
 Ok(cx.string(r))
}

fn marshalling_hello_from_rust(mut cx: FunctionContext) -> JsResult<JsString> {
    let r = hello_from_rust();
    Ok(cx.string(r))
}

fn marshalling_cp(mut cx: FunctionContext) -> JsResult<JsString> {
    let from = cx.argument::<JsString>(0)?.value();
    let to = cx.argument::<JsString>(1)?.value();
    let r = cp(&from, &to);
    Ok(cx.string(r))
}

register_module!(mut cx, {
 cx.export_function("example_add", marshalling_example_add)?;
 cx.export_function("example_concat", marshalling_example_concat);
 cx.export_function("hello_from_rust", marshalling_hello_from_rust);
 cx.export_function("cp", marshalling_cp)
});
