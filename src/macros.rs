#[macro_export(local_inner_macros)]
macro_rules! success_provider {
    ( $self:ident, $provider_classname:ident ) => {
        Ok(Provider {
            data: Rc::clone(&$self.data),
            state: std::marker::PhantomData,
        })
    };
}

