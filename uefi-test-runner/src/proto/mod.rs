use uefi::prelude::*;

use uefi::proto;

pub fn test(st: &mut SystemTable<Boot>) {
    info!("Testing various protocols");

    console::test(st);

    let bt = st.boot_services();
    find_protocol(bt);

    debug::test(bt);
    media::test(bt);
    pi::test(bt);

    #[cfg(any(
        target_arch = "i386",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64"
    ))]
    shim::test(bt);
}

fn find_protocol(bt: &BootServices) {
    type SearchedProtocol<'boot> = proto::console::text::Output<'boot>;

    let handles = bt
        .find_handles::<SearchedProtocol>()
        .expect_success("Failed to retrieve list of handles");

    assert!(
        handles.len() > 1,
        "There should be at least one implementation of Simple Text Output (stdout)"
    );
}

mod console;
mod debug;
mod media;
mod pi;
#[cfg(any(
    target_arch = "i386",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64"
))]
mod shim;
