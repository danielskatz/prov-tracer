thread_local!{
    pub static ENABLE_TRACE: std::cell::Cell<bool> = true.into();
}
