#[cfg(test)]
mod tests {
    extern crate loge;
    use log::{debug, error, info, trace, warn};
    use std::env;

    #[test]
    fn test_fileline_loge() {
        env::set_var("RUST_LOG", "trace");
        loge::init_fileline();

        self::deep();
        debug!("deboogging");
        info!("such information");
        warn!("o_O");
        error!("boom");
    }

    pub fn deep() {
        trace!("one level deep!");
    }
}