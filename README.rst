erl_interface.rs
================

Attempt at a rust wrapper for `erl_interface <http://www.erlang.org/doc/tutorial/cnode.html>`_ C API.

Disclaimer: I have yet to learn rust, this is me writing C in rust syntax until it compiles.

Build
-----

::

    cargo build

Run
---

::

    erl -sname e1 -setcookie secretcookie
    cargo run
