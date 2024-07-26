extern crate embed_resource;

fn main() {
    embed_resource::compile("resource.rc", embed_resource::NONE);
}
