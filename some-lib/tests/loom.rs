#[cfg(loom)]
#[cfg(test)]
mod loom_tests {
    use loom::sync::atomic::AtomicUsize;
    use loom::thread;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::Arc;

    #[test]
    fn loom() {
        loom::model(|| {
            let v1 = Arc::new(AtomicUsize::new(0));
            let v2 = v1.clone();

            thread::spawn(move || {
                v1.store(1, SeqCst);
            });

            assert!(true)
        })
    }
}
