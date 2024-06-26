## htmlInput file md5 calculate

Compute the MD5 signature of a file. If the file is larger than 40MB, only the first and last 10MB, as well as the middle 10MB, are read to calculate the MD5. Therefore, even when computing large files, the process will be extremely fast.

### Install
```shell
npm add @quansitech/file-md5-wasm
```

### Usage

use in ES6
```javascript
import init, {calc_file_hash} from '@quansitech/file-md5-wasm';

const upload = async (file: File) => {
    await init();
    const hashId = await calc_file_hash(file);
    console.log(hashId);
}

```

PS. init()方法会根据webpack的 output.publicPath自动寻找wasm文件所在的位置，如果发现这个地址不正确，需要调整webpack的output.publicPath配置。

use in browser

Download the contents of the 'dist' folder. When loading 'index.js' on the webpage, the 'calc_file_hash' function will be automatically added to 'window' object.


```html
<html>
<head>
  <title>文件哈希校验</title>
</head>
<body>
  <input type="file" id="fileInput" />
  <button onclick="handleFile()">上传文件</button>
  <div id="result"></div>

  <script src="./index.js"></script>
  <script>
    function handleFile() {
        const fileInput = document.getElementById("fileInput");
        const start = new Date().getTime();
        window.calc_file_hash(fileInput.files[0]).then(function(res){
          const end = new Date().getTime();
          console.log(`文件Md5: ${res}`);
          console.log(`耗时: ${end - start}ms`);
        });
    }
  </script>
</body>
</html>
```


#### how to build

```shell
wasm-pack build --target web --scope quansitech --package file-md5-wasm
```

publish to npm
```shell
cd pkg
npm publish
```

build index.js for browser
```shell
npm run build
```


#### Old Version Browsers

Old version browsers have poor compatibility with wasm. You may use the JS code in the JS folder as a substitute, which will also create the `calc_file_hash` function to achieve the same functionality.

```html
<script src="md5.min.js"></script>
<script src="calc_file_hash.js"></script>
```