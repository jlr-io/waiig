import XCTest
@testable import Interpreter

final class LexerTests: XCTestCase {
    func testNextToken() {
				let input = """
					let five = 5;
					let ten = 10;

					let add = fn(x, y) {
					x + y;
					};

					let result = add(five, ten);
				"""
				
        var lexer = Lexer.init(input);
				let tests: [Token] = [
					.let,
					.ident("five"),
					.assign,
					.int(5),
					.semicolon,
					.let,
					.ident("ten"),
					.assign,
					.int(10),
					.semicolon,
					.let,
					.ident("add"),
					.assign,
					.function,
					.lparen,
					.ident("x"),
					.comma,
					.ident("y"),
					.rparen,
					.lbrace,
					.ident("x"),
					.plus,
					.ident("y"),
					.semicolon,
					.rbrace,
					.semicolon,
					.let,
					.ident("result"),
					.assign,
					.ident("add"),
					.lparen,
					.ident("five"),
					.comma,
					.ident("ten"),
					.rparen,
					.semicolon,
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
