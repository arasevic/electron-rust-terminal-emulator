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

fn marshalling_pwd(mut cx: FunctionContext) -> JsResult<JsString> {
 let r = pwd();
 Ok(cx.string(r))
}

fn marshalling_help(mut cx: FunctionContext) -> JsResult<JsString> {
 let r = help();
 Ok(cx.string(r))
}

fn marshalling_cd(mut cx: FunctionContext) -> JsResult<JsString> {
 let r = cd();
 Ok(cx.string(r))
}

fn marshalling_cd_with_args(mut cx: FunctionContext) -> JsResult<JsString> {
 let dest = cx.argument::<JsString>(0)?.value();
 let r = cd_with_args(&dest);
 Ok(cx.string(r))
}

fn marshalling_mkdir(mut cx: FunctionContext) -> JsResult<JsString> {
 let path = cx.argument::<JsString>(0)?.value();
 let r = mkdir(&path);
 Ok(cx.string(r))
}

register_module!(mut cx, {
 cx.export_function("example_add", marshalling_example_add)?;
 cx.export_function("example_concat", marshalling_example_concat);
 cx.export_function("hello_from_rust", marshalling_hello_from_rust);
 cx.export_function("pwd", marshalling_pwd);
 cx.export_function("help", marshalling_help);
 cx.export_function("cd", marshalling_cd);
 cx.export_function("cd_with_args", marshalling_cd_with_args);
 cx.export_function("mkdir", marshalling_mkdir)
});
