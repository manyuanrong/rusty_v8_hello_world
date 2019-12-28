use rusty_v8 as v8;

fn main() {
    let platform = v8::platform::new_default_platform();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let mut create_params = v8::Isolate::create_params();
    create_params.set_array_buffer_allocator(v8::new_default_allocator());
    let isolate = v8::Isolate::new(create_params);
    let mut locker = v8::Locker::new(&isolate);

    {
        let mut handle_scope = v8::HandleScope::new(&mut locker);
        let scope = handle_scope.enter();

        let mut context = v8::Context::new(scope);
        context.enter();
        let code = rusty_v8::String::new(scope, "'Hello World!'").unwrap();
        code.to_rust_string_lossy(scope);
        let mut script = v8::Script::compile(scope, context, code, None).unwrap();
        let result = script.run(scope, context).unwrap();
        let result: v8::Local<v8::String> = unsafe { std::mem::transmute_copy(&result) };

        let str = result.to_rust_string_lossy(scope);

        println!("{}", str);

        context.exit();
    }

    drop(locker);
}