use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use lorust::debounce;

#[test]
fn test_debounce_concept() {
    let effect_run_times = Arc::new(Mutex::new(0));
    let param = Arc::new(Mutex::new(0));
    let dup_effect_run_times = effect_run_times.clone();
    let dup_param = param.clone();
    let debounce_fn = debounce(
        move |param| {
            *dup_effect_run_times.lock().unwrap() += 1;
            *dup_param.lock().unwrap() = param;
        },
        std::time::Duration::from_millis(100),
    );
    {
        debounce_fn.call(1);
        debounce_fn.call(2);
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert_eq!(*effect_run_times.lock().unwrap(), 1);
        assert_eq!(*param.lock().unwrap(), 2);
    }

    {
        debounce_fn.call(3);
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert_eq!(*effect_run_times.lock().unwrap(), 2);
        assert_eq!(*param.lock().unwrap(), 3);
    }

    {
        debounce_fn.call(4);
        debounce_fn.terminate();
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert_eq!(*effect_run_times.lock().unwrap(), 2);
        assert_eq!(*param.lock().unwrap(), 3);
    }
}

#[test]
fn test_debounce_functionality() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);
    debounce.call(());
    debounce.call(());

    // Sleep more than debounce_delay to make sure the function has been called.
    std::thread::sleep(Duration::from_millis(150));

    // The function should have been called once.
    assert_eq!(*shared_count.lock().unwrap(), 1);

    // Call the function again and immediately terminate.
    debounce.call(());
    debounce.terminate();

    // The function should still have been called only once.
    assert_eq!(*shared_count.lock().unwrap(), 1);
}

#[test]
fn test_debounce_termination() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);
    debounce.call(());

    // Immediately terminate.
    debounce.terminate();

    // The function should have not been called yet.
    assert_eq!(*shared_count.lock().unwrap(), 0);

    // Wait longer than debounce_delay.
    std::thread::sleep(Duration::from_millis(150));

    // The function still should not have been called.
    assert_eq!(*shared_count.lock().unwrap(), 0);
}

#[test]
fn test_debounce_drop() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);
    debounce.call(());

    // Immediately drop.
    drop(debounce);

    // The function should have not been called yet.
    assert_eq!(*shared_count.lock().unwrap(), 0);

    // Wait longer than debounce_delay.
    std::thread::sleep(Duration::from_millis(150));

    // The function still should not have been called.
    assert_eq!(*shared_count.lock().unwrap(), 0);
}

#[test]
fn test_debounce_multiple_calls() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());

    // Sleep more than debounce_delay to make sure the function has been called.
    std::thread::sleep(Duration::from_millis(150));

    // The function should have been called once.
    assert_eq!(*shared_count.lock().unwrap(), 1);
}

#[test]
fn test_debounce_multiple_calls_with_termination() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());

    // Immediately terminate.
    debounce.terminate();

    // The function should have not been called yet.
    assert_eq!(*shared_count.lock().unwrap(), 0);

    // Wait longer than debounce_delay.
    std::thread::sleep(Duration::from_millis(150));

    // The function still should not have been called.
    assert_eq!(*shared_count.lock().unwrap(), 0);
}

#[test]
fn test_debounce_multiple_calls_with_drop() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());

    // Immediately drop.
    drop(debounce);

    // The function should have not been called yet.
    assert_eq!(*shared_count.lock().unwrap(), 0);

    // Wait longer than debounce_delay.
    std::thread::sleep(Duration::from_millis(150));

    // The function still should not have been called.
    assert_eq!(*shared_count.lock().unwrap(), 0);
}

#[test]
fn test_debounce_multiple_calls_with_termination_and_drop() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());
    debounce.call(());

    // Immediately terminate.
    debounce.terminate();

    // The function should have not been called yet.
    assert_eq!(*shared_count.lock().unwrap(), 0);

    // Immediately drop.
    drop(debounce);

    // Wait longer than debounce_delay.
    std::thread::sleep(Duration::from_millis(150));

    // The function still should not have been called.
    assert_eq!(*shared_count.lock().unwrap(), 0);
}

#[test]
fn test_performance() {
    let shared_count = Arc::new(Mutex::new(0));
    let debounce_delay = Duration::from_millis(100);
    let test_fn = {
        let shared_count = shared_count.clone();
        move |_| {
            let mut num = shared_count.lock().unwrap();
            *num += 1;
        }
    };

    let debounce = debounce(test_fn, debounce_delay);

    // Call the function 1000 times.
    for _ in 0..1000 {
        debounce.call(());
    }

    // Immediately terminate.
    debounce.terminate();

    // The function should have not been called yet.
    assert_eq!(*shared_count.lock().unwrap(), 0);

    // Wait longer than debounce_delay.
    std::thread::sleep(Duration::from_millis(150));

    // The function still should not have been called.
    assert_eq!(*shared_count.lock().unwrap(), 0);
}
