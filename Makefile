
bindings:
	bindgen -l erl_interface -match erl_interface.h -o src/erl_interface.rs /usr/lib/erlang/usr/include/erl_interface.h

