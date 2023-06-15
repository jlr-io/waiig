type token =
(* Special *)
| Illegal
| Eof
(* Identifiers *)
| Ident of string
| Int of int
(* Keywords *)
| Function
| Let
| True
| False
| If
| Else
| Return
(* Symbols *)
(* Operators *)
| Assign
| Plus
| Minus
| Bang
| Asterisk
| Slash
(* Logical *)
| LessThan
| GreaterThan
| Equal
| NotEqual
(* Delimiters *)
| Comma
| Semicolon
| LeftParen
| RightParen
| LeftCurly
| RightCurly
[@@deriving show]

let keywords = [
  ("fn", Function);
  ("let", Let);
  ("true", True);
  ("false", False);
  ("if", If);
  ("else", Else);
  ("return", Return);
]

let lookup_ident ident =
  match List.assoc_opt ident keywords with
  | Some keyword -> keyword
  | None -> Ident ident