#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    stylus_research_counter::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}