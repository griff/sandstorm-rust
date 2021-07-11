extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/supervisor.capnp")
        .file("schema/util.capnp")
        .file("schema/powerbox.capnp")
        .file("schema/identity.capnp")
        .file("schema/activity.capnp")
        .file("schema/grain.capnp")
        .file("schema/web-session.capnp")
        .file("schema/ip.capnp")
        .file("schema/email.capnp")
        .file("schema/web-publishing.capnp")
        .file("schema/sandstorm-http-bridge.capnp")
        .file("schema/api-session.capnp")
        .run()
        .expect("compiling");
    let path = std::env::current_dir().unwrap();
    eprintln!("The current directory is {}", path.display());
    println!("cargo:rerun-if-changed=schema");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:capnp_include={}/schema", path.display());
    eprintln!("cargo:capnp_include={}/schema", path.display());
}
