use dbus::Message;
use dbus::blocking::{Connection, Proxy};

use dbus::message::SignalArgs;
use dbus::blocking::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged as PG;
use std::time::Duration;



fn connect2() -> Result<(), Box<dyn std::error::Error>> {

    let c = Connection::new_session()?;
    let p = c.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));
    let (names,): (Vec<String>,) = p.method_call("org.freedesktop.DBus", "ListNames", ())?;
    for name in names {println!("{}", name)}
}

fn connect1(){
    let c = Connection::new_session().unwrap();
    let bus = Message::;
    let a = PG::match_rule(bus, None); // rule out those args

    c.add_match(a, |ir: PG, _, _| {
        println!("{:?}", ir.changed_properties);
        true})
        .ok().
        expect(""); //Write error msg

// Wait for the signal to arrive.
    loop { c.process(Duration::from_millis(1000)).unwrap(); }
}

fn main() {
    connect2();
}
