// 链接到 `image` 和 `base64` 库，导入其中的项
extern crate image;
extern crate base64;
// 使用 `use` 从 image 的命名空间导入对应的方法
use image::DynamicImage;
use image::ImageFormat;
// 从 std（基础库）的命名空间导入对应方法，可用解构的方式
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::panic;
use base64::{encode};

// 引入 wasm_bindgen 下 prelude 所有模块，用作 在 Rust 与 JavaScript 之间通信
use wasm_bindgen::prelude::*;

// 当`wee_alloc`特性启用的时候，使用`wee_alloc`作为全局分配器。
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen] 属性表明它下面的函数可以在JavaScript和Rust中访问。
#[wasm_bindgen]
extern "C"  {
    // 该 extern 块将外部 JavaScript 函数 console.log 导入 Rust。
    // 通过以这种方式声明它，wasm-bindgen 将创建 JavaScript 存根 console
    // 允许我们在 Rust 和 JavaScript 之间来回传递字符串。
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    // 使用 match 进行兜底报错匹配
    let img = match image::load_from_memory_with_format(_array, ImageFormat::Png) {
        Ok(img) => img,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    img
}

fn get_image_as_base64(_img: DynamicImage) -> String {
    // 使用 mut 声明可变变量，类似 js 中的 let，不使用 mut 为不可变
    // 使用 Cursor 创建一个内存缓存区，里面是动态数组类型
    let mut c = Cursor::new(Vec::new());
    // 写入图片
    match _img.write_to(&mut c, ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => {
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };
    // 寻找以字节为单位的偏移量，直接用 unwrap 隐式处理 Option 类型，直接返回值或者报错
    c.seek(SeekFrom::Start(0)).unwrap();

    // 声明一个可变的动态数组作输出
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();
    // 使用 encode 转换
    let stt = encode(&mut out);
    let together = format!("{}{}", "data:image/png;base64,", stt);
    together
}

#[wasm_bindgen]
pub fn grayscale(_array: &[u8]) -> Result<(), JsValue> {
    let mut img = load_image_from_array(_array);
    // img = img.grayscale();
    // img = img.brighten(100); // 变亮
    // img = img.rotate90();// 旋转90度
    img.invert(); // 反色
    let base64_str = get_image_as_base64(img);
    append_img(base64_str)
}

pub fn append_img(image_src: String) -> Result<(), JsValue> {
    // 使用 `web_sys` 来获取 window 对象
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // 创建 img 元素
    // 使用 `?` 在出现错误的时候会直接返回 Err
    let val = document.create_element("img")?;
    // val.set_inner_html("Hello from Rust!");
    val.set_attribute("src", &image_src)?;
    val.set_attribute("style", "height: 200px")?;
    body.append_child(&val)?;
    log("success!");

    Ok(())
}