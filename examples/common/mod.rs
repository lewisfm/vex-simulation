macro_rules! create_main {
    ($entrypoint:ident) => {
        #[::vexide::main]
        async fn main(p: Peripherals) {
            #[cfg(target_os = "vexos")]
            $entrypoint(p).await;

            #[cfg(not(target_os = "vexos"))]
            {
                ::tracing_subscriber::fmt()
                    .with_env_filter(::tracing_subscriber::EnvFilter::from_default_env())
                    .init();

                ::vex_sdk_desktop::run_simulator(move || {
                    ::vexide::runtime::block_on(async move {
                        $entrypoint(p).await;
                    });
                })
                .unwrap();
            }
        }
    };
}

pub(crate) use create_main;
