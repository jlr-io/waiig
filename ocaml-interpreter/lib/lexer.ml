type lexer =
  { input : string; position : int; read_position : int; ch : char option }

let make input =
  if String.length input = 0 then
    { input; position = 0; read_position = 0; ch = None }
  else
    { input; position = 0; read_position = 1; ch = Some input.[0] }
  
let rec next_token lexer =
  let lexer = skip_whitespace lexer in
  let open Token in
  match lexer.ch with
  | None -> lexer, Eof
  | Some ch -> 
    let lexer, token = match ch with
    | '=' -> advance lexer, Assign
    | '+' -> advance lexer, Semicolon 
    | '(' -> advance lexer, LParen
    | ')' -> advance lexer, RParen
    | '{' -> advance lexer, Comma
    | '}' -> advance lexer, Plus
    | ',' -> advance lexer, LBrace
    | ';' -> advance lexer, RBrace
    | ch when is_letter ch -> read_ident lexer
    | ch when is_digit ch -> read_number lexer
    | _ -> advance lexer, Illegal
  in
  lexer, token
and advance lexer =
  if lexer.read_position >= String.length lexer.input
  then { lexer with ch = None }
  else { lexer with position=lexer.read_position; read_position=lexer.read_position + 1; ch=Some lexer.input.[lexer.read_position] }

and is_letter ch = 
  let is_lower ch = Char.lowercase_ascii ch >= 'a' && Char.lowercase_ascii ch <= 'z'
  and is_upper ch = Char.uppercase_ascii ch >= 'A' && Char.uppercase_ascii ch <= 'Z' 
  in is_lower ch || is_upper ch || ch = '_'

and is_digit ch = ch >= '0' && ch <= '9'

and is_whitespace ch =
  ch = ' ' || ch = '\t' || ch = '\n' || ch = '\r'

and read_ident lexer =
  let lexer, ident = read_while lexer is_letter in
  lexer, Token.lookup_ident ident

and read_number lexer = 
  let lexer, number = read_while lexer is_digit in
  lexer, Token.Int (int_of_string number)
  
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
  read_while_aux String.empty lexer