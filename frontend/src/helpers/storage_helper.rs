pub fn store_in_storage(key: String, value: String) {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    storage.set(key.as_str(), value.as_str()).unwrap();
}
