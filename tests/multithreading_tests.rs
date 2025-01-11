mod common;

#[test]
fn multiple_senders_one_receiver() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_udperf_receiver(Some(vec!["--port=45001".to_string()]));

    let args = vec!["sender", "--parallel=2", "--port=45001", "--multiplex-port-receiver=sharding"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn multiple_senders_multiple_receiver() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_udperf_receiver(Some(vec!["--port=45101".to_string(), "--parallel=2".to_string()]));

    let args = vec!["sender", "--parallel=2", "--port=45101"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}
