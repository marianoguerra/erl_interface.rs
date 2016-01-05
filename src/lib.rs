extern crate libc;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod erl_interface;
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod ei_constants;
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
pub mod ei;

use std::fmt;

#[derive(Debug)]
pub enum EType {
    Int(::libc::c_int),
    UInt(::libc::c_uint),
    Float(::libc::c_double),
    Binary(Vec<::libc::c_uchar>),
    Atom(String),
    Pid { node: String, number: ::libc::c_uint,
          serial: ::libc::c_uint, creation: ::libc::c_uchar},
    Tuple { size: ::libc::c_int, items: Vec<EType> }
}

impl fmt::Display for EType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &EType::Int(val) => write!(f, "Int({})", val),
            &EType::UInt(val) => write!(f, "UInt({})", val),
            &EType::Float(val) => write!(f, "Float({})", val),
            &EType::Binary(ref val) => write!(f, "Binary({})", val.len()),
            &EType::Atom(ref val) => write!(f, "Atom({})", val),
            &EType::Pid { ref node, number, serial, creation } =>
                write!(f, "Pid(node: {}, number: {}, serial: {}, creation: {})",
                    node, number, serial, creation),

            &EType::Tuple { size, ref items } => {
                let _ = write!(f, "Tuple(size: {}, items: (", size);
                let mut count = 1;

                for item in items {
                    let _ = if count < size {
                        write!(f, "{}, ", item)
                    } else {
                        write!(f, "{}", item)
                    };

                    count += 1;
                }

                write!(f, "))")
            }
        }
    }
}

pub unsafe fn eterm_type_num(eterm: &mut erl_interface::ETERM) -> u8 {
    // here we transmute to ival just to access the header field which is
    // in all types of the enum, we don't know yet if it's an int
    let t = (*eterm.uval.ival()).h.type_and_count;
    ((t & 0xFF000000) >> 24) as u8
}

pub unsafe fn eterm_to_etype(eterm: &mut erl_interface::ETERM) -> EType {
    let tnum = eterm_type_num(eterm);
    match tnum {
        ei_constants::ERL_INTEGER => EType::Int((*eterm.uval.ival()).i),
        ei_constants::ERL_U_INTEGER => EType::UInt((*eterm.uval.uival()).u),
        ei_constants::ERL_FLOAT => EType::Float((*eterm.uval.fval()).f),
        ei_constants::ERL_ATOM => atom_to_etype((*eterm.uval.aval()).d),
        ei_constants::ERL_BINARY => {
            let bin_size = (*eterm.uval.bval()).size as usize;
            EType::Binary(Vec::from_raw_parts((*eterm.uval.bval()).b,
                            bin_size, bin_size))
        },
        ei_constants::ERL_TUPLE => {
            let size = (*eterm.uval.tval()).size;
            let mut items: Vec<EType> = vec!();

            for i in 0..size {
                let eterm_item = erl_interface::erl_element(i + 1, eterm);
                items.push(eterm_to_etype(&mut *eterm_item));
            }

            EType::Tuple { size: size, items: items }
        },
        ei_constants::ERL_PID => {
            let pid = *eterm.uval.pidval();
            match atom_to_etype(pid.node) {
                EType::Atom(node) => {
                    EType::Pid { node: node, number: pid.number,
                    serial: pid.serial,
                    creation: pid.creation }
                }
                other =>
                    panic!("Expected node to be atom, got {}", other)
            }
        },
        _ => panic!("Unknown type EType {}", tnum),
    }
}

pub unsafe fn atom_to_etype(atom: erl_interface::Erl_Atom_data) -> EType {
    let size = atom.lenU as usize;
    let s = Vec::from_raw_parts(atom.utf8 as *mut u8, size, size);
    EType::Atom(String::from_utf8(s).ok().unwrap())
}
