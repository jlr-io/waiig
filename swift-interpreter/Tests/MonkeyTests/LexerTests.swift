import XCTest
@testable import Monkey

final class LexerTests: XCTestCase {
    func testNextToken() {
				let input = """
					let five = 5;
					let ten = 10;

					let add = fn(x, y) {
					x + y;
					};

					let result = add(five, ten);
					!-/*5;
					5 < 10 > 5;

					if (5 < 10) {
						return true;
					} else {
						return false;
					}

					10 == 10;
					10 != 9;
				"""
				
        var lexer = Lexer.init(input);
				let tests: [Token] = [
					.let, .ident("five"), .assign, .int(5), .semicolon,
					.let, .ident("ten"), .assign, .int(10), .semicolon,
					.let, .ident("add"), .assign, .function, .lparen, .ident("x"), .comma, .ident("y"), .rparen, .lcurly,
					.ident("x"), .plus, .ident("y"), .semicolon,
					.rcurly, .semicolon,
					.let, .ident("result"), .assign, .ident("add"), .lparen, .ident("five"), .comma, .ident("ten"), .rparen, .semicolon,
					.bang, .minus, .slash, .asterisk, .int(5), .semicolon,
					.int(5), .lt, .int(10), .gt, .int(5), .semicolon,
					.if, .lparen, .int(5), .lt, .int(10), .rparen, .lcurly,
					.return, .true, .semicolon,
					.rcurly, .else, .lcurly,
					.return, .false, .semicolon,
					.rcurly,
					.int(10), .eq, .int(10), .semicolon,
					.int(10), .notEq, .int(9), .semicolon,
					.eof,
				]
				for test in tests {
					XCTAssertEqual(test, lexer.nextToken())
				}
    }

    static var allTests = [
        ("testNextToken", testNextToken),
    ]
}
