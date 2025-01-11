mod common;

#[test]
fn test_sender_send() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_udperf_receiver(Some(vec!["--port=45001".to_string()]));

    let args = vec!["sender", "--exchange-function=normal", "--port=45001"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn test_sender_sendmsg() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_udperf_receiver(Some(vec!["--port=45101".to_string()]));

    let args = vec!["sender", "--port=45101"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn test_sender_sendmmsg() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_udperf_receiver(Some(vec!["--port=45201".to_string()]));

    let args = vec!["sender", "--exchange-function=mmsg", "--with-mmsg-amount=20", "--port=45201"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}