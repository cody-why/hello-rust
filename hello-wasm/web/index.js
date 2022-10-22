
import('../pkg/hello_wasm.js').then(async m => {
    await m.default();
    // m.greet("Hello from Rust!");
    m.greet(m.fib(10)+'');
});