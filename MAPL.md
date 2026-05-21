# What is MAPL?

MAPL (pronounced like maple /ˈmeɪ.pəl/) short for Maybe A Programming Language is the best programming language (or rather specification for a language).

# MAPL Specifications
## Variables

Variables are reasignable & mutable values that can be named with any string of alphanumeric characters (numbers cannot be used standalone and must be accompanied by a letter). Variables in MAPL are declared with the `var` keyword followed by a space, the variable name, a colon (`:`), then another space, the variable's data type, a space, followed by an equals `=`, another space, the variable's value, and finally a semi-colon. If done correctly the line should look something like this:

```
var x: string = "This is a string";
```

The second type of variable is a constant, constants are **NOT** reasignable and are immutable. Constants are defined with the `const` keyword and follow the same structure as a regular variable. The declaration of a constant should look similar to this:

``` 
const y: int = 10;
```

## Line termination

Much like all C-Style languages, MAPL requires lines to be terminated with a semi-colon `;` this allows for long, complex lines to span multiple lines & to have multiple lines of logic within a singular line. 

## Data Types

**Booleans (Bools)** can be either `true` or `false`
**Integers** non-decimal number values e.g. `1`, `13`, `99`
**Floats** decimal number values e.g. `1.5`, `12.233`, `99.999`
**Strings** assortment of any unicode characters wrapped with quotations `"`
**Arrays** a set of values wrapped with square braces `[]`
**Hashmap** or linked list, is a set of values with a counterpart that can be looked up wrapped with curly braces `{}`
**ENUM** set of possible values wrapped with curly braces `{}`

```
var isDead: bool = false; // boolean
var health: int = 10; // integer
var damage: float = 5.5; // float
var playerName: str = "John"; // string
var playerInventory: arr = ["axe", "sword", "iron"]; // array
var itemDamages: hmap = { // hashmap
	"sword": 6,
	"axe": 5
};
var playerState: enum = { // enumerator
	ALIVE,
	DEAD
};
```

*Note:
The type can also be set to "any" which makes the interpreter infer the type based on its value at declaration*

## Loops & Statements

### If statements

MAPL includes the standard if statement in the format

```
if (args) { 
	...
}

e.g.

if (i == 10) {
	print("i is equal to 10");
}
```

if statements can be used to check equality using `==` which checks for a direct comparison & ~= for a loose equality check (be careful with this it can cause problems). Double ampersands `&&` can be used to add multiple checks to an if statement e.g.

```
if (i <= 10 && i != 7) {
	print("i is less than 10 but isn't 7");
}
```

Double poles `||` can be used to check for multiple separate possibilities that you want to give the same outcome e.g.

```
if (i == 10 || i == 11) {
	print("i is equal to ten or eleven");
}
```

the `else` keyword can be used to handle if the statement in the args of the if statement return false like so:

```
var x: int = 13;

if (i < 10) {
	return;
} else {
	print("x is not less than 10");
}
```

**How does loose equality `~=` work?** Loose equality works virtually the same as JavaScript's `==` which is also called loose equality.

### For loops

MAPL also features a normal for loop although more similar to the Python for loop than a C-style one, in the format of

```
for (i :: [number]) {
	...
}
```

example:

```
for (i :: 10) {
	print(i);
}
```

it uses the 'in' operator `::` to determine if the value is apart of the comparative value.

### While loops

Much like if statements while loops take the standard approach of

```
while (args) {
	...
}
```

I'm not going to elaborate much on while loops because you probably already understand them.

## Definitions

### Classes

Classes are defined with the `class` keyword followed by the name and curly braces. Classes are your standard class, they can hold methods (functions), and variables/constants.

```
class ____ {
	...
}
```

### Functions

In MAPL, functions are defined with the `func` keyword, they can be parsed arguments, and contain logic.

```
func ____(args) : type {
	...
}

e.g. 

func add(a: float, b: float) : float {
	var sum: float = null;
	sum = a + b;
	
	return sum;
}
```

### Structures (Structs)

Structs are used to store several variables of different types, they are defined with the `struct` keyword followed by a name, an equals sign `=`, and curly braces.

```
struct ____ {
	...
}
```

## Calling

Calling can be done on functions or classes. Calling allows code to be reused and run by other parts of your code.

### calling functions

```
main(); // calls the function called "main"
```

### calling methods on classes

```
Player.damage(); // calls the method "damage" on the "Player" class
```

## Parsing datatypes

MAPL allows the ability to alter the data types of variables (not constants) using the corresponding function. The available datatype parsing functions are:
```
str() // parses to a string
int() // parses to integer
float() // parses to float
bool() // parses to a boolean (only works on strings containing "true" or "false", or floats/ints of 0 & 1)
enum() // parses to an enum (only works on arrays)
arr() // parses to an array
```

example:

```
print(float(9)) // 9.0
```

## arithmetic & operators

MAPL uses the standard `+`, `-`, `*`, `^`, `%` MAPL also operates using BIDMAS. the division operator in MAPL is the `/` for division of floats *or* integers while keeping them their datatype.

### operators

`+` adds 2 values together (also joins together strings)
`++` increases the value of an integer or float by 1
`+=` adds 2 values together and sets the first value to the output

`-` subtracts 1 numeric value from another
`-=` subtracts 1 numeric value from another and sets the first value to the output

`*` multiplies 2 numeric values together
`*=` multiplies 2 numeric values together and sets the first value to the output

`!` can be used for "not" like `!=` or just `![variable]` which checks for a boolean to be true.

etc. *just your standard arithmetic*

## Exporting & Importing

To export a function, class, struct, constant, or variable to make it usable when other MAPL scripts import the file you simply add @export to the front of it e.g. 

``` 
@export class Player {
	...
}
```

To import a function, class, struct, constant, or variable from another file you simply use @import as so:

```
@import { Player } from "other-script.mapl";
```

*note: if you do not set a path for the import it will use the path to your MAPL packages*

Packages are imported the same way although you can specify a package name rather than a file path.

## Class initialisation

Classes must contain a function named "init" which is used when instantiating a class to parse data like so:

```

class Player {
	func init(name: string) {
		self.name = name;
	}
}

const playerA: Player = Player("player 1");

```

classes use the `self` identifier locally so you'd reference a variable as `self.[var name]` when in the class.


## Comments

MAPL uses standard C-Style code comments where `//` is a single line comment and `/* */` is used for block comments

## String interpolation

Whenever you may want to put a variable in a string you use `${...}` like so:

```
var health: float = 10.0;
var name: string = "John";

print("The player ${name} is at ${health} health!") // The player John is at 10.0 health!
```

## Built-in functions

*note: you do not need to import these!*

### sys
sys is a standard built-in library that allows you to run console commands, it has several built in functions like:

```
sys.exit(); // exits the program
sys.execute(...); // executes a console command 
sys.info(); // returns information about the current OS & device
```

### io
IO short for Input-Output allows access to files through `IO.readFile()`, `IO.writeFile()`, `IO.deleteFile()`, and `IO.createFile()`

### math
The math library is used for advanced mathematics includes functions like `Math.sqrt()`, `Math.abs()`, `Math.sin()`, and `Math.cos()` with several others used for more advanced calculations than the regular arithmetic allows for.

## Missing values
MAPL much like other languages can have values be nothing, in the case of MAPL we have 3 no value keywords: `null` a value you can assign, `undefined` an error return for when an argument in a function isn't populated, and `NaN` short for Not A Number exclusively used for divide by 0.

