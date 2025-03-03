mod common;

#[test]
fn uring_fillmode_topup() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_udperf_sender(Some(vec!["--port=45001".to_string(), "--with-gsro".to_string()]));

    let args = vec!["receiver", "--io-model=io-uring", "--port=45001", "--uring-sq-mode=topup"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn uring_fillmode_syscall() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_udperf_sender(Some(vec!["--port=45002".to_string(), "--with-gsro".to_string()]));

    let args = vec!["receiver", "--io-model=io-uring", "--port=45002", "--uring-sq-mode=syscall"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}
