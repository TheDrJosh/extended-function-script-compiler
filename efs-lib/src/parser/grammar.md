# EFS Grammar Outline

program = (function_dec | const_dec | use_file | struct_def)*

const_dec = `Keyword(Const)` ~ `Identifier` ~ `TypeClarify` ~ `Type` ~ `Assign` ~ (math | any)

use_file = `Keyword(UseFile)` ~ Path ~ `NEWLINE`

struct_def = 

## Function

function_dec = (`Keyword(Static)` | atribuibute_dec) ~ function_def

atribuibute_dec = `Attribute` ~ `LBracket` ~ (atribuibute ~ (`Comma` ~ atribuibute)*) ~ `RBracket` ~ `NEWLINE`

atribuibute = `Identifier` ~ (`LParen` ~ (`Identifier` ~ (`Comma` ~ `Identifier` )*) ~ `RParen`)?

function_def = `Keyword(Function)` ~ `Identifier` ~ `LParen` ~ function_params ~ `RParen` ~ function_return ~  code_block

function_params = `Identifier` ~ (`Comma` ~ `Identifier`)*

function_return = (`FunctionReturn` ~ `Type`)?

## Expression

code_block = `LBraces` ~ code_line* ~ `RBraces`

statement = (for | static_for | while | (expression ~ `EndLine`))

expression = (var_dec | assign | any)

for = `Keyword(for)` ~ `LParen` ~ statement ~ statement ~ statement ~ `RParen` ~ code_block

for_list = `Keyword(for)` ~ `LParen` ~ `Identifier` ~ `Keyword(in)` ~ `Identifier` ~ `RParen` ~ code_block

static_for = `Keyword(static)` ~ `Keyword(for)` ~ `LParen` ~ `Identifier` ~ `Comma` ~ `Integer` ~ `Comma` ~ `Integer` ~ (`Comma` ~ `Integer`)? ~ `RParen` ~ code_block

while = `Keyword(while)` ~ `LParen` ~ any `RParen` ~ code_block

assign = `Identifier` ~ `Assign` ~ any

function = `Identifier` ~ `LParen` ~ expression ~ (`Comma` ~ expression)* ~ `RParen`

var_dec = `Keyword(VarDeceleration)` ~ `Identifier` ~ (`TypeClarify` ~ `Type`)? ~ `Assign` ~ expression ~ `EndLine`

if = `Keyword(If)` ~ `LParen` ~ 


### Math

math = term ~ (order_operation_level_1 ~ term)*

order_operation_level_1 = `Multi` | `Div`

term = expr ~ (order_operation_level_2 ~ expr)*

order_operation_level_2 = `Plus` | `Minus`

expr = function | `Integer` | `Float` | (`LParen` ~ math ~ `RParen`)

## Type

any_type = `Integer` | `Float` | `String` | struct | math

string = `Quote` ~ `ANY`* ~ `Quote`

bool = `Keyword(true)` | `Keyword(false)`

struct = !todo