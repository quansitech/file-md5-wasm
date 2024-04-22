(function(w){
    var MD5_RANGE_SIZE = 10 * 1024 * 1024;
    var CUT_LIMIT = 40 * 1024 * 1024;

    w.calc_file_hash = function(file){
        var p = new Promise(function(resolve, reject){
            var fileSize = file.size;
            var range = makeMd5Range(fileSize);

            var index = 0;
            var hash = md5.create();// 创建MD5哈希对象
        
            var reader = new FileReader();
        
            reader.onload = function(e) {
                // 处理当前分块的读取结果
                var chunkData = e.target.result;
                
                hash.update(chunkData); // 更新哈希计算结果
                
                if (index < range.length - 1) {
                    index++;
                    readNextChunk(range, index);
                } else {
                    var fileMD5 = hash.hex(); // 计算文件的最终MD5哈希值

                    resolve(fileMD5);
                }
            };
        
            reader.onerror = function(e) {
                reject(e);

            };
        
            function readNextChunk(chunkRange, index) {
                const chunk = file.slice(chunkRange[index][0], chunkRange[index][1]);
                reader.readAsArrayBuffer(chunk);
            }
        
            readNextChunk(range, index);
        });

        return p;
    };

    function makeMd5Range(fileSize) {
        if(CUT_LIMIT > fileSize) {
          return [[0, fileSize]];
        }
    
         // MD5_RANGE_SIZE设置为10
        let first = [0.0, MD5_RANGE_SIZE];
        let last = [fileSize - MD5_RANGE_SIZE, fileSize];
    
        // 中间段算法
        let rangeMod = fileSize - 3.0 * MD5_RANGE_SIZE;
        let midStart = fileSize % rangeMod;
        let middle = [MD5_RANGE_SIZE + midStart, MD5_RANGE_SIZE + midStart + MD5_RANGE_SIZE];
    
        return [first, middle, last];
    }

})(window);