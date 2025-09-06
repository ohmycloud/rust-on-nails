pub mod api {
    tonic::include_proto!("api");
}

pub fn get_file_descriptor_set() -> &'static [u8] {
    tonic::include_file_descriptor_set!("api_descriptor_set")
}
