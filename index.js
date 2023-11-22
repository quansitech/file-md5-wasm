import init, {calc_file_hash} from "./pkg/file_md5_wasm.js";

init().then(function(){
  window.calc_file_hash = calc_file_hash;
})
