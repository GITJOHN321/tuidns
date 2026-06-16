use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy_to_clipboard(text: &str) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(text.to_owned()).unwrap();

}

