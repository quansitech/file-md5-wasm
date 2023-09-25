use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use md5::{Md5, Digest};
use hex;
use web_sys;

const CHUNK_SIZE: f64 = (2 * 1024 * 1024) as f64;
const MD5_RANGE_SIZE: f64 = (10 * 1024 * 1024) as f64;
const CUT_LIMIT: f64 = ( 40 * 1024 * 1024) as f64;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn calc_file_hash(file: web_sys::File) -> String {
    let file_size = file.size();
    let range = match file_size < CUT_LIMIT {
        true => vec![(0.0, file_size)],
        false => make_md5_range(file_size)
    };

    let mut hasher: Md5 = Md5::new();

    for r in range {
        hash_part(&file, &mut hasher, r.0, r.1).await;
    }

    let result = hasher.finalize();
    hex::encode(result)
}

async fn hash_part(file: &web_sys::File, hasher: &mut Md5, start: f64, end: f64) {
    let mut offset: f64 = start;

    

    loop {
        let offset_size = match (offset + CHUNK_SIZE) > end {
            true => end - offset,
            false => CHUNK_SIZE
        };

        let slice = match file.slice_with_f64_and_f64(offset, offset + offset_size) {
            Ok(slice) => slice,
            Err(_) => break
        };

        let array_buffer = JsFuture::from(slice.array_buffer()).await.unwrap();

        let bytes = js_sys::Uint8Array
        ::new(&array_buffer)
        .to_vec();

        let bytes_size: i32 = i32::try_from(bytes.len()).unwrap();

        hasher.update(bytes);
        offset += bytes_size as f64;

        if offset >= end {
            break;
        }
    }

}

fn make_md5_range(file_size: f64)-> Vec<(f64, f64)> {
    let first: (f64, f64) = (0.0, MD5_RANGE_SIZE);
    let last: (f64, f64) = (file_size - MD5_RANGE_SIZE, file_size);

    //中间段算法：减去头尾两端，剩余中间位置，计算可容纳MD5_RANGE_SIZE长度的部位起始区间，再根据size取模的方式获取固定起始值
    let range_mod = file_size - 3.0 * MD5_RANGE_SIZE;
    let mid_start = file_size % range_mod;
    let middle = (MD5_RANGE_SIZE + mid_start, MD5_RANGE_SIZE + mid_start + MD5_RANGE_SIZE);

    vec![first, middle, last]
}