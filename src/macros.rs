#[macro_export(local_inner_macros)]
macro_rules! success {
    ( $self:ident ) => {
        Ok(Provider {
            data: Rc::clone(&$self.data),
            state: std::marker::PhantomData,
        })
    };
}

