test = require('./out/test')
test.set_panic_hook();
try {
    console.info("Running tests...");
    test.wasm.main();
} catch (error) {
    process.exit(1);
}
console.info("Tests passed")
