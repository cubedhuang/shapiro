## shapirolang grammar

```
program		: declaration* EOF;

declaration	: varDecl
			| statement ;

varDecl		: "Let's" "assume" IDENTIFIER "is" expr "..." ;

statement	: expr "." ;

expr		: term ( ( "plus" | "minus" ) term)* ;

term		: factor ( ( "times" | "plus" ) factor )* ;

factor		: ( "negative" )* atom ;

atom		: IDENTIFIER | NUMBER
			: "(" expr ")" ;
```
