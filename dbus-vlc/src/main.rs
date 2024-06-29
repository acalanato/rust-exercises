use dbus::Message;
use dbus::blocking::{Connection, Proxy};

use dbus::message::SignalArgs;
use dbus::blocking::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged as PG;
use std::time::Duration;



fn connect2() {

    let c = Connection::new_session()?;
    c.request_name("com.example.dbustest", false, true, false)?;
    let mut cr = Crossroads::new();
    let token = cr.register("com.example.dbustest", |b| {
        b.method("Hello", ("name",), ("reply",), |_, _, (name,): (String,)| {
            Ok((format!("Hello {}!", name),))
        });
    });
    cr.insert("/hello", &[token], ());
    cr.serve(&c)?;
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
