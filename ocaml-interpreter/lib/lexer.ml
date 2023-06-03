open Base

type lexer = { input : string; position : int; ch : char option }

let make input =
  if String.is_empty input then
    { input; position = 0; ch = None }
  else
    { input; position = 0; ch = Some (String.get input 0) }
  
let rec next_token lexer =
  let open Token in
  let lexer = skip_whitespace lexer in
  match lexer.ch with
  | None -> lexer, Eof
  | Some ch -> 
    let token, lexer = match ch with
    | '=' -> Assign, advance lexer
    | '+' -> Plus, advance lexer
    | '(' -> LParen, advance lexer
    | ')' -> RParen, advance lexer
    | '{' -> LBrace, advance lexer
    | '}' -> RBrace, advance lexer
    | ',' -> Comma, advance lexer
    | ';' -> Semicolon, advance lexer
    | ch when is_letter ch -> read_ident_and_advance lexer
    | ch when is_digit ch -> read_number_and_advance lexer
    | _ -> Illegal, advance lexer 
  in
  lexer, token

and advance lexer =
  let ch = peek_next lexer in
  match ch with
  | None -> { lexer with ch }
  | ch -> { lexer with position= (inc lexer.position); ch }

and peek_next lexer =
  if inc lexer.position >= String.length lexer.input
  then None
  else Some (String.get lexer.input (inc lexer.position))
    
and read_ident_and_advance lexer =
  let lexer, ident = read_while lexer is_letter in
  Token.lookup_ident ident, lexer

and read_number_and_advance lexer = 
  let lexer, number = read_while lexer is_digit in
  Token.Int (Int.of_string number), lexer 
  
and skip_whitespace lexer =
  let lexer, _ = read_while lexer is_whitespace in
  lexer

and read_while lexer pred =
  let rec read_while_aux acc lexer =
    match lexer.ch with
    | None -> lexer, acc
    | Some ch -> if pred ch
      then read_while_aux (acc ^ String.make 1 ch) (advance lexer)
      else lexer, acc
  in
  read_while_aux Caml.String.empty lexer

and is_letter ch = 
  Char.(is_alpha ch || ch = '_')
and is_digit = Char.is_digit
and is_whitespace = Char.is_whitespace
and inc = Int.succ