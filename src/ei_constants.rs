pub const EIO: ::libc::c_int = 5;
pub const EMSGSIZE: ::libc::c_int =        EIO;
pub const ETIMEDOUT: ::libc::c_int =       EIO;
pub const EHOSTUNREACH: ::libc::c_int =    EIO;
pub const ERL_ERROR: ::libc::c_int = -1;           /* Error of some kind */
pub const ERL_NO_DAEMON: ::libc::c_int = -2;       /* No contact with EPMD */
pub const ERL_NO_PORT: ::libc::c_int = -3;         /* No port received from EPMD */   
pub const ERL_CONNECT_FAIL: ::libc::c_int = -4;    /* Connect to Erlang Node failed */
pub const ERL_TIMEOUT: ::libc::c_int = -5;         /* A timeout has expired */
pub const ERL_NO_REMOTE: ::libc::c_int = -6;       /* Cannot execute rsh */
pub const ERL_TICK: ::libc::c_int = 0;
pub const ERL_MSG: ::libc::c_int = 1;
pub const ERL_NO_TIMEOUT: ::libc::c_int = -1;
pub const ERL_LINK: ::libc::c_int =           1;
pub const ERL_SEND: ::libc::c_int =           2;
pub const ERL_EXIT: ::libc::c_int =           3;
pub const ERL_UNLINK: ::libc::c_int =         4;
pub const ERL_NODE_LINK: ::libc::c_int =      5;
pub const ERL_REG_SEND: ::libc::c_int =       6;
pub const ERL_GROUP_LEADER: ::libc::c_int =   7;
pub const ERL_EXIT2: ::libc::c_int =          8;
pub const ERL_PASS_THROUGH: ::libc::c_char =  'p' as ::libc::c_char;
pub const ERL_SEND_TT: ::libc::c_int =        12;
pub const ERL_EXIT_TT: ::libc::c_int =        13;
pub const ERL_REG_SEND_TT: ::libc::c_int =    16;
pub const ERL_EXIT2_TT: ::libc::c_int =       18;
pub const ERL_MONITOR_P: ::libc::c_int =      19;
pub const ERL_DEMONITOR_P: ::libc::c_int =    20;
pub const ERL_MONITOR_P_EXIT: ::libc::c_int = 21;
pub const ERL_SMALL_INTEGER_EXT: ::libc::c_char = 'a' as ::libc::c_char;
pub const ERL_INTEGER_EXT: ::libc::c_char =       'b' as ::libc::c_char;
pub const ERL_FLOAT_EXT: ::libc::c_char =         'c' as ::libc::c_char;
pub const NEW_FLOAT_EXT: ::libc::c_char =         'F' as ::libc::c_char;
pub const ERL_ATOM_EXT: ::libc::c_char =          'd' as ::libc::c_char;
pub const ERL_SMALL_ATOM_EXT: ::libc::c_char =    's' as ::libc::c_char;
pub const ERL_ATOM_UTF8_EXT: ::libc::c_char =     'v' as ::libc::c_char;
pub const ERL_SMALL_ATOM_UTF8_EXT: ::libc::c_char = 'w' as ::libc::c_char;
pub const ERL_REFERENCE_EXT: ::libc::c_char =     'e' as ::libc::c_char;
pub const ERL_NEW_REFERENCE_EXT: ::libc::c_char = 'r' as ::libc::c_char;
pub const ERL_PORT_EXT: ::libc::c_char =          'f' as ::libc::c_char;
pub const ERL_PID_EXT: ::libc::c_char =           'g' as ::libc::c_char;
pub const ERL_SMALL_TUPLE_EXT: ::libc::c_char =   'h' as ::libc::c_char;
pub const ERL_LARGE_TUPLE_EXT: ::libc::c_char =   'i' as ::libc::c_char;
pub const ERL_NIL_EXT: ::libc::c_char =           'j' as ::libc::c_char;
pub const ERL_STRING_EXT: ::libc::c_char =        'k' as ::libc::c_char;
pub const ERL_LIST_EXT: ::libc::c_char =          'l' as ::libc::c_char;
pub const ERL_BINARY_EXT: ::libc::c_char =        'm' as ::libc::c_char;
pub const ERL_SMALL_BIG_EXT: ::libc::c_char =     'n' as ::libc::c_char;
pub const ERL_LARGE_BIG_EXT: ::libc::c_char =     'o' as ::libc::c_char;
pub const ERL_NEW_FUN_EXT	: ::libc::c_char =      'p' as ::libc::c_char;
pub const ERL_MAP_EXT: ::libc::c_char =           't' as ::libc::c_char;
pub const ERL_FUN_EXT	: ::libc::c_char =      'u' as ::libc::c_char;
pub const ERL_NEW_CACHE: ::libc::c_char =         'N' as ::libc::c_char; /* c nodes don't know these two */
pub const ERL_CACHED_ATOM: ::libc::c_char =       'C' as ::libc::c_char;
pub const EI_MAXHOSTNAMELEN: ::libc::c_int = 64;
pub const EI_MAXALIVELEN: ::libc::c_int = 63;
pub const EI_MAX_COOKIE_SIZE: ::libc::c_int = 512;
pub const MAXNODELEN: ::libc::c_int = EI_MAXALIVELEN+1+EI_MAXHOSTNAMELEN;
pub const HOST_NOT_FOUND: ::libc::c_int =	1; /* Authoritative Answer Host not found */
pub const TRY_AGAIN: ::libc::c_int =	2; /* Non-Authoritive Host not found, or SERVERFAIL */
pub const NO_RECOVERY: ::libc::c_int =	3; /* Non recoverable errors, FORMERR, REFUSED, NOTIMP */
pub const NO_DATA: ::libc::c_int =		4; /* Valid name, no data record of requested type */
pub const NO_ADDRESS: ::libc::c_int =	NO_DATA;		/* no address, look for MX record */
pub const EI_SMALLKEY: ::libc::c_int = 32;
pub const EI_DIRTY: ::libc::c_int = 0x01; /* dirty bit (object value differs from backup) */
pub const EI_DELET: ::libc::c_int = 0x02; /* object is deleted */
pub const EI_INT: ::libc::c_int = 0x10; /* object is an integer */
pub const EI_FLT: ::libc::c_int = 0x20; /* object is a float */
pub const EI_STR: ::libc::c_int = 0x40; /* object is a string */
pub const EI_BIN: ::libc::c_int = 0x80; /* object is a binary, i.e. pointer to arbitrary type */
pub const EI_FORCE: ::libc::c_int = 0x1; /* dump all records (not just dirty ones) */
pub const EI_NOPURGE: ::libc::c_int = 0x2; /* don't purge deleted records */