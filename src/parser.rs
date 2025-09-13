use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RE_DIFF: Regex = Regex::new(r"diff (\d+)").unwrap();
    pub static ref RE_CPU: Regex  = Regex::new(r"\((\d+)\s+threads\)").unwrap();
    pub static ref RE_SPEED: Regex = Regex::new(
        r"speed\s+10s/60s/15m\s+(\d+\.?\d*|n/a)\s+(\d+\.?\d*|n/a)\s+(\d+\.?\d*|n/a)\s+H/s\s+max\s+(\d+\.?\d*|n/a)"
    ).unwrap();
}

pub fn parse_float_or_zero(s: &str) -> f64 {
    if s == "n/a" { 0.0 } else { s.parse::<f64>().unwrap_or(0.0) }
}
