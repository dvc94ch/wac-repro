use anyhow::Result;
use exports::wasi::cli::run::GuestIndices;
use wac_graph::types::Package;
use wac_graph::{CompositionGraph, EncodeOptions};
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wit_component::ComponentEncoder;

wasmtime::component::bindgen!({
    world: "run",
    path: "../wit",
});

const WASI_INTERFACES: &[&'static str] = &[
    "wasi:cli/environment@0.3.0",
    "wasi:cli/exit@0.3.0",
    "wasi:cli/types@0.3.0",
    "wasi:cli/stdin@0.3.0",
    "wasi:cli/stdout@0.3.0",
    "wasi:cli/stderr@0.3.0",
    "wasi:cli/stderr@0.3.0",
    "wasi:cli/terminal-input@0.3.0",
    "wasi:cli/terminal-output@0.3.0",
    "wasi:cli/terminal-stdin@0.3.0",
    "wasi:cli/terminal-stdout@0.3.0",
    "wasi:cli/terminal-stderr@0.3.0",
    "wasi:clocks/types@0.3.0",
    "wasi:clocks/monotonic-clock@0.3.0",
    "wasi:clocks/system-clock@0.3.0",
];

#[tokio::main]
async fn main() -> Result<()> {
    let app =
        std::fs::read("../hello-world/target/wasm32-wasip3/release/hello-world.wasm").unwrap();
    let wasi_impl =
        std::fs::read("../wasi-impl/target/wasm32-unknown-unknown/release/wasi_impl.wasm").unwrap();
    let wasi_impl = ComponentEncoder::default()
        .module(&wasi_impl)?
        .validate(true)
        .encode()?;

    let mut graph = CompositionGraph::default();
    let wasi_impl = Package::from_bytes("wasi_impl", None, wasi_impl, graph.types_mut())?;
    let wasi_impl = graph.register_package(wasi_impl)?;
    let wasi_impl = graph.instantiate(wasi_impl);
    let app = Package::from_bytes("app", None, app, graph.types_mut())?;
    let app = graph.register_package(app)?;
    let app = graph.instantiate(app);

    for interface in WASI_INTERFACES {
        let export = graph.alias_instance_export(wasi_impl, interface)?;
        graph.set_instantiation_argument(app, interface, export)?;
    }

    let export = graph.alias_instance_export(app, "wasi:cli/run@0.3.0")?;
    graph.export(export, "wasi:cli/run@0.3.0")?;

    let wasm = graph.encode(EncodeOptions::default())?;

    let config = Config::default();
    let engine = Engine::new(&config)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    let component = Component::from_binary(&engine, &wasm)?;
    let instance_pre = linker.instantiate_pre(&component)?;
    let indices = GuestIndices::new(&instance_pre)?;
    let instance = instance_pre.instantiate_async(&mut store).await?;
    let guest = indices.load(&mut store, &instance)?;
    println!("running task");
    store
        .run_concurrent(async |accessor| guest.call_run(accessor).await)
        .await??
        .map_err(|err| anyhow::anyhow!("{err:?}"))?;
    Ok(())
}
