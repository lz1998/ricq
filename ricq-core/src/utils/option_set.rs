pub trait OptionSet: Sized {
    /// set value if is `Some(Self)` or do noting
    fn option_set(&mut self, value: Option<Self>);
}

impl<T: Sized> OptionSet for T {
    #[inline]
    fn option_set(&mut self, value: Option<Self>) {
        if let Some(value) = value {
            *self = value
        }
    }
}
