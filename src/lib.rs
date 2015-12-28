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

pub unsafe fn eterm_type_num(eterm: &mut erl_interface::ETERM) -> u8 {
    // here we transmute to ival just to access the header field which is
    // in all types of the enum, we don't know yet if it's an int
    let t = (*eterm.uval.ival()).h.type_and_count;
    ((t & 0xFF000000) >> 24) as u8
}
