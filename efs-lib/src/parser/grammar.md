# EFS Grammar Outline

program = (function_dec | const_dec | use_file | struct_def)*

const_dec = `Keyword(Const)` ~ `Identifier` ~ `TypeClarify` ~ `Type` ~ `Assign` ~ (math | any)

use_file = `Keyword(UseFile)` ~ Path ~ `NEWLINE`

## Struct

struct_def = `Keyword(Struct)` ~ `Identifier` ~ `LBraces` ~ struct_body ~ `RBraces`

struct_def_body = struct_pair ~ (`Comma` ~ struct_pair)*

struct_def_pair = `Identifier` ~ `TypeClarify` ~ `Type`

## Function

function_dec = (`Keyword(Static)` | atribuibute_dec) ~ function_def

atribuibute_dec = `Attribute` ~ `LBracket` ~ (atribuibute ~ (`Comma` ~ atribuibute)*) ~ `RBracket` ~ `NEWLINE`

atribuibute = `Identifier` ~ (`LParen` ~ (`Identifier` ~ (`Comma` ~ `Identifier` )*) ~ `RParen`)?

function_def = `Keyword(Function)` ~ `Identifier` ~ `LParen` ~ function_params ~ `RParen` ~ function_return ~  code_block

function_params = `Identifier` ~ (`Comma` ~ `Identifier`)*

function_return = (`FunctionReturn` ~ `Type`)?

## Expression

code_block = `LBraces` ~ code_line* ~ `RBraces`

statement = (for | static_for | while | if | (expression ~ `EndLine`))

expression = (var_dec | assign | any_type)

for = `Keyword(for)` ~ `LParen` ~ statement ~ statement ~ statement ~ `RParen` ~ code_block

for_list = `Keyword(for)` ~ `LParen` ~ `Identifier` ~ `Keyword(in)` ~ `Identifier` ~ `RParen` ~ code_block

static_for = `Keyword(static)` ~ `Keyword(for)` ~ `LParen` ~ `Identifier` ~ `Comma` ~ `Integer` ~ `Comma` ~ `Integer` ~ (`Comma` ~ `Integer`)? ~ `RParen` ~ code_block

while = `Keyword(while)` ~ `LParen` ~ any `RParen` ~ code_block

assign = `Identifier` ~ `Assign` ~ any

function = `Identifier` ~ `LParen` ~ expression ~ (`Comma` ~ expression)* ~ `RParen`

var_dec = `Keyword(VarDeceleration)` ~ `Identifier` ~ (`TypeClarify` ~ `Type`)? ~ `Assign` ~ expression ~ `EndLine`

if = `Keyword(If)` ~ `LParen` ~ any ~ `RParen` ~ code_block

### Math

math = term ~ (order_operation_level_1 ~ term)*

order_operation_level_1 = `Multi` | `Div`

term = expr ~ (order_operation_level_2 ~ expr)*

order_operation_level_2 = `Plus` | `Minus`

expr = function | `Integer` | `Float` | (`LParen` ~ math ~ `RParen`)

## Type

any_type = `Integer` | `Float` | `String` | struct | math | `Keyword(None)`

string = `Quote` ~ `ANY`* ~ `Quote`

bool = `Keyword(true)` | `Keyword(false)`

struct = `Identifier` ~ `LBraces` ~ struct_body ~ `RBraces`

struct_body = struct_pair ~ (`Comma` ~ struct_pair)*

struct_pair = `Identifier` ~ `TypeClarify` ~ any_type
