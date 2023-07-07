use std::{error::Error, ffi::CStr, future::pending};
use zbus::{dbus_interface, ConnectionBuilder};

// functions from libsuseconnect.go
extern "C" {
    fn version(full: bool) -> *const libc::c_char;
    fn free_string(string: *const libc::c_char);
}

fn connect_version(full: bool) -> String {
    unsafe {
        let c_version = version(full);
        let cstr = CStr::from_ptr(c_version);
        // TODO: handle the error
        let version = cstr.to_str().unwrap().into();
        // golang allocates the strign so we need to free it
        free_string(c_version);
        version
    }
}

struct Greeter {
    count: u64,
}

#[dbus_interface(name = "com.github.suse.ConnectdRs")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }

    fn version(&mut self, full: bool) -> String {
        connect_version(full)
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let greeter = Greeter { count: 0 };
    let _conn = ConnectionBuilder::session()?
        .name("com.github.suse.ConnectdRs")?
        .serve_at("/com/github/suse/ConnectdRs", greeter)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
