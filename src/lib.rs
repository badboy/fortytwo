#[allow(dead_code)]
mod ascii;

/// Don't panic!
pub fn dont_panic() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("{}\n{}", ascii::MONSTER, ascii::DONT_PANIC);
        if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("{}", msg);
        }
    }));
}
