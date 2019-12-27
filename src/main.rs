use rusty_v8::{Isolate, V8, HandleScope, Script, Context, Locker, Local};
use rusty_v8::{platform, new_default_allocator};

fn main() {
    let platform = platform::new_default_platform();
    V8::initialize_platform(platform);
    V8::initialize();

    let mut create_params = Isolate::create_params();
    create_params.set_array_buffer_allocator(new_default_allocator());
    let isolate = Isolate::new(create_params);
    let mut locker = Locker::new(&isolate);

    {
        let mut handle_scope = HandleScope::new(&mut locker);
        let scope = handle_scope.enter();

        let mut context = Context::new(scope);
        context.enter();
        let code = rusty_v8::String::new(scope, "'Hello World!'").unwrap();
        code.to_rust_string_lossy(scope);
        let mut script = Script::compile(scope, context, code, None).unwrap();
        let result = script.run(scope, context).unwrap();
        let result: Local<rusty_v8::String> = unsafe { std::mem::transmute_copy(&result) };

        let str = result.to_rust_string_lossy(scope);

        println!("{}", str);

        context.exit();
    }

    drop(locker);
}