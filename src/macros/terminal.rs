#[macro_export]
macro_rules! terminal {
    ( $( $x:expr ),* ) => {
        {
            let mut cmd = std::process::Command::new("osascript");
            $(
                cmd.arg($x);
            )*
            cmd
        }
    };
}
