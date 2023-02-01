pub static INTERVAL: i64 = 600; // 10 minutes

pub fn get_rem_euclid(seconds: i64, interval: i64) -> i64 {
    if interval == 0 {
        return 0;
    }
    if seconds % interval == 0 {
        return seconds;
    }
    seconds - seconds.rem_euclid(interval)
}

pub fn get_key(producer: &str, seconds: i64, interval: i64) -> String {
    format!("{}:{}:{}", producer, interval, get_rem_euclid(seconds, interval))
}

#[test]
fn test_get_key() {
    assert_eq!("producer:86400:0", get_key("producer", 0, 86400));
    assert_eq!("producer:86400:86400", get_key("producer", 86400, 86400));
    assert_eq!("producer:86400:86400", get_key("producer", 100000, 86400));
    assert_eq!("producer:86400:172800", get_key("producer", 200000, 86400));
    assert_eq!("producer:86400:1528502400", get_key("producer", 1528512423, 86400));
}
