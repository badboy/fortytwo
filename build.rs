use rustc_version::version;

include!("src/ascii.rs");

const MSG: &str = r#"
It is said that despite its many glaring (and occasionally fatal) inaccuracies,
the Rust Compiler Version 42 has outsold the Encyclopedia Galactica Compiler Collection
because it is slightly cheaper,
and because it has the words 'DON'T PANIC' in large, friendly letters on the cover.
"#;

fn main() {
    let ver = version().unwrap();
    if ver.major == 1 && ver.minor == 42 {
        let msg = format!("{}\n{}\n{}\n", MONSTER, DONT_PANIC, MSG);
        for line in msg.lines() {
            println!("cargo:warning={}", line);
        }
    } else {
        panic!("{}\n{}\n{}\n", MONSTER, PANIC, MSG);
    }
}
