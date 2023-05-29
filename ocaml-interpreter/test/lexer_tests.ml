open OUnit2
open Interpreter.Lexer
open Interpreter.Token

let lexer_tests () =
  let test_next_token _ =
    let input =
      "let five = ;" 
      (* "let five = 5;"
      ^ "let ten = 10;"
      ^ "let add = fn(x, y) {"
      ^ "x + y;"
      ^ "};"
      ^ "let result = add(five, ten);" *)
    in
    let tests = [
      Let;
      Ident "five";
      Assign;
      (* Int 5; *)
      Semicolon;
      (*Let;
      Ident "ten";
      Assign;
      Int 10;
      Semicolon;
      Let;
      Ident "add";
      Assign;
      Function;
      LParen;
      Ident "x";
      Comma;
      Ident "y";
      RParen;
      LBrace;
      Ident "x";
      Plus;
      Ident "y";
      Semicolon;
      RBrace;
      Semicolon;
      Let;
      Ident "result";
      Assign;
      Ident "add";
      LParen;
      Ident "five";
      Comma;
      Ident "ten";
      RParen;
      Semicolon; *)
      Eof;
    ] in
    let rec run_tests lexer tests =
      let lexer, token = next_token lexer in
      match tests with
        | [] -> ()
        | tok :: tests -> 
          assert_equal tok token;
          run_tests lexer tests
    in
    run_tests (make input) tests
  in
  
  "Lexer Tests" >::: [
    "next_token" >:: test_next_token;
  ]

let () =
  run_test_tt_main (lexer_tests ())
   