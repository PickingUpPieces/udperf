use udperf::udperf;

fn main() {
    let udperf = udperf::new();

    let parameter = match udperf.parse_parameter() {
        Some(x) => x,
        None => { return },
    };

    udperf.exec(parameter);
}