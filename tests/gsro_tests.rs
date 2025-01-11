mod common;

// Test sender sendmsg/sendmmsg with receiver sendmsg/sendmmsg in different combinations
#[test]
fn gro_no_gso() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_nperf_receiver(Some(vec!["--with-gsro".to_string(), "--port=45001".to_string()]));

    let args = vec!["sender",  "--port=45001"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn gso_no_gro() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_nperf_receiver(Some(vec!["--port=45101".to_string()]));

    let args = vec!["sender", "--with-gsro", "--port=45101"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}

#[test]
fn gso_gro() -> Result<(), Box<dyn std::error::Error>>{
    let handle = common::start_nperf_receiver(Some(vec!["--with-gsro".to_string(), "--port=45201".to_string()]));

    let args = vec!["sender", "--with-gsro", "--port=45201"];
    let udperf = udperf::udperf::new().set_args(args);
    let arguments = udperf.parse_parameter().unwrap();
    if let Some(x) = udperf.exec(arguments) {
        assert!(x.amount_datagrams > 10000);
    };

    handle.join().unwrap();
    Ok(())
}
