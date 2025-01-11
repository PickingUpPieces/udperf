mod common;

// Test sender send/sendmsg/sendmmsg with receiver send/sendmsg/sendmmsg in different combinations

#[test]
fn sendmsg_recvmsg() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_nperf_receiver(Some(vec!["--port=45001".to_string()]));

    let args = vec!["sender", "--port=45001"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn sendmmsg_recvmsg() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_nperf_receiver(Some(vec!["--port=45101".to_string()]));

    let args = vec!["sender", "--exchange-function=mmsg", "--port=45101"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn sendmmsg_recvmmsg() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_nperf_receiver(Some(vec!["--exchange-function=mmsg".to_string(), "--port=45201".to_string()]));

    let args = vec!["sender", "--exchange-function=mmsg", "--port=45201"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn sendmsg_recvmmsg() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_nperf_receiver(Some(vec!["--exchange-function=mmsg".to_string(), "--port=45301".to_string()]));

    let args = vec!["sender", "--port=45301"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

