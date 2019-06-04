use std::path::Path;

use chainx_sync_parse::{Result, Tail};

fn main() -> Result<()> {
    let path = Path::new("./data/tail.log");

    let tail = Tail::new();
    let handle = tail.run(&path, 0)?;

    while let Ok((height, key, value)) = tail.recv_data() {
        if let Ok(key) = std::str::from_utf8(&key) {
            println!(
                "height = {:?}, key = {:?}, value = {:?}",
                height, key, value
            );
        } else {
            println!(
                "height = {:?}, key = Invalid UTF8({:?}), value = {:?}",
                height, key, value
            );
        }
    }

    handle.join().expect("Join should be successful");
    Ok(())
}
