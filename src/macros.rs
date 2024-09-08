#[macro_export(local_inner_macros)]
macro_rules! success_provider {
    ( $self:ident, $provider_classname:ident ) => {
        Ok($provider_classname {
            // TODO: handle the lifetimes, if you care. These will happen relatively rarely (a few
            // times a second) so the perf hit is essentially non-existent unless you're storing a
            // lot of data here.
            data: Rc::clone(&$self.data),
            state: std::marker::PhantomData,
        })
    };
}

