Macros vs Functions:
- Functions must declare the amount of parameters while macros have a variable number of parameters.
- Functions are called at runtime while macros are expanded before the program finishes compiling.
- Downside is increased complexity because you are writing code that writes other code.

2 Forms of Macros:
- Declarative
	- Most widely used form of macros.
	- Allow you to write something similar to a match expression.
- Procedural
	- Like functions in the sense that they take code as input, operate on that code, and produce code as output.
	- Declarative replace code with other code.
