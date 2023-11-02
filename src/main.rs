use esp_idf_sys::*;

fn main() {
    esp_idf_sys::link_patches();
    unsafe { nvs_flash_init() };
    std::thread::spawn(|| {
        println!("hi");
    })
    .join()
    .unwrap();
}
