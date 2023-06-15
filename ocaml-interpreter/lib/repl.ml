let rec read_eval_print lexer out_channel =
  match Lexer.next_token lexer with
  | Eof, _ -> ()  (* stop when we reach EOF *)
  | token, lexer' ->  (* print token and continue *)
    Printf.fprintf out_channel "%s\n%!" (Token.show_token token);
    read_eval_print lexer' out_channel

let rec repl in_channel out_channel =
  Printf.fprintf out_channel ">> %!";
  match input_line in_channel with
  | line ->
    let lexer = Lexer.make line in
    read_eval_print lexer out_channel;
    repl in_channel out_channel  (* Loop for the next line *)
  | exception End_of_file -> ()  (* stop at end of input *)