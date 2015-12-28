extern crate libc;
extern crate erl_interface;

use erl_interface::erl_interface as eif;
use erl_interface::ei_constants as eic;

use std::env;
use std::ptr;
use std::ffi::CString;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        let program = args[0].clone();
        println!("Usage: {} <address> <cookie>", program);
        return;
    }

    let address = args[1].clone(); //"e1@ganesha";
    let cookie = args[2].clone(); //"secretcookie";
    const BUF_SIZE: i32 = 1024;
    // how do I put BUFSIZE here
    let mut buf = [0u8; BUF_SIZE as usize];

    let c_cookie = CString::new(cookie).unwrap();
    let c_node_address = CString::new(address).unwrap();

    let mut emsg = eif::ErlMessage::default();
    let mut done = false;

    unsafe {
        eif::erl_init(ptr::null_mut(), 0);
        if eif::erl_connect_init(1, c_cookie.into_raw(), 0) == -1 {
            panic!("erl_connect_init == -1");
        }

        let fd = eif::erl_connect(c_node_address.into_raw());

        if fd < 0 {
            panic!("erl_connect < 0");
        }

        println!("Connected to {}", args[1].clone());

        while !done {
            let got = eif::erl_receive_msg(fd, buf.as_mut_ptr(), BUF_SIZE, &mut emsg);
            if got == eic::ERL_TICK {
                println!("tick!");
                /* ignore */
            } else if got == eic::ERL_ERROR {
                println!("got error {}", got);
                done = true;
            } else {
                if emsg._type == eic::ERL_REG_SEND {
                    let t = (*(*emsg.msg).uval.ival()).h.type_and_count;
                    println!("got a send! {}", ((t & 0xFF000000) >> 24));
                } else {
                    println!("got something else");
                }
                done = true;
            }
        }
    }
}
