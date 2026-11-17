//! Integration tests for SigmaOS-configuration-migration.

use sigmaos_configuration_migration_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
