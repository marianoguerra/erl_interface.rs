extern crate libc;
extern crate erl_interface;

use std::ptr;
use std::ffi::CString;

fn main() {
    let address = "e1@ganesha";
    let cookie = "secretcookie";
    const BUF_SIZE: i32 = 1024;
    // how do I put BUFSIZE here
    let mut buf = [0u8; BUF_SIZE as usize];

    let c_cookie = CString::new(cookie).unwrap();
    let c_node_address = CString::new(address).unwrap();

    let mut emsg = erl_interface::erl_interface::ErlMessage::default();
    let mut done = false;

    unsafe {
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
            let got = erl_interface::erl_interface::erl_receive_msg(fd, buf.as_mut_ptr(), BUF_SIZE, &mut emsg);
            if got == erl_interface::ei_constants::ERL_TICK {
                println!("tick!");
                /* ignore */
            } else if got == erl_interface::ei_constants::ERL_ERROR {
                println!("got error {}", got);
                done = true;
            } else {
                if emsg._type == erl_interface::ei_constants::ERL_REG_SEND {
                    println!("got a send!");
                } else {
                    println!("got something else");
                }
                done = true;
            }
        }
    }
}
