use std::time::Instant;

use once_cell::sync::Lazy;
use prometheus::{self, IntCounterVec, IntGaugeVec, Opts};

/// The delicate [`Histogram`] buckets.
/// The default buckets are tailored to broadly measure the response time (in
/// seconds) of a network service.
pub const DELICATE_BUCKETS: &[f64; 15] = &[0.00005, 0.0001, 0.0005, 0.001, 0.005, 0.01, 0.025,
                                           0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0];

/// current active/running/handling/pending rpc requests count
pub static RPC_HANDLING_COUNT: Lazy<IntGaugeVec> = Lazy::new(|| {
    let gauge = IntGaugeVec::new(Opts::new("delicate_server_rpc_handling_count",
                                           "Total number of handling rpc."),
                                 &["type"]).unwrap();
    prometheus::register(Box::new(gauge.clone())).unwrap();
    gauge
});

/// current handled rpc requests count
pub static RPC_HANDLED_COUNT: Lazy<IntCounterVec> = Lazy::new(|| {
    let counter = IntCounterVec::new(Opts::new("delicate_server_rpc_handled_count",
                                               "Total number of handled rpc."),
                                     &["type"]).expect("Init IntCounterVec failed!");
    prometheus::register(Box::new(counter.clone())).expect("register IntCounterVec failed!");
    counter
});

pub static RPC_EXECUTION_DURATION: Lazy<prometheus::HistogramVec> = Lazy::new(|| {
    prometheus::register_histogram_vec!("delicate_server_rpc_execution_duration_seconds",
                                        "Execution time(sec) of rpc.",
                                        &["type"],
                                        Vec::from(DELICATE_BUCKETS as &'static [f64])).unwrap()
});

/// Start to record rpc status
fn start_record_rpc_status(fn_name: &str) -> Instant {
    RPC_HANDLING_COUNT.with_label_values(&[fn_name]).inc();
    RPC_HANDLED_COUNT.with_label_values(&[fn_name]).inc();
    Instant::now()
}

/// Stop recording rpc status
fn stop_record_rpc_status(fn_name: &str, timer: Instant) {
    RPC_HANDLING_COUNT.with_label_values(&[fn_name]).dec();
    RPC_EXECUTION_DURATION.with_label_values(&[fn_name]).observe(timer.elapsed().as_secs_f64());
}

pub struct AutoRecorder {
    fn_name: String,
    start_at: Instant,
}

impl AutoRecorder {
    pub fn new(fn_name: &str) -> Self {
        let start_at = start_record_rpc_status(fn_name);
        Self { fn_name: fn_name.to_string(), start_at }
    }
}

impl Drop for AutoRecorder {
    fn drop(&mut self) {
        stop_record_rpc_status(&self.fn_name, self.start_at);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        RPC_HANDLING_COUNT.with_label_values(&["put key"]).inc();
        RPC_HANDLING_COUNT.with_label_values(&["get key"]).inc();
        RPC_HANDLING_COUNT.with_label_values(&["get key"]).inc();
        RPC_HANDLING_COUNT.with_label_values(&["get key"]).dec();

        RPC_EXECUTION_DURATION.with_label_values(&["put key"]).observe(0.000001);
        RPC_EXECUTION_DURATION.with_label_values(&["put key"]).observe(0.00003);
        RPC_EXECUTION_DURATION.with_label_values(&["put key"]).observe(0.003);
        RPC_EXECUTION_DURATION.with_label_values(&["put key"]).observe(3.0);
        RPC_EXECUTION_DURATION.with_label_values(&["put key"]).observe(5.0);

        let put_key_count =
            RPC_HANDLING_COUNT.get_metric_with_label_values(&["put key"]).unwrap().get();
        assert_eq!(put_key_count, 1);
        let get_key_count =
            RPC_HANDLING_COUNT.get_metric_with_label_values(&["get key"]).unwrap().get();
        assert_eq!(get_key_count, 1);

        let rpc_time_sum = RPC_EXECUTION_DURATION.get_metric_with_label_values(&["put key"])
                                                 .unwrap()
                                                 .get_sample_sum();

        assert!((rpc_time_sum - 8.003031).abs() < f64::EPSILON);
    }

    #[test]
    fn macro_test() {
        let sleep_sec = 0.01;
        let rpc_name: &str = "test_rpc";
        let timer = start_record_rpc_status(rpc_name);
        std::thread::sleep(std::time::Duration::from_secs_f64(sleep_sec));
        stop_record_rpc_status(rpc_name, timer);

        assert_eq!(RPC_HANDLING_COUNT.with_label_values(&[rpc_name]).get(), 0);
        assert_eq!(RPC_EXECUTION_DURATION.with_label_values(&[rpc_name]).get_sample_count(), 1);
        debug_assert!(RPC_EXECUTION_DURATION.with_label_values(&[rpc_name]).get_sample_sum()
                      >= sleep_sec as f64);
    }

    #[test]
    fn auto_recorder_test() {
        let sleep_sec = 0.01;
        let rpc_name = "test_rpc_with_auto_recorder";

        {
            let _recorder = AutoRecorder::new(rpc_name);
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_sec));
        }

        assert_eq!(RPC_HANDLING_COUNT.with_label_values(&[rpc_name]).get(), 0);
        assert_eq!(RPC_EXECUTION_DURATION.with_label_values(&[rpc_name]).get_sample_count(), 1);
        debug_assert!(RPC_EXECUTION_DURATION.with_label_values(&[rpc_name]).get_sample_sum()
                      >= sleep_sec as f64);
    }
}
