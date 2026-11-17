// Integration test for SigmaOS-configuration-migration
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use configuration_migration as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
