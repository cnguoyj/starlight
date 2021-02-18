use criterion::{criterion_group, criterion_main, Criterion};
use gc::handle::Handle;
use runtime::{arguments::Arguments, value::JsValue};
use starlight::*;
use vm::{Options, VirtualMachine};

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
const CODE: &'static str = r#"
var obj = new Object()
obj.x = 0
for (;obj.x < 10000;obj.x = obj.x + 1) {
    
}

"#;
fn criterion_benchmark(c: &mut Criterion) {
    let mut vm = VirtualMachine::new(Options::default());
    // vm.space().defer_gc();
    let mut func = vm
        .compile(false, CODE, "<Code>")
        .unwrap_or_else(|_| panic!())
        .root();
    let mut boa_ctx = boa::Context::new();
    let args = Arguments::new(&mut vm, JsValue::undefined(), 0);
    let mut args = Handle::new(vm.space(), args);
    c.bench_function("starlight-prop-for-loop", |b| {
        b.iter(|| match func.as_function_mut().call(&mut vm, &mut args) {
            Ok(_) => (),
            Err(_) => unreachable!(),
        });
    });

    c.bench_function("boa-prop-eval-for-loop", |b| {
        b.iter(|| {
            boa_ctx.eval(CODE).unwrap();
        });
    });
}
