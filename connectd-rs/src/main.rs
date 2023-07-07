use std::{error::Error, ffi::CStr, future::pending, rc::Rc};
use zbus::{dbus_interface, ConnectionBuilder};

// functions from libsuseconnect.go
extern "C" {
    fn version(full: bool) -> *const libc::c_char;
    fn free_string(string: *const libc::c_char);
}

/// Safely handling strings from libsuseconnect
struct GoString {
    raw_str: *const libc::c_char,
    string: Rc<str>,
}

impl AsRef<str> for GoString {
    fn as_ref(&self) -> &str {
        &self.string
    }
}

impl From<*const libc::c_char> for GoString {
    fn from(value: *const libc::c_char) -> Self {
        unsafe {
            // TODO: create TryFrom to make this safer
            let string = CStr::from_ptr(value).to_str().unwrap();
            Self {
                string: string.into(),
                raw_str: value,
            }
        }
    }
}

impl From<GoString> for String {
    fn from(value: GoString) -> Self {
        value.string.as_ref().into()
    }
}

impl Drop for GoString {
    fn drop(&mut self) {
        unsafe { free_string(self.raw_str) };
    }
}

fn connect_version(full: bool) -> GoString {
    unsafe { version(full).into() }
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
        connect_version(full).into()
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
