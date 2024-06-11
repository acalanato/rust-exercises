
fn main() {
    
    use dbus::blocking::Connection;
    use dbus::message::SignalArgs;
    use dbus::blocking::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged as PG;
    use std::time::Duration;

    let c = Connection::new_session().unwrap();

    let a = PG::match_rule(None, None); // rule out those args

    #[allow(unused_must_use)]
    c.add_match(a, |ir: PG, _, _| {
        println!("{:?}", ir.changed_properties); //remember to remove debug
        true
    });

// Wait for the signal to arrive.
    loop { c.process(Duration::from_millis(1000)).unwrap(); }
    
        
}
