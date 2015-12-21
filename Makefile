
bindings:
	bindgen -l erl_interface -match erl_interface.h -o src/erl_interface.rs /usr/lib/erlang/usr/include/erl_interface.h
	bindgen -l ei -match ei.h -o src/ei.rs /usr/lib/erlang/usr/include/ei.h

