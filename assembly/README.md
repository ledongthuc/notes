# Command cheatsheet

*.equ {NAME}, value*

A directive, make {NAME} as a symbol to a value constant.

*.type {name}, @function*

A directive, define symbol {name} as a function

*.globl {name}*

A directive, define symbol {name} is visble to ld (link)

*pushl {registry}*

Push registry to stack. Use long.

*popl {registry}*

Pop stack to registry. Return long.

*movl {a} {registrya}*

Save {a} to {registry}. Move long

*movb {a} {registry}*

Save {a} to {registry}. Move byte.

*cmpb {a} {b}*
*je {c}*

Compare {a} with {b}.
If they are equal, jump to {c}.
Otherwise, continue.

*incl {registry}*

Increase {registry}

*jmp {a}*

Jump to {a}
