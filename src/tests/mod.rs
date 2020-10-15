#[cfg(not(loom))]
mod idx {
    use crate::{
        cfg,
        page::{self, slot},
        Pack, Tid,
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn tid_roundtrips(tid in 0usize..Tid::<cfg::DefaultConfig>::BITS) {
            let tid = Tid::<cfg::DefaultConfig>::from_usize(tid);
            let packed = tid.pack(0);
            assert_eq!(tid, Tid::from_packed(packed));
        }

        #[test]
        fn idx_roundtrips(
            tid in 0usize..Tid::<cfg::DefaultConfig>::BITS,
            gen in 0usize..slot::Generation::<cfg::DefaultConfig>::BITS,
            addr in 0usize..page::Addr::<cfg::DefaultConfig>::BITS,
        ) {
            let tid = Tid::<cfg::DefaultConfig>::from_usize(tid);
            let gen = slot::Generation::<cfg::DefaultConfig>::from_usize(gen);
            let addr = page::Addr::<cfg::DefaultConfig>::from_usize(addr);
            let packed = tid.pack(gen.pack(addr.pack(0)));
            assert_eq!(addr, page::Addr::from_packed(packed));
            assert_eq!(gen, slot::Generation::from_packed(packed));
            assert_eq!(tid, Tid::from_packed(packed));
        }
    }
}

#[cfg(loom)]
pub(crate) mod util {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    pub(crate) struct TinyConfig;

    impl crate::Config for TinyConfig {
        const INITIAL_PAGE_SIZE: usize = 4;
    }

    pub(crate) fn run_model(name: &'static str, f: impl Fn() + Sync + Send + 'static) {
        run_builder(name, loom::model::Builder::new(), f)
    }

    pub(crate) fn run_builder(
        name: &'static str,
        builder: loom::model::Builder,
        f: impl Fn() + Sync + Send + 'static,
    ) {
        let iters = AtomicUsize::new(1);
        builder.check(move || {
            test_println!(
                "\n------------ running test {}; iteration {} ------------\n",
                name,
                iters.fetch_add(1, Ordering::SeqCst)
            );
            f()
        });
    }
}

#[cfg(loom)]
mod loom_pool;
#[cfg(loom)]
mod loom_slab;
