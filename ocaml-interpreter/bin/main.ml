open Interpreter.Repl

let print_greeting () =
  print_newline ();
  print_endline "Hello! This is the Monkey programming language!";
  print_endline "Feel free to type in commands";
  print_newline ()

let () =
  print_greeting ();
  let in_channel = stdin in
  let out_channel = stdout in
  repl in_channel out_channel

