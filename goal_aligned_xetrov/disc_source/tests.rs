#![cfg(test)]

use super::scenarios::*;
use super::types::*;

// Number of each test to execute.
pub const NUM_RUNS: u32 = 1_000;

// Tests whether disc identifies that sample is inside it.
#[test]
fn test_case1() {
    for scenario in create_scenarios(NUM_RUNS, case1_scenario).iter() {
        match scenario.disc.source(scenario.sample) {
            SourceResult::Case1(_) => (),
            _ => assert!(false)
        };
    }
}

// Tests whether disc identifies that sample is outside it.
#[test]
fn test_case2() {
    for scenario in create_scenarios(NUM_RUNS, case2_scenario).iter() {
        match scenario.disc.source(scenario.sample) {
            SourceResult::Case2(_) => (),
            _ => assert!(false)
        };
    }
}
