/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub type_and_count : ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed1 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Header = Struct_Unnamed1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub h: Erl_Header,
    pub i: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Integer = Struct_Unnamed2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub h: Erl_Header,
    pub u: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Uinteger = Struct_Unnamed3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub h: Erl_Header,
    pub i: ::libc::c_longlong,
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_LLInteger = Struct_Unnamed4;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub h: Erl_Header,
    pub u: ::libc::c_ulonglong,
}
impl ::std::clone::Clone for Struct_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_ULLInteger = Struct_Unnamed5;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub h: Erl_Header,
    pub f: ::libc::c_double,
}
impl ::std::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Float = Struct_Unnamed6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub utf8: *mut ::libc::c_char,
    pub lenU: ::libc::c_int,
    pub latin1: *mut ::libc::c_char,
    pub lenL: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Atom_data = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub h: Erl_Header,
    pub d: Erl_Atom_data,
}
impl ::std::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Atom = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub h: Erl_Header,
    pub node: Erl_Atom_data,
    pub number: ::libc::c_uint,
    pub serial: ::libc::c_uint,
    pub creation: ::libc::c_uchar,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Pid = Struct_Unnamed9;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed10 {
    pub h: Erl_Header,
    pub node: Erl_Atom_data,
    pub number: ::libc::c_uint,
    pub creation: ::libc::c_uchar,
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Port = Struct_Unnamed10;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub h: Erl_Header,
    pub node: Erl_Atom_data,
    pub len: ::libc::c_int,
    pub n: [::libc::c_uint; 3usize],
    pub creation: ::libc::c_uchar,
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Ref = Struct_Unnamed11;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub h: Erl_Header,
    pub arity: ::libc::c_int,
    pub is_neg: ::libc::c_int,
    pub digits: *mut ::libc::c_ushort,
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Big = Struct_Unnamed12;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed13 {
    pub h: Erl_Header,
    pub head: *mut Struct__eterm,
    pub tail: *mut Struct__eterm,
}
impl ::std::clone::Clone for Struct_Unnamed13 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed13 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_List = Struct_Unnamed13;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub h: Erl_Header,
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_EmptyList = Struct_Unnamed14;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed15 {
    pub h: Erl_Header,
    pub size: ::libc::c_int,
    pub elems: *mut *mut Struct__eterm,
}
impl ::std::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Tuple = Struct_Unnamed15;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed16 {
    pub h: Erl_Header,
    pub size: ::libc::c_int,
    pub b: *mut ::libc::c_uchar,
}
impl ::std::clone::Clone for Struct_Unnamed16 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Binary = Struct_Unnamed16;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed17 {
    pub h: Erl_Header,
    pub len: ::libc::c_int,
    pub name: *mut ::libc::c_char,
    pub v: *mut Struct__eterm,
}
impl ::std::clone::Clone for Struct_Unnamed17 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed17 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Variable = Struct_Unnamed17;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed18 {
    pub h: Erl_Header,
    pub size: ::libc::c_int,
    pub arity: ::libc::c_int,
    pub md5: [::libc::c_uchar; 16usize],
    pub new_index: ::libc::c_int,
    pub creator: *mut Struct__eterm,
    pub module: *mut Struct__eterm,
    pub index: *mut Struct__eterm,
    pub uniq: *mut Struct__eterm,
    pub closure: *mut *mut Struct__eterm,
}
impl ::std::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed18 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Erl_Function = Struct_Unnamed18;
#[repr(C)]
#[derive(Copy)]
pub struct Struct__eterm {
    pub uval: Union_Unnamed19,
}
impl ::std::clone::Clone for Struct__eterm {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct__eterm {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed19 {
    pub _bindgen_data_: [u64; 9usize],
}
impl Union_Unnamed19 {
    pub unsafe fn ival(&mut self) -> *mut Erl_Integer {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn uival(&mut self) -> *mut Erl_Uinteger {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn llval(&mut self) -> *mut Erl_LLInteger {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ullval(&mut self) -> *mut Erl_ULLInteger {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn fval(&mut self) -> *mut Erl_Float {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn aval(&mut self) -> *mut Erl_Atom {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn pidval(&mut self) -> *mut Erl_Pid {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn portval(&mut self) -> *mut Erl_Port {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn refval(&mut self) -> *mut Erl_Ref {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn lval(&mut self) -> *mut Erl_List {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn nval(&mut self) -> *mut Erl_EmptyList {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn tval(&mut self) -> *mut Erl_Tuple {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn bval(&mut self) -> *mut Erl_Binary {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn vval(&mut self) -> *mut Erl_Variable {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn funcval(&mut self) -> *mut Erl_Function {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn bigval(&mut self) -> *mut Erl_Big {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed19 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed19 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ETERM = Struct__eterm;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed20 {
    pub _type: ::libc::c_int,
    pub msg: *mut ETERM,
    pub from: *mut ETERM,
    pub to: *mut ETERM,
    pub to_name: [::libc::c_char; 1021usize],
}
impl ::std::clone::Clone for Struct_Unnamed20 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed20 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ErlMessage = Struct_Unnamed20;
pub type Erl_Heap = ::libc::c_uchar;
#[link(name = "erl_interface")]
extern "C" {
    pub fn erl_atom_ptr_latin1(arg1: *mut Erl_Atom_data)
     -> *mut ::libc::c_char;
    pub fn erl_atom_ptr_utf8(arg1: *mut Erl_Atom_data) -> *mut ::libc::c_char;
    pub fn erl_atom_size_latin1(arg1: *mut Erl_Atom_data) -> ::libc::c_int;
    pub fn erl_atom_size_utf8(arg1: *mut Erl_Atom_data) -> ::libc::c_int;
    pub fn erl_atom_init_latin1(arg1: *mut Erl_Atom_data,
                                arg2: *const ::libc::c_char)
     -> *mut ::libc::c_char;
    pub fn erl_init(x: *mut ::libc::c_void, y: ::libc::c_long) -> ();
    pub fn erl_set_compat_rel(arg1: ::libc::c_uint) -> ();
    pub fn erl_connect_init(arg1: ::libc::c_int, arg2: *mut ::libc::c_char,
                            arg3: ::libc::c_short) -> ::libc::c_int;
    pub fn erl_connect_xinit(arg1: *mut ::libc::c_char,
                             arg2: *mut ::libc::c_char,
                             arg3: *mut ::libc::c_char,
                             arg4: *mut ::libc::in_addr,
                             arg5: *mut ::libc::c_char, arg6: ::libc::c_short)
     -> ::libc::c_int;
    pub fn erl_connect(arg1: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn erl_xconnect(arg1: *mut ::libc::in_addr, arg2: *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn erl_close_connection(arg1: ::libc::c_int) -> ::libc::c_int;
    pub fn erl_receive(arg1: ::libc::c_int, arg2: *mut ::libc::c_uchar,
                       arg3: ::libc::c_int) -> ::libc::c_int;
    pub fn erl_receive_msg(arg1: ::libc::c_int, arg2: *mut ::libc::c_uchar,
                           arg3: ::libc::c_int, arg4: *mut ErlMessage)
     -> ::libc::c_int;
    pub fn erl_xreceive_msg(arg1: ::libc::c_int,
                            arg2: *mut *mut ::libc::c_uchar,
                            arg3: *mut ::libc::c_int, arg4: *mut ErlMessage)
     -> ::libc::c_int;
    pub fn erl_send(arg1: ::libc::c_int, arg2: *mut ETERM, arg3: *mut ETERM)
     -> ::libc::c_int;
    pub fn erl_reg_send(arg1: ::libc::c_int, arg2: *mut ::libc::c_char,
                        arg3: *mut ETERM) -> ::libc::c_int;
    pub fn erl_rpc(arg1: ::libc::c_int, arg2: *mut ::libc::c_char,
                   arg3: *mut ::libc::c_char, arg4: *mut ETERM) -> *mut ETERM;
    pub fn erl_rpc_to(arg1: ::libc::c_int, arg2: *mut ::libc::c_char,
                      arg3: *mut ::libc::c_char, arg4: *mut ETERM)
     -> ::libc::c_int;
    pub fn erl_rpc_from(arg1: ::libc::c_int, arg2: ::libc::c_int,
                        arg3: *mut ErlMessage) -> ::libc::c_int;
    pub fn erl_publish(port: ::libc::c_int) -> ::libc::c_int;
    /* TODO: expose this one */
    /* pub fn erl_accept(arg1: ::libc::c_int, arg2: *mut ErlConnect)
     -> ::libc::c_int; */
    pub fn erl_thiscookie() -> *const ::libc::c_char;
    pub fn erl_thisnodename() -> *const ::libc::c_char;
    pub fn erl_thishostname() -> *const ::libc::c_char;
    pub fn erl_thisalivename() -> *const ::libc::c_char;
    pub fn erl_thiscreation() -> ::libc::c_short;
    pub fn erl_unpublish(alive: *const ::libc::c_char) -> ::libc::c_int;
    pub fn erl_err_msg(__template: *const ::libc::c_char, ...) -> ();
    pub fn erl_err_quit(__template: *const ::libc::c_char, ...) -> ();
    pub fn erl_err_ret(__template: *const ::libc::c_char, ...) -> ();
    pub fn erl_err_sys(__template: *const ::libc::c_char, ...) -> ();
    pub fn erl_cons(arg1: *mut ETERM, arg2: *mut ETERM) -> *mut ETERM;
    pub fn erl_copy_term(arg1: *const ETERM) -> *mut ETERM;
    pub fn erl_element(arg1: ::libc::c_int, arg2: *const ETERM) -> *mut ETERM;
    pub fn erl_hd(arg1: *const ETERM) -> *mut ETERM;
    pub fn erl_iolist_to_binary(term: *const ETERM) -> *mut ETERM;
    pub fn erl_iolist_to_string(term: *const ETERM) -> *mut ::libc::c_char;
    pub fn erl_iolist_length(arg1: *const ETERM) -> ::libc::c_int;
    pub fn erl_length(arg1: *const ETERM) -> ::libc::c_int;
    pub fn erl_mk_atom(arg1: *const ::libc::c_char) -> *mut ETERM;
    pub fn erl_mk_binary(arg1: *const ::libc::c_char, arg2: ::libc::c_int)
     -> *mut ETERM;
    pub fn erl_mk_empty_list() -> *mut ETERM;
    pub fn erl_mk_estring(arg1: *const ::libc::c_char, arg2: ::libc::c_int)
     -> *mut ETERM;
    pub fn erl_mk_float(arg1: ::libc::c_double) -> *mut ETERM;
    pub fn erl_mk_int(arg1: ::libc::c_int) -> *mut ETERM;
    pub fn erl_mk_longlong(arg1: ::libc::c_longlong) -> *mut ETERM;
    pub fn erl_mk_list(arg1: *mut *mut ETERM, arg2: ::libc::c_int)
     -> *mut ETERM;
    pub fn erl_mk_pid(arg1: *const ::libc::c_char, arg2: ::libc::c_uint,
                      arg3: ::libc::c_uint, arg4: ::libc::c_uchar)
     -> *mut ETERM;
    pub fn erl_mk_port(arg1: *const ::libc::c_char, arg2: ::libc::c_uint,
                       arg3: ::libc::c_uchar) -> *mut ETERM;
    pub fn erl_mk_ref(arg1: *const ::libc::c_char, arg2: ::libc::c_uint,
                      arg3: ::libc::c_uchar) -> *mut ETERM;
    pub fn erl_mk_long_ref(arg1: *const ::libc::c_char, arg2: ::libc::c_uint,
                           arg3: ::libc::c_uint, arg4: ::libc::c_uint,
                           arg5: ::libc::c_uchar) -> *mut ETERM;
    pub fn erl_mk_string(arg1: *const ::libc::c_char) -> *mut ETERM;
    pub fn erl_mk_tuple(arg1: *mut *mut ETERM, arg2: ::libc::c_int)
     -> *mut ETERM;
    pub fn erl_mk_uint(arg1: ::libc::c_uint) -> *mut ETERM;
    pub fn erl_mk_ulonglong(arg1: ::libc::c_ulonglong) -> *mut ETERM;
    pub fn erl_mk_var(arg1: *const ::libc::c_char) -> *mut ETERM;
    pub fn erl_print_term(arg1: *mut ::libc::FILE, arg2: *const ETERM)
     -> ::libc::c_int;
    pub fn erl_size(arg1: *const ETERM) -> ::libc::c_int;
    pub fn erl_tl(arg1: *const ETERM) -> *mut ETERM;
    pub fn erl_var_content(arg1: *const ETERM, arg2: *const ::libc::c_char)
     -> *mut ETERM;
    pub fn erl_format(arg1: *mut ::libc::c_char, ...) -> *mut ETERM;
    pub fn erl_match(arg1: *mut ETERM, arg2: *mut ETERM) -> ::libc::c_int;
    pub fn erl_global_names(fd: ::libc::c_int, count: *mut ::libc::c_int)
     -> *mut *mut ::libc::c_char;
    pub fn erl_global_register(fd: ::libc::c_int, name: *const ::libc::c_char,
                               pid: *mut ETERM) -> ::libc::c_int;
    pub fn erl_global_unregister(fd: ::libc::c_int,
                                 name: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn erl_global_whereis(fd: ::libc::c_int, name: *const ::libc::c_char,
                              node: *mut ::libc::c_char) -> *mut ETERM;
    pub fn erl_init_malloc(arg1: *mut Erl_Heap, arg2: ::libc::c_long) -> ();
    pub fn erl_alloc_eterm(arg1: ::libc::c_uchar) -> *mut ETERM;
    pub fn erl_eterm_release() -> ();
    pub fn erl_eterm_statistics(arg1: *mut ::libc::c_ulong,
                                arg2: *mut ::libc::c_ulong) -> ();
    pub fn erl_free_array(arg1: *mut *mut ETERM, arg2: ::libc::c_int) -> ();
    pub fn erl_free_term(arg1: *mut ETERM) -> ();
    pub fn erl_free_compound(arg1: *mut ETERM) -> ();
    pub fn erl_malloc(arg1: ::libc::c_long) -> *mut ::libc::c_void;
    pub fn erl_free(arg1: *mut ::libc::c_void) -> ();
    pub fn erl_compare_ext(arg1: *mut ::libc::c_uchar,
                           arg2: *mut ::libc::c_uchar) -> ::libc::c_int;
    pub fn erl_decode(arg1: *mut ::libc::c_uchar) -> *mut ETERM;
    pub fn erl_decode_buf(arg1: *mut *mut ::libc::c_uchar) -> *mut ETERM;
    pub fn erl_encode(arg1: *mut ETERM, t: *mut ::libc::c_uchar)
     -> ::libc::c_int;
    pub fn erl_encode_buf(arg1: *mut ETERM, arg2: *mut *mut ::libc::c_uchar)
     -> ::libc::c_int;
    pub fn erl_ext_size(arg1: *mut ::libc::c_uchar) -> ::libc::c_int;
    pub fn erl_ext_type(arg1: *mut ::libc::c_uchar) -> ::libc::c_uchar;
    pub fn erl_peek_ext(arg1: *mut ::libc::c_uchar, arg2: ::libc::c_int)
     -> *mut ::libc::c_uchar;
    pub fn erl_term_len(arg1: *mut ETERM) -> ::libc::c_int;
    pub fn cmp_latin1_vs_utf8(sL: *const ::libc::c_char, lenL: ::libc::c_int,
                              sU: *const ::libc::c_char, lenU: ::libc::c_int)
     -> ::libc::c_int;
    pub fn erl_gethostbyname(name: *const ::libc::c_char)
     -> *mut ::libc::hostent;
    pub fn erl_gethostbyaddr(addr: *const ::libc::c_char, len: ::libc::c_int,
                             _type: ::libc::c_int) -> *mut ::libc::hostent;
    pub fn erl_gethostbyname_r(name: *const ::libc::c_char,
                               hostp: *mut ::libc::hostent,
                               buffer: *mut ::libc::c_char,
                               buflen: ::libc::c_int,
                               h_errnop: *mut ::libc::c_int)
     -> *mut ::libc::hostent;
    pub fn erl_gethostbyaddr_r(addr: *const ::libc::c_char,
                               length: ::libc::c_int, _type: ::libc::c_int,
                               hostp: *mut ::libc::hostent,
                               buffer: *mut ::libc::c_char,
                               buflen: ::libc::c_int,
                               h_errnop: *mut ::libc::c_int)
     -> *mut ::libc::hostent;
    pub fn erl_init_resolve() -> ();
    pub fn erl_distversion(fd: ::libc::c_int) -> ::libc::c_int;
    pub fn erl_epmd_connect(inaddr: *mut ::libc::in_addr) -> ::libc::c_int;
    pub fn erl_epmd_port(inaddr: *mut ::libc::in_addr,
                         alive: *const ::libc::c_char,
                         dist: *mut ::libc::c_int) -> ::libc::c_int;
}
