/* automatically generated by rust-bindgen */

pub type Enum_Unnamed1 = ::libc::c_uint;
pub const ERLANG_ASCII: ::libc::c_uint = 1;
pub const ERLANG_LATIN1: ::libc::c_uint = 2;
pub const ERLANG_UTF8: ::libc::c_uint = 4;
pub type erlang_char_encoding = Enum_Unnamed1;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed2 {
    pub node: [::libc::c_char; 1021usize],
    pub num: ::libc::c_uint,
    pub serial: ::libc::c_uint,
    pub creation: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed2 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type erlang_pid = Struct_Unnamed2;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed3 {
    pub node: [::libc::c_char; 1021usize],
    pub id: ::libc::c_uint,
    pub creation: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed3 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type erlang_port = Struct_Unnamed3;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub node: [::libc::c_char; 1021usize],
    pub len: ::libc::c_int,
    pub n: [::libc::c_uint; 3usize],
    pub creation: ::libc::c_uint,
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type erlang_ref = Struct_Unnamed4;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed5 {
    pub serial: ::libc::c_long,
    pub prev: ::libc::c_long,
    pub from: erlang_pid,
    pub label: ::libc::c_long,
    pub flags: ::libc::c_long,
}
impl ::std::clone::Clone for Struct_Unnamed5 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed5 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type erlang_trace = Struct_Unnamed5;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed6 {
    pub msgtype: ::libc::c_long,
    pub from: erlang_pid,
    pub to: erlang_pid,
    pub toname: [::libc::c_char; 1021usize],
    pub cookie: [::libc::c_char; 1021usize],
    pub token: erlang_trace,
}
impl ::std::clone::Clone for Struct_Unnamed6 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed6 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type erlang_msg = Struct_Unnamed6;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed7 {
    pub arity: ::libc::c_long,
    pub module: [::libc::c_char; 1021usize],
    pub module_org_enc: erlang_char_encoding,
    pub md5: [::libc::c_char; 16usize],
    pub index: ::libc::c_long,
    pub old_index: ::libc::c_long,
    pub uniq: ::libc::c_long,
    pub n_free_vars: ::libc::c_long,
    pub pid: erlang_pid,
    pub free_var_len: ::libc::c_long,
    pub free_vars: *mut ::libc::c_char,
}
impl ::std::clone::Clone for Struct_Unnamed7 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed7 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type erlang_fun = Struct_Unnamed7;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed8 {
    pub arity: ::libc::c_uint,
    pub is_neg: ::libc::c_int,
    pub digits: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct_Unnamed8 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed8 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type erlang_big = Struct_Unnamed8;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub ei_type: ::libc::c_char,
    pub arity: ::libc::c_int,
    pub size: ::libc::c_int,
    pub value: Union_Unnamed10,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed10 {
    pub _bindgen_data_: [u64; 131usize],
}
impl Union_Unnamed10 {
    pub unsafe fn i_val(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn d_val(&mut self) -> *mut ::libc::c_double {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn atom_name(&mut self) -> *mut [::libc::c_char; 1021usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn pid(&mut self) -> *mut erlang_pid {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn port(&mut self) -> *mut erlang_port {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn _ref(&mut self) -> *mut erlang_ref {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ei_term = Struct_Unnamed9;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub ipadr: [::libc::c_char; 4usize],
    pub nodename: [::libc::c_char; 129usize],
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ErlConnect = Struct_Unnamed11;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ei_cnode_s {
    pub thishostname: [::libc::c_char; 65usize],
    pub thisnodename: [::libc::c_char; 129usize],
    pub thisalivename: [::libc::c_char; 64usize],
    pub ei_connect_cookie: [::libc::c_char; 513usize],
    pub creation: ::libc::c_short,
    pub _self: erlang_pid,
}
impl ::std::clone::Clone for Struct_ei_cnode_s {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ei_cnode_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ei_cnode = Struct_ei_cnode_s;
pub type Erl_IpAddr = *mut ::libc::in_addr;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ei_x_buff_TAG {
    pub buff: *mut ::libc::c_char,
    pub buffsz: ::libc::c_int,
    pub index: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_ei_x_buff_TAG {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ei_x_buff_TAG {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ei_x_buff = Struct_ei_x_buff_TAG;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_bucket_s {
    pub rawhash: ::libc::c_int,
    pub key: *const ::libc::c_char,
    pub keybuf: [::libc::c_char; 32usize],
    pub value: *const ::libc::c_void,
    pub next: *mut Struct_bucket_s,
}
impl ::std::clone::Clone for Struct_bucket_s {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_bucket_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ei_bucket = Struct_bucket_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub tab: *mut *mut ei_bucket,
    pub hash: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                             *const ::libc::c_char)
                                        -> ::libc::c_int>,
    pub size: ::libc::c_int,
    pub nelem: ::libc::c_int,
    pub npos: ::libc::c_int,
    pub freelist: *mut ei_bucket,
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ei_hash = Struct_Unnamed12;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ei_reg_inode {
    pub attr: ::libc::c_int,
    pub size: ::libc::c_int,
    pub val: Union_Unnamed13,
    pub next: *mut Struct_ei_reg_inode,
}
impl ::std::clone::Clone for Struct_ei_reg_inode {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ei_reg_inode {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed13 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed13 {
    pub unsafe fn i(&mut self) -> *mut ::libc::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn f(&mut self) -> *mut ::libc::c_double {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn s(&mut self) -> *mut *mut ::libc::c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn p(&mut self) -> *mut *mut ::libc::c_void {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed13 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed13 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ei_reg_obj = Struct_ei_reg_inode;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub freelist: *mut ei_reg_obj,
    pub tab: *mut ei_hash,
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type ei_reg = Struct_Unnamed14;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ei_reg_stat {
    pub attr: ::libc::c_int,
    pub size: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_ei_reg_stat {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ei_reg_stat {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ei_reg_tabstat {
    pub size: ::libc::c_int,
    pub nelem: ::libc::c_int,
    pub npos: ::libc::c_int,
    pub collisions: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_ei_reg_tabstat {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_ei_reg_tabstat {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "ei")]
extern "C" {
    pub static mut __erl_errno: ::libc::c_int;
}
#[link(name = "ei")]
extern "C" {
    pub fn ei_connect_init(ec: *mut ei_cnode,
                           this_node_name: *const ::libc::c_char,
                           cookie: *const ::libc::c_char,
                           creation: ::libc::c_short) -> ::libc::c_int;
    pub fn ei_connect_xinit(ec: *mut ei_cnode,
                            thishostname: *const ::libc::c_char,
                            thisalivename: *const ::libc::c_char,
                            thisnodename: *const ::libc::c_char,
                            thisipaddr: Erl_IpAddr,
                            cookie: *const ::libc::c_char,
                            creation: ::libc::c_short) -> ::libc::c_int;
    pub fn ei_connect(ec: *mut ei_cnode, nodename: *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_connect_tmo(ec: *mut ei_cnode, nodename: *mut ::libc::c_char,
                          ms: ::libc::c_uint) -> ::libc::c_int;
    pub fn ei_xconnect(ec: *mut ei_cnode, adr: Erl_IpAddr,
                       alivename: *mut ::libc::c_char) -> ::libc::c_int;
    pub fn ei_xconnect_tmo(ec: *mut ei_cnode, adr: Erl_IpAddr,
                           alivename: *mut ::libc::c_char, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_receive(fd: ::libc::c_int, bufp: *mut ::libc::c_uchar,
                      bufsize: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_receive_tmo(fd: ::libc::c_int, bufp: *mut ::libc::c_uchar,
                          bufsize: ::libc::c_int, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_receive_msg(fd: ::libc::c_int, msg: *mut erlang_msg,
                          x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_receive_msg_tmo(fd: ::libc::c_int, msg: *mut erlang_msg,
                              x: *mut ei_x_buff, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_xreceive_msg(fd: ::libc::c_int, msg: *mut erlang_msg,
                           x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_xreceive_msg_tmo(fd: ::libc::c_int, msg: *mut erlang_msg,
                               x: *mut ei_x_buff, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_send(fd: ::libc::c_int, to: *mut erlang_pid,
                   buf: *mut ::libc::c_char, len: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_send_tmo(fd: ::libc::c_int, to: *mut erlang_pid,
                       buf: *mut ::libc::c_char, len: ::libc::c_int,
                       ms: ::libc::c_uint) -> ::libc::c_int;
    pub fn ei_reg_send(ec: *mut ei_cnode, fd: ::libc::c_int,
                       server_name: *mut ::libc::c_char,
                       buf: *mut ::libc::c_char, len: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_reg_send_tmo(ec: *mut ei_cnode, fd: ::libc::c_int,
                           server_name: *mut ::libc::c_char,
                           buf: *mut ::libc::c_char, len: ::libc::c_int,
                           ms: ::libc::c_uint) -> ::libc::c_int;
    pub fn ei_rpc(ec: *mut ei_cnode, fd: ::libc::c_int,
                  _mod: *mut ::libc::c_char, fun: *mut ::libc::c_char,
                  inbuf: *const ::libc::c_char, inbuflen: ::libc::c_int,
                  x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_rpc_to(ec: *mut ei_cnode, fd: ::libc::c_int,
                     _mod: *mut ::libc::c_char, fun: *mut ::libc::c_char,
                     buf: *const ::libc::c_char, len: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_rpc_from(ec: *mut ei_cnode, fd: ::libc::c_int,
                       timeout: ::libc::c_int, msg: *mut erlang_msg,
                       x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_publish(ec: *mut ei_cnode, port: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_publish_tmo(ec: *mut ei_cnode, port: ::libc::c_int,
                          ms: ::libc::c_uint) -> ::libc::c_int;
    pub fn ei_accept(ec: *mut ei_cnode, lfd: ::libc::c_int,
                     conp: *mut ErlConnect) -> ::libc::c_int;
    pub fn ei_accept_tmo(ec: *mut ei_cnode, lfd: ::libc::c_int,
                         conp: *mut ErlConnect, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_unpublish(ec: *mut ei_cnode) -> ::libc::c_int;
    pub fn ei_unpublish_tmo(alive: *const ::libc::c_char, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_thisnodename(ec: *const ei_cnode) -> *const ::libc::c_char;
    pub fn ei_thishostname(ec: *const ei_cnode) -> *const ::libc::c_char;
    pub fn ei_thisalivename(ec: *const ei_cnode) -> *const ::libc::c_char;
    pub fn ei_self(ec: *mut ei_cnode) -> *mut erlang_pid;
    pub fn ei_set_compat_rel(rel: ::libc::c_uint) -> ();
    pub fn ei_set_tracelevel(arg1: ::libc::c_int) -> ();
    pub fn ei_get_tracelevel() -> ::libc::c_int;
    pub fn ei_gethostbyname(name: *const ::libc::c_char)
     -> *mut ::libc::hostent;
    pub fn ei_gethostbyaddr(addr: *const ::libc::c_char, len: ::libc::c_int,
                            _type: ::libc::c_int) -> *mut ::libc::hostent;
    pub fn ei_gethostbyname_r(name: *const ::libc::c_char,
                              hostp: *mut ::libc::hostent,
                              buffer: *mut ::libc::c_char,
                              buflen: ::libc::c_int,
                              h_errnop: *mut ::libc::c_int)
     -> *mut ::libc::hostent;
    pub fn ei_gethostbyaddr_r(addr: *const ::libc::c_char,
                              length: ::libc::c_int, _type: ::libc::c_int,
                              hostp: *mut ::libc::hostent,
                              buffer: *mut ::libc::c_char,
                              buflen: ::libc::c_int,
                              h_errnop: *mut ::libc::c_int)
     -> *mut ::libc::hostent;
    pub fn ei_encode_version(buf: *mut ::libc::c_char,
                             index: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_x_encode_version(x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_encode_long(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                          p: ::libc::c_long) -> ::libc::c_int;
    pub fn ei_x_encode_long(x: *mut ei_x_buff, n: ::libc::c_long)
     -> ::libc::c_int;
    pub fn ei_encode_ulong(buf: *mut ::libc::c_char,
                           index: *mut ::libc::c_int, p: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn ei_x_encode_ulong(x: *mut ei_x_buff, n: ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn ei_encode_double(buf: *mut ::libc::c_char,
                            index: *mut ::libc::c_int, p: ::libc::c_double)
     -> ::libc::c_int;
    pub fn ei_x_encode_double(x: *mut ei_x_buff, dbl: ::libc::c_double)
     -> ::libc::c_int;
    pub fn ei_encode_boolean(buf: *mut ::libc::c_char,
                             index: *mut ::libc::c_int, p: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_x_encode_boolean(x: *mut ei_x_buff, p: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_encode_char(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                          p: ::libc::c_char) -> ::libc::c_int;
    pub fn ei_x_encode_char(x: *mut ei_x_buff, p: ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_encode_string(buf: *mut ::libc::c_char,
                            index: *mut ::libc::c_int,
                            p: *const ::libc::c_char) -> ::libc::c_int;
    pub fn ei_encode_string_len(buf: *mut ::libc::c_char,
                                index: *mut ::libc::c_int,
                                p: *const ::libc::c_char, len: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_x_encode_string(x: *mut ei_x_buff, s: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_x_encode_string_len(x: *mut ei_x_buff, s: *const ::libc::c_char,
                                  len: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_encode_atom(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                          p: *const ::libc::c_char) -> ::libc::c_int;
    pub fn ei_encode_atom_as(buf: *mut ::libc::c_char,
                             index: *mut ::libc::c_int,
                             p: *const ::libc::c_char,
                             from: erlang_char_encoding,
                             to: erlang_char_encoding) -> ::libc::c_int;
    pub fn ei_encode_atom_len(buf: *mut ::libc::c_char,
                              index: *mut ::libc::c_int,
                              p: *const ::libc::c_char, len: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_encode_atom_len_as(buf: *mut ::libc::c_char,
                                 index: *mut ::libc::c_int,
                                 p: *const ::libc::c_char, len: ::libc::c_int,
                                 from: erlang_char_encoding,
                                 to: erlang_char_encoding) -> ::libc::c_int;
    pub fn ei_x_encode_atom(x: *mut ei_x_buff, s: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_x_encode_atom_as(x: *mut ei_x_buff, s: *const ::libc::c_char,
                               from: erlang_char_encoding,
                               to: erlang_char_encoding) -> ::libc::c_int;
    pub fn ei_x_encode_atom_len(x: *mut ei_x_buff, s: *const ::libc::c_char,
                                len: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_x_encode_atom_len_as(x: *mut ei_x_buff,
                                   s: *const ::libc::c_char,
                                   len: ::libc::c_int,
                                   from: erlang_char_encoding,
                                   to: erlang_char_encoding) -> ::libc::c_int;
    pub fn ei_encode_binary(buf: *mut ::libc::c_char,
                            index: *mut ::libc::c_int,
                            p: *const ::libc::c_void, len: ::libc::c_long)
     -> ::libc::c_int;
    pub fn ei_x_encode_binary(x: *mut ei_x_buff, s: *const ::libc::c_void,
                              len: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_encode_pid(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                         p: *const erlang_pid) -> ::libc::c_int;
    pub fn ei_x_encode_pid(x: *mut ei_x_buff, pid: *const erlang_pid)
     -> ::libc::c_int;
    pub fn ei_encode_fun(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                         p: *const erlang_fun) -> ::libc::c_int;
    pub fn ei_x_encode_fun(x: *mut ei_x_buff, fun: *const erlang_fun)
     -> ::libc::c_int;
    pub fn ei_encode_port(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                          p: *const erlang_port) -> ::libc::c_int;
    pub fn ei_x_encode_port(x: *mut ei_x_buff, p: *const erlang_port)
     -> ::libc::c_int;
    pub fn ei_encode_ref(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                         p: *const erlang_ref) -> ::libc::c_int;
    pub fn ei_x_encode_ref(x: *mut ei_x_buff, p: *const erlang_ref)
     -> ::libc::c_int;
    pub fn ei_encode_term(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                          t: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn ei_x_encode_term(x: *mut ei_x_buff, t: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ei_encode_trace(buf: *mut ::libc::c_char,
                           index: *mut ::libc::c_int, p: *const erlang_trace)
     -> ::libc::c_int;
    pub fn ei_x_encode_trace(x: *mut ei_x_buff, p: *const erlang_trace)
     -> ::libc::c_int;
    pub fn ei_encode_tuple_header(buf: *mut ::libc::c_char,
                                  index: *mut ::libc::c_int,
                                  arity: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_x_encode_tuple_header(x: *mut ei_x_buff, n: ::libc::c_long)
     -> ::libc::c_int;
    pub fn ei_encode_list_header(buf: *mut ::libc::c_char,
                                 index: *mut ::libc::c_int,
                                 arity: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_x_encode_list_header(x: *mut ei_x_buff, n: ::libc::c_long)
     -> ::libc::c_int;
    pub fn ei_x_encode_empty_list(x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_encode_map_header(buf: *mut ::libc::c_char,
                                index: *mut ::libc::c_int,
                                arity: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_x_encode_map_header(x: *mut ei_x_buff, n: ::libc::c_long)
     -> ::libc::c_int;
    pub fn ei_get_type(buf: *const ::libc::c_char,
                       index: *const ::libc::c_int, _type: *mut ::libc::c_int,
                       size: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_get_type_internal(buf: *const ::libc::c_char,
                                index: *const ::libc::c_int,
                                _type: *mut ::libc::c_int,
                                size: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_decode_version(buf: *const ::libc::c_char,
                             index: *mut ::libc::c_int,
                             version: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_decode_long(buf: *const ::libc::c_char,
                          index: *mut ::libc::c_int, p: *mut ::libc::c_long)
     -> ::libc::c_int;
    pub fn ei_decode_ulong(buf: *const ::libc::c_char,
                           index: *mut ::libc::c_int, p: *mut ::libc::c_ulong)
     -> ::libc::c_int;
    pub fn ei_decode_double(buf: *const ::libc::c_char,
                            index: *mut ::libc::c_int,
                            p: *mut ::libc::c_double) -> ::libc::c_int;
    pub fn ei_decode_boolean(buf: *const ::libc::c_char,
                             index: *mut ::libc::c_int, p: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_decode_char(buf: *const ::libc::c_char,
                          index: *mut ::libc::c_int, p: *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_decode_string(buf: *const ::libc::c_char,
                            index: *mut ::libc::c_int, p: *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_decode_atom(buf: *const ::libc::c_char,
                          index: *mut ::libc::c_int, p: *mut ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_decode_atom_as(buf: *const ::libc::c_char,
                             index: *mut ::libc::c_int,
                             p: *mut ::libc::c_char, destlen: ::libc::c_int,
                             want: erlang_char_encoding,
                             was: *mut erlang_char_encoding,
                             result: *mut erlang_char_encoding)
     -> ::libc::c_int;
    pub fn ei_decode_binary(buf: *const ::libc::c_char,
                            index: *mut ::libc::c_int, p: *mut ::libc::c_void,
                            len: *mut ::libc::c_long) -> ::libc::c_int;
    pub fn ei_decode_fun(buf: *const ::libc::c_char,
                         index: *mut ::libc::c_int, p: *mut erlang_fun)
     -> ::libc::c_int;
    pub fn free_fun(f: *mut erlang_fun) -> ();
    pub fn ei_decode_pid(buf: *const ::libc::c_char,
                         index: *mut ::libc::c_int, p: *mut erlang_pid)
     -> ::libc::c_int;
    pub fn ei_decode_port(buf: *const ::libc::c_char,
                          index: *mut ::libc::c_int, p: *mut erlang_port)
     -> ::libc::c_int;
    pub fn ei_decode_ref(buf: *const ::libc::c_char,
                         index: *mut ::libc::c_int, p: *mut erlang_ref)
     -> ::libc::c_int;
    pub fn ei_decode_term(buf: *const ::libc::c_char,
                          index: *mut ::libc::c_int, t: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn ei_decode_trace(buf: *const ::libc::c_char,
                           index: *mut ::libc::c_int, p: *mut erlang_trace)
     -> ::libc::c_int;
    pub fn ei_decode_tuple_header(buf: *const ::libc::c_char,
                                  index: *mut ::libc::c_int,
                                  arity: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_decode_list_header(buf: *const ::libc::c_char,
                                 index: *mut ::libc::c_int,
                                 arity: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_decode_map_header(buf: *const ::libc::c_char,
                                index: *mut ::libc::c_int,
                                arity: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_decode_ei_term(buf: *const ::libc::c_char,
                             index: *mut ::libc::c_int, term: *mut ei_term)
     -> ::libc::c_int;
    pub fn ei_print_term(fp: *mut ::libc::FILE, buf: *const ::libc::c_char,
                         index: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_s_print_term(s: *mut *mut ::libc::c_char,
                           buf: *const ::libc::c_char,
                           index: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_x_format(x: *mut ei_x_buff, fmt: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn ei_x_format_wo_ver(x: *mut ei_x_buff,
                              fmt: *const ::libc::c_char, ...)
     -> ::libc::c_int;
    pub fn ei_x_new(x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_x_new_with_version(x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_x_free(x: *mut ei_x_buff) -> ::libc::c_int;
    pub fn ei_x_append(x: *mut ei_x_buff, x2: *const ei_x_buff)
     -> ::libc::c_int;
    pub fn ei_x_append_buf(x: *mut ei_x_buff, buf: *const ::libc::c_char,
                           len: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_skip_term(buf: *const ::libc::c_char, index: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_reg_open(size: ::libc::c_int) -> *mut ei_reg;
    pub fn ei_reg_resize(oldreg: *mut ei_reg, newsize: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_reg_close(reg: *mut ei_reg) -> ::libc::c_int;
    pub fn ei_reg_setival(reg: *mut ei_reg, key: *const ::libc::c_char,
                          i: ::libc::c_long) -> ::libc::c_int;
    pub fn ei_reg_setfval(reg: *mut ei_reg, key: *const ::libc::c_char,
                          f: ::libc::c_double) -> ::libc::c_int;
    pub fn ei_reg_setsval(reg: *mut ei_reg, key: *const ::libc::c_char,
                          s: *const ::libc::c_char) -> ::libc::c_int;
    pub fn ei_reg_setpval(reg: *mut ei_reg, key: *const ::libc::c_char,
                          p: *const ::libc::c_void, size: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_reg_setval(reg: *mut ei_reg, key: *const ::libc::c_char,
                         flags: ::libc::c_int, ...) -> ::libc::c_int;
    pub fn ei_reg_getival(reg: *mut ei_reg, key: *const ::libc::c_char)
     -> ::libc::c_long;
    pub fn ei_reg_getfval(reg: *mut ei_reg, key: *const ::libc::c_char)
     -> ::libc::c_double;
    pub fn ei_reg_getsval(reg: *mut ei_reg, key: *const ::libc::c_char)
     -> *const ::libc::c_char;
    pub fn ei_reg_getpval(reg: *mut ei_reg, key: *const ::libc::c_char,
                          size: *mut ::libc::c_int) -> *const ::libc::c_void;
    pub fn ei_reg_getval(reg: *mut ei_reg, key: *const ::libc::c_char,
                         flags: ::libc::c_int, ...) -> ::libc::c_int;
    pub fn ei_reg_markdirty(reg: *mut ei_reg, key: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_reg_delete(reg: *mut ei_reg, key: *const ::libc::c_char)
     -> ::libc::c_int;
    pub fn ei_reg_stat(reg: *mut ei_reg, key: *const ::libc::c_char,
                       obuf: *mut Struct_ei_reg_stat) -> ::libc::c_int;
    pub fn ei_reg_tabstat(reg: *mut ei_reg, obuf: *mut Struct_ei_reg_tabstat)
     -> ::libc::c_int;
    pub fn ei_reg_dump(fd: ::libc::c_int, reg: *mut ei_reg,
                       mntab: *const ::libc::c_char, flags: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_reg_restore(fd: ::libc::c_int, reg: *mut ei_reg,
                          mntab: *const ::libc::c_char) -> ::libc::c_int;
    pub fn ei_reg_purge(reg: *mut ei_reg) -> ::libc::c_int;
    pub fn ei_decode_longlong(buf: *const ::libc::c_char,
                              index: *mut ::libc::c_int,
                              p: *mut ::libc::c_longlong) -> ::libc::c_int;
    pub fn ei_decode_ulonglong(buf: *const ::libc::c_char,
                               index: *mut ::libc::c_int,
                               p: *mut ::libc::c_ulonglong) -> ::libc::c_int;
    pub fn ei_encode_longlong(buf: *mut ::libc::c_char,
                              index: *mut ::libc::c_int,
                              p: ::libc::c_longlong) -> ::libc::c_int;
    pub fn ei_encode_ulonglong(buf: *mut ::libc::c_char,
                               index: *mut ::libc::c_int,
                               p: ::libc::c_ulonglong) -> ::libc::c_int;
    pub fn ei_x_encode_longlong(x: *mut ei_x_buff, n: ::libc::c_longlong)
     -> ::libc::c_int;
    pub fn ei_x_encode_ulonglong(x: *mut ei_x_buff, n: ::libc::c_ulonglong)
     -> ::libc::c_int;
    pub fn ei_decode_intlist(buf: *const ::libc::c_char,
                             index: *mut ::libc::c_int,
                             a: *mut ::libc::c_long,
                             count: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_receive_encoded(fd: ::libc::c_int,
                              bufp: *mut *mut ::libc::c_char,
                              bufsz: *mut ::libc::c_int, to: *mut erlang_msg,
                              msglen: *mut ::libc::c_int) -> ::libc::c_int;
    pub fn ei_receive_encoded_tmo(fd: ::libc::c_int,
                                  bufp: *mut *mut ::libc::c_char,
                                  bufsz: *mut ::libc::c_int,
                                  to: *mut erlang_msg,
                                  msglen: *mut ::libc::c_int,
                                  ms: ::libc::c_uint) -> ::libc::c_int;
    pub fn ei_send_encoded(fd: ::libc::c_int, to: *const erlang_pid,
                           msg: *mut ::libc::c_char, msglen: ::libc::c_int)
     -> ::libc::c_int;
    pub fn ei_send_encoded_tmo(fd: ::libc::c_int, to: *const erlang_pid,
                               msg: *mut ::libc::c_char,
                               msglen: ::libc::c_int, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_send_reg_encoded(fd: ::libc::c_int, from: *const erlang_pid,
                               to: *const ::libc::c_char,
                               msg: *mut ::libc::c_char,
                               msglen: ::libc::c_int) -> ::libc::c_int;
    pub fn ei_send_reg_encoded_tmo(fd: ::libc::c_int, from: *const erlang_pid,
                                   to: *const ::libc::c_char,
                                   msg: *mut ::libc::c_char,
                                   msglen: ::libc::c_int, ms: ::libc::c_uint)
     -> ::libc::c_int;
    pub fn ei_encode_big(buf: *mut ::libc::c_char, index: *mut ::libc::c_int,
                         big: *mut erlang_big) -> ::libc::c_int;
    pub fn ei_x_encode_big(x: *mut ei_x_buff, big: *mut erlang_big)
     -> ::libc::c_int;
    pub fn ei_decode_big(buf: *const ::libc::c_char,
                         index: *mut ::libc::c_int, p: *mut erlang_big)
     -> ::libc::c_int;
    pub fn ei_big_comp(x: *mut erlang_big, y: *mut erlang_big)
     -> ::libc::c_int;
    pub fn ei_big_to_double(b: *mut erlang_big, resp: *mut ::libc::c_double)
     -> ::libc::c_int;
    pub fn ei_small_to_big(s: ::libc::c_int, b: *mut erlang_big)
     -> ::libc::c_int;
    pub fn ei_alloc_big(arity: ::libc::c_uint) -> *mut erlang_big;
    pub fn ei_free_big(b: *mut erlang_big) -> ();
}
