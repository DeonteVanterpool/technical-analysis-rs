use anyhow::{Error, anyhow};

pub(crate) fn check_window_size(size: usize) -> Result<(), Error> {
    if size <= 1 {
        return Err(anyhow!("Window size must be greater than one. You used size {}", size));
    }
    Ok(())
}
