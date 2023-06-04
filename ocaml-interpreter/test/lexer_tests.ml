open OUnit2
open Interpreter.Lexer
open Interpreter.Token

let lexer_tests () =
  let test_next_token _ =
    let input = 
      "let five = 5;"
      ^ "let ten = 10;"
      ^ "let add = fn(x, y) {"
      ^ "x + y;"
      ^ "};"
      ^ "let result = add(five, ten);"
      ^ "!-/*5;"
      ^ "5 < 10 > 5;"
      ^ "if (5 < 10) {"
      ^ "return true;"
      ^ "} else {"
      ^ "return false;"
      ^ "}"
      ^ "10 == 10;"
      ^ "10 != 9;"
    in

    let tests = [
      Let; Ident "five"; Assign; Int 5; Semicolon;
      Let; Ident "ten"; Assign; Int 10; Semicolon;
      Let; Ident "add"; Assign; Function; LeftParen; Ident "x"; Comma; Ident "y"; RightParen; LeftCurly;
      Ident "x"; Plus; Ident "y"; Semicolon;
      RightCurly; Semicolon;
      Let;Ident "result";
      Assign; Ident "add"; LeftParen; Ident "five"; Comma; Ident "ten"; RightParen; Semicolon;
      Bang; Minus; Slash; Asterisk; Int 5; Semicolon;
      Int 5; LessThan; Int 10; GreaterThan; Int 5; Semicolon;
      If; LeftParen; Int 5; LessThan; Int 10; RightParen; LeftCurly;
      Return; True; Semicolon;
      RightCurly; Else; LeftCurly;
      Return; False; Semicolon;
      RightCurly;
      Int 10; Equal; Int 10; Semicolon;
      Int 10; NotEqual; Int 9; Semicolon;
      Eof;
    ] in

    let rec run_tests lexer tests =
      let token, lexer = next_token lexer in
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
   