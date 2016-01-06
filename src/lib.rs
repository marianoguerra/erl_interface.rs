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
    Tuple { size: ::libc::c_int, items: Vec<EType> },
    List  { size: ::libc::c_int, items: Vec<EType> }
}


fn display_seq<T: fmt::Display>(name: &str, f: &mut fmt::Formatter,
               size: ::libc::c_int, items: &Vec<T>) -> fmt::Result {
    let _ = write!(f, "{}(size: {}, items: (", name, size);
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

impl fmt::Display for EType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &EType::Int(val) => write!(f, "Int({})", val),
            &EType::UInt(val) => write!(f, "UInt({})", val),
            &EType::Float(val) => write!(f, "Float({})", val),
            &EType::Binary(ref val) => {
                display_seq("Binary", f, val.len() as i32, val)
            },
            &EType::Atom(ref val) => write!(f, "Atom({})", val),
            &EType::Pid { ref node, number, serial, creation } =>
                write!(f, "Pid(node: {}, number: {}, serial: {}, creation: {})",
                    node, number, serial, creation),

            &EType::List { size, ref items } => {
                display_seq("List", f, size, items)
            },
            &EType::Tuple { size, ref items } => {
                display_seq("Tuple", f, size, items)
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

pub unsafe fn is_nil(eterm: &mut erl_interface::ETERM) -> bool {
    (eterm_type_num(eterm) == ei_constants::ERL_NIL)
}

pub unsafe fn etypes_to_eterms(items: &Vec<EType>) -> Vec<*mut erl_interface::ETERM> {
    let mut eterms: Vec<*mut erl_interface::ETERM> = vec!();
    for etype in items {
        eterms.push(etype_to_eterm(etype));
    }

    eterms
}

pub unsafe fn etype_to_eterm(etype: &EType) -> *mut erl_interface::ETERM {
    match etype {
        &EType::Int(val) => erl_interface::erl_mk_int(val),
        &EType::UInt(val) => erl_interface::erl_mk_uint(val),
        &EType::Float(val) => erl_interface::erl_mk_float(val),
        &EType::Binary(ref val) =>
            erl_interface::erl_mk_binary(val.as_ptr() as *const i8,
                                         val.len() as i32),
        &EType::Atom(ref val) => {
            erl_interface::erl_mk_atom(val.as_ptr() as *const i8)
        },
        &EType::Pid { ref node, number, serial, creation } => {
            erl_interface::erl_mk_pid(node.as_ptr() as *const i8, number, serial, creation)
        }
        &EType::List { size, ref items } => {
            let mut eterms = etypes_to_eterms(items);
            erl_interface::erl_mk_list(eterms.as_mut_ptr(), size)
        },
        &EType::Tuple { size, ref items } => {
            let mut eterms = etypes_to_eterms(items);
            erl_interface::erl_mk_tuple(eterms.as_mut_ptr(), size)
        }
    }
}

pub unsafe fn eterm_to_etype(eterm: &mut erl_interface::ETERM) -> EType {
    let tnum = eterm_type_num(eterm);
    match tnum {
        ei_constants::ERL_INTEGER => EType::Int((*eterm.uval.ival()).i),
        ei_constants::ERL_U_INTEGER => EType::UInt((*eterm.uval.uival()).u),
        ei_constants::ERL_FLOAT => EType::Float((*eterm.uval.fval()).f),
        ei_constants::ERL_ATOM => atom_to_etype(&mut (*eterm.uval.aval()).d),
        ei_constants::ERL_BINARY => {
            let bin_size = (*eterm.uval.bval()).size as usize;
            EType::Binary(Vec::from_raw_parts((*eterm.uval.bval()).b,
                            bin_size, bin_size))
        },
        ei_constants::ERL_NIL => {
            EType::List { size: 0, items: vec!() }
        },
        ei_constants::ERL_LIST => {
            let mut items: Vec<EType> = vec!();
            let mut elist = eterm;
            let mut size: ::libc::c_int = 0;

            while !is_nil(elist) {
                items.push(eterm_to_etype(&mut *erl_interface::erl_hd(elist)));
                elist = &mut *erl_interface::erl_tl(elist);
                size += 1;
            }


            EType::List { size: size, items: items }
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
            let mut node = pid.node;
            match atom_to_etype(&mut node) {
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

pub unsafe fn atom_to_etype(atom: &mut erl_interface::Erl_Atom_data) -> EType {
    let size = erl_interface::erl_atom_size_utf8(atom) as usize;
    let s = Vec::from_raw_parts(erl_interface::erl_atom_ptr_utf8(atom) as *mut u8, size, size);
    EType::Atom(String::from_utf8(s).ok().unwrap())
}
