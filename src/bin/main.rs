extern crate erl_interface;
extern crate libc;

use std::ptr;
use std::ffi::CString;

fn main() {
    let address = "e1@ganesha";
    let cookie = "secretcookie";
    let erl_tick = 0;
    let erl_error = -1;
    let buf_size = 1024;
    // how do I put BUFSIZE here
    let mut buf = [0u8; 1024];

    let c_cookie = CString::new(cookie).unwrap();
    let c_node_address = CString::new(address).unwrap();

    let mut done = false;

    unsafe {
        let mut emsg = erl_interface::erl_interface::ErlMessage::default();
        erl_interface::erl_interface::erl_init(ptr::null_mut(), 0);
        if erl_interface::erl_interface::erl_connect_init(1, c_cookie.into_raw(), 0) == -1 {
            panic!("erl_connect_init == -1");
        }

        let fd = erl_interface::erl_interface::erl_connect(c_node_address.into_raw());

        if fd < 0 {
            panic!("erl_connect < 0");
        }

        println!("Connected to {}", address);

        while !done {
            let got = erl_interface::erl_interface::erl_receive_msg(fd, buf.as_mut_ptr(), buf_size, &mut emsg);
            if got == erl_tick {
                println!("tick!");
                /* ignore */
            } else if got == erl_error {
                println!("got error {}", got);
                done = true;
            } else {
                println!("got something else");
                done = true;
            }
        }
    }
}
