type token =
  | Illegal
  | Eof
  | Ident of string
  | Int of int
  | Assign
  | Plus
  | Comma
  | Semicolon
  | LParen
  | RParen
  | LBrace
  | RBrace
  | Function
  | Let

let lookup_ident ident =
  match ident with
  | "fn" -> Function
  | "let" -> Let
  | _ -> Ident ident