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

Change hostname for your hostname (appears on the erlang shell)

::

    erl -sname e1 -setcookie secretcookie
    cargo run e1@hostname secretcookie

In the erlang sell send something::

    (e1@hostname)1> {any, c1@hostname} ! self().

On the rust side you should see something like::

    got a send! Pid(e1@hostname, 39, 0, 1)

If you send something like {pid(), any()} it will send you back any(), let's try it.

Start the erlang node as explained above::

    erl -sname e1 -setcookie secretcookie

Then start the rust node::

    cargo run e1@hostname secretcookie
         Running `target/debug/erl_interface e1@ganesha secretcookie`
    Connected to e1@ganesha

From the erlang shell send the message with the format explained above::

    (e1@ganesha)33> {any, c1@ganesha} ! {self(), [32, 1.4, foo, {<<"asd">>, "lala"}]}.
    {<0.39.0>,[32,1.4,foo,{<<"asd">>,"lala"}]}

On the rust node you should see::

    got echo! sending List(size: 4, items: (Int(32), Float(1.4), Atom(foo), Tuple(size: 2, items: (Binary(size: 3, items: (97, 115, 100)), List(size: 4, items: (Int(108), Int(97), Int(108), Int(97)))))))

And flushing received messages on the erlang node you should see::

    (e1@ganesha)34> flush().
    Shell got [32,1.4,foo,{<<"asd">>,"lala"}]
    ok

