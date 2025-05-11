# Modular interpreter

Proof of concept modular tree-walk interpreter in rust.

There are some things that could be improved in more serious implementation:

1. Get rid of all the cloning. Replace `Box`es with `Rc`s and owned values
(especially in core traits) with either shared references or `Rc`s
2. Helper functions for more common operations (`Ok(L::wrap(_))` and
`L::try_into_val(_.parse())``)
3. Allow for checking if error is of the type of fragment we are working in.
This would allow implementing exceptions
4. Helper traits to allow for using more functions with `.` syntax?

This interpreter is just that - an interpreter. It does not contain parser nor
a repl. Seeing that the core approach works I'll likely move on to something
else now.
