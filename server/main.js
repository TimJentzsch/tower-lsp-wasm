// Wrapper code launch the server binary.
// It is expected to be compiled to the pkg folder with wasm-pack.
const { main } = require("./pkg/tower_lsp_wasm");

// Launch the server
main();
