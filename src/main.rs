use gloo::utils::document_element;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlCanvasElement;
use wgpu::Instance;

/*wasmでwgpu
ここ従ってやってみる
https://zenn.dev/matcha_choco010/articles/2022-07-11-wgpu-web
今度は動くか？

*/
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

async fn run() {
    let canvas = document().create_element("canvas").unwrap();
    document_element().append_child(&canvas).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into().unwrap();

    let width = canvas.width();
    let height = canvas.height();

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = instance.create_surface_from_canvas();
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    spawn_local(run());
}
