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
    | None -> Eof, lexer
    | Some ch -> match ch with
      | ch when is_letter ch -> read_ident_and_advance lexer
      | ch when is_digit ch -> read_number_and_advance lexer
      | _ -> read_symbol_and_advance lexer

and advance lexer =
  let ch = peek_next lexer in
  match ch with
  | None -> { lexer with ch }
  | ch -> { lexer with position= (inc lexer.position); ch }

and peek_next lexer =
  if inc lexer.position >= String.length lexer.input
  then None
  else Some (String.get lexer.input (inc lexer.position))

and if_followed_by lexer value ~is ~elseis=
  let token, lexer = match peek_next lexer with
  | Some next when Char.(next = value) -> is, advance lexer
  | _ -> elseis, lexer
  in
  token, advance lexer
  
and read_ident_and_advance lexer =
  let lexer, ident = read_while lexer is_letter in
  Token.lookup_ident ident, lexer

and read_number_and_advance lexer = 
  let lexer, number = read_while lexer is_digit in
  Token.Int (Int.of_string number), lexer 

and read_symbol_and_advance lexer = 
  let open Token in
  match lexer.ch with
    | None -> Eof, lexer
    | Some ch -> 
      let token, lexer = match ch with 
      | '=' -> if_followed_by lexer '=' ~is:Equal ~elseis:Assign
      | '+' -> Plus, advance lexer
      | '(' -> LeftParen, advance lexer
      | ')' -> RightParen, advance lexer
      | '{' -> LeftCurly, advance lexer
      | '}' -> RightCurly, advance lexer
      | ',' -> Comma, advance lexer
      | ';' -> Semicolon, advance lexer
      | '-' -> Minus, advance lexer
      | '!' -> if_followed_by lexer '=' ~is:NotEqual ~elseis:Bang
      | '*' -> Asterisk, advance lexer
      | '/' -> Slash, advance lexer
      | '<' -> LessThan, advance lexer
      | '>' -> GreaterThan, advance lexer
      | _ -> Illegal, advance lexer
    in
    token, lexer
  
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