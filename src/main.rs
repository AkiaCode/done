use std::fs;

use rusty_v8 as v8;
use v8::{FunctionCallback, Local, MapFnTo};

fn main() {
    let platform = v8::new_default_platform().unwrap();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    {
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
        let handle_scope = &mut v8::HandleScope::new(isolate);

        let context = init(handle_scope);
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        modules(scope);

        let script = compile(scope, "./done.js").unwrap();
        let mut ts_scope = rusty_v8::TryCatch::new(scope);
        match script.run(&mut ts_scope) {
            Some(_) => {}
            None => {
                eprintln!(
                    "{}",
                    ts_scope
                        .stack_trace()
                        .unwrap()
                        .to_string(&mut ts_scope)
                        .unwrap()
                        .to_rust_string_lossy(&mut ts_scope)
                );
            }
        };
    }

    unsafe {
        v8::V8::dispose();
    }
    v8::V8::shutdown_platform();
}

fn modules<'a>(scope: &mut v8::HandleScope<'a>) {
    let modules = [
        "./src/modules/require.js",
        "./src/modules/console.js",
        "./src/modules/colors.js",
    ];
    for module in modules.iter() {
        let module = compile(scope, module).unwrap();
        module.run(scope);
    }
}

fn compile<'a>(
    scope: &mut v8::HandleScope<'a>,
    path: &'a str,
) -> Result<rusty_v8::Local<'a, rusty_v8::Script>, std::string::String> {
    let name = path.split("/").last().unwrap();
    let str = fs::read_to_string(path).expect("Something went wrong reading the file");

    let code = v8::String::new(scope, &str).unwrap();

    let mut ts_scope = rusty_v8::TryCatch::new(scope);

    let resource_name = v8::String::new(&mut ts_scope, name).unwrap();
    let source_map_url = v8::String::new(&mut ts_scope, path).unwrap();
    let origin = v8::ScriptOrigin::new(
        &mut ts_scope,
        resource_name.into(),
        0,
        0,
        false,
        0,
        source_map_url.into(),
        false,
        false,
        false,
    );

    match v8::Script::compile(&mut ts_scope, code, Some(&origin)) {
        Some(script) => Ok(script),
        None => {
            let exception = ts_scope.stack_trace().unwrap();
            Err(exception
                .to_string(&mut ts_scope)
                .unwrap()
                .to_rust_string_lossy(&mut ts_scope))
        }
    }
}

// https://github.com/denoland/deno/blob/c00872c0c0ad09fb0e45e7a10c6ffeff6a7bddef/core/bindings.rs#L103
fn init<'a>(scope: &mut v8::HandleScope<'a, ()>) -> Local<'a, rusty_v8::Context> {
    let scope = &mut v8::EscapableHandleScope::new(scope);

    let context = v8::Context::new(scope);
    let global = context.global(scope);

    let scope = &mut v8::ContextScope::new(scope, context);

    //Done
    let name = v8::String::new(scope, "Done").unwrap();
    let done = v8::Object::new(scope);
    global.set(scope, name.into(), done.into());

    //core
    let name = v8::String::new(scope, "core").unwrap();
    let core = v8::Object::new(scope);
    done.set(scope, name.into(), core.into());

    set_function(scope, core, "print", print);
    set_function(scope, core, "println", println);

    //fs
    let name = v8::String::new(scope, "fs").unwrap();
    let fs = v8::Object::new(scope);
    global.set(scope, name.into(), fs.into());

    set_function(scope, fs, "readFileSync", read_file_sync);

    return scope.escape(context);
}

fn set_function(
    scope: &mut v8::HandleScope,
    core: rusty_v8::Local<'_, rusty_v8::Object>,
    name: &str,
    callback: impl MapFnTo<FunctionCallback>,
) {
    let print_val = v8::String::new(scope, name).unwrap();
    let function_template = v8::FunctionTemplate::new(scope, callback);
    let val = function_template.get_function(scope).unwrap();
    core.set(scope, print_val.into(), val.into());
}

fn print(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    print!(
        "{}",
        args.get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope)
    )
}

fn println(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    println!(
        "{}",
        args.get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope)
    )
}

fn read_file_sync(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let path = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let str = match fs::read_to_string(path) {
        Ok(e) => e,
        Err(e) => String::from(e.to_string()),
    };
    let value = v8::String::new(scope, &str.to_string()).unwrap();
    rv.set(value.into());
}
