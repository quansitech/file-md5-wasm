const js = import("./pkg/md5_wasm.js");
js.then((js) => {
  window.calc_file_hash = js.calc_file_hash;
  window.file_size = js.file_size;
  window.file_debug = js.file_debug;
});