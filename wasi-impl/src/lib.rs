use exports::wasi::cli::types::ErrorCode;
use wit_bindgen::{FutureReader, StreamReader};

wit_bindgen::generate!({
    world: "impl",
    path: "../wit",
    generate_all,
});

struct WasiHandler;
export!(WasiHandler);

impl exports::wasi::cli::environment::Guest for WasiHandler {
    fn get_environment() -> Vec<(String, String)> {
        vec![]
    }

    fn get_arguments() -> Vec<String> {
        vec![]
    }

    fn get_initial_cwd() -> Option<String> {
        None
    }
}

impl exports::wasi::cli::stdout::Guest for WasiHandler {
    fn write_via_stream(stream: StreamReader<u8>) -> FutureReader<Result<(), ErrorCode>> {
        fn f() -> Result<(), ErrorCode> {
            Ok(())
        }
        let (writer, reader) = wit_future::new(f);
        wit_bindgen::spawn_local(async move {
            let bytes = stream.collect().await;
            panic!("{bytes:?}");
            writer.write(Ok(()));
        });
        reader
    }
}

struct TerminalInput;

impl exports::wasi::cli::terminal_input::GuestTerminalInput for TerminalInput {}

impl exports::wasi::cli::terminal_stdin::Guest for WasiHandler {
    fn get_terminal_stdin() -> Option<exports::wasi::cli::terminal_stdin::TerminalInput> {
        None
    }
}

impl exports::wasi::cli::terminal_input::Guest for WasiHandler {
    type TerminalInput = TerminalInput;
}

impl exports::wasi::cli::stdin::Guest for WasiHandler {
    fn read_via_stream() -> (StreamReader<u8>, FutureReader<Result<(), ErrorCode>>) {
        todo!()
    }
}

struct TerminalOutput;

impl exports::wasi::cli::terminal_output::GuestTerminalOutput for TerminalOutput {}

impl exports::wasi::cli::terminal_stdout::Guest for WasiHandler {
    fn get_terminal_stdout() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        None
    }
}

impl exports::wasi::cli::terminal_output::Guest for WasiHandler {
    type TerminalOutput = TerminalOutput;
}

impl exports::wasi::cli::terminal_stderr::Guest for WasiHandler {
    fn get_terminal_stderr() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        None
    }
}

impl exports::wasi::cli::stderr::Guest for WasiHandler {
    fn write_via_stream(_stream: StreamReader<u8>) -> FutureReader<Result<(), ErrorCode>> {
        todo!()
    }
}

impl exports::wasi::cli::exit::Guest for WasiHandler {
    fn exit(_ret: Result<(), ()>) {}

    fn exit_with_code(_code: u8) {}
}

impl exports::wasi::clocks::monotonic_clock::Guest for WasiHandler {
    fn now() -> u64 {
        0
    }

    fn get_resolution() -> u64 {
        1
    }

    async fn wait_until(_time: u64) {}

    async fn wait_for(_duration: u64) {}
}

impl exports::wasi::clocks::system_clock::Guest for WasiHandler {
    fn now() -> exports::wasi::clocks::system_clock::Instant {
        todo!()
    }

    fn get_resolution() -> u64 {
        1
    }
}
