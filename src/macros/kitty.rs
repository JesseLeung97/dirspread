#[macro_export]
macro_rules! kitty {
    ($( $x:expr ),* ) => {
        {
            let mut cmd = std::process::Command::new("kitty");
            $(
                cmd.arg($x);
            )*
            cmd
        }
    };
}
