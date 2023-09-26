const js = import("./pkg/file_md5_wasm.js");
js.then((js) => {
  window.calc_file_hash = js.calc_file_hash;
});