//!
//! The Solidity compiler unit tests for factory dependencies.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_solidity::solc::codegen::Codegen as SolcCodegen;
use era_compiler_solidity::solc::Compiler as SolcCompiler;

use crate::common;

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn default_04_evmla() {
    default(semver::Version::new(0, 4, 26), SolcCodegen::EVMLA);
}
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn default_05_evmla() {
    default(semver::Version::new(0, 5, 17), SolcCodegen::EVMLA);
}
#[test]
fn default_06_evmla() {
    default(semver::Version::new(0, 6, 12), SolcCodegen::EVMLA);
}
#[test]
fn default_07_evmla() {
    default(semver::Version::new(0, 7, 6), SolcCodegen::EVMLA);
}
#[test]
fn default_08_evmla() {
    default(SolcCompiler::LAST_SUPPORTED_VERSION, SolcCodegen::EVMLA);
}
#[test]
fn default_08_yul() {
    default(SolcCompiler::LAST_SUPPORTED_VERSION, SolcCodegen::Yul);
}

pub const MAIN_CODE: &str = r#"
// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

import "./callable.sol";

contract Main {
    function main() external returns(uint256) {
        Callable callable = new Callable();

        callable.set(10);
        return callable.get();
    }
}
"#;

pub const CALLABLE_CODE: &str = r#"
// SPDX-License-Identifier: MIT

pragma solidity >=0.4.12;

contract Callable {
    uint256 value;

    function set(uint256 x) external {
        value = x;
    }

    function get() external view returns(uint256) {
        return value;
    }
}
"#;

fn default(version: semver::Version, pipeline: SolcCodegen) {
    let mut sources = BTreeMap::new();
    sources.insert("main.sol".to_owned(), MAIN_CODE.to_owned());
    sources.insert("callable.sol".to_owned(), CALLABLE_CODE.to_owned());

    let output = common::build_solidity(
        sources.clone(),
        BTreeMap::new(),
        BTreeSet::new(),
        &version,
        pipeline,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");

    assert_eq!(
        output
            .contracts
            .as_ref()
            .expect("Missing field `contracts`")
            .get("main.sol")
            .expect("Missing file `main.sol`")
            .get("Main")
            .expect("Missing contract `main.sol:Main`")
            .factory_dependencies
            .len(),
        1,
        "Expected 1 factory dependency in `main.sol:Main`"
    );
    assert_eq!(
        output
            .contracts
            .as_ref()
            .expect("Missing field `contracts`")
            .get("callable.sol")
            .expect("Missing file `callable.sol`")
            .get("Callable")
            .expect("Missing contract `callable.sol:Callable`")
            .factory_dependencies
            .len(),
        0,
        "Expected 0 factory dependencies in `callable.sol:Callable`"
    );
}
