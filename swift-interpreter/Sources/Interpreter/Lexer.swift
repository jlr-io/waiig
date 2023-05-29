public struct Lexer {
	let input: String
	var position: Int
	var readPosition: Int
	var ch: Character?

	public init(_ input: String) {
		self.input = input
		self.position = 0
		self.readPosition = 1
		self.ch = input.first
	}

	public mutating func nextToken() -> Token {	
		self.skipWhitespace()
		let token: Token
		switch self.ch {
			case nil : token = .eof
			case "=" : token = .assign
			case "+" : token = .plus
			case "(" : token = .lparen
			case ")" : token = .rparen
			case "{" : token = .lbrace
			case "}" : token = .rbrace
			case "," : token = .comma
			case ";" : token = .semicolon
			case let .some(ch) where ch.isLetter : return readIdent() // <- early return
			case let .some(ch) where ch.isNumber : return readDigit() // <- early return
			default: token = .illegal
		}
		
		self.readChar()
		return token
	}

	mutating func readChar() {
		if self.readPosition >= self.input.count{
			self.ch = nil
		} else {
			let index = self.input.index(self.input.startIndex, offsetBy: self.readPosition)
			self.ch = self.input[index]
		}
		self.position = self.readPosition
		self.readPosition += 1
	}

	mutating func readWhile(_ pred: (Character) -> Bool) -> String {
		var str = ""
		while let ch = self.ch, pred(ch) {
			str += String(ch)
			readChar()
		}
		return str
	}

	mutating func skipWhitespace() {
		let _ = readWhile({ $0.isWhitespace })
	}

	mutating func readIdent() -> Token {
		let ident = readWhile({ $0.isLetter })
		return lookupIdent(ident)
	}

	mutating func readDigit() -> Token {
		let digit = readWhile({ $0.isNumber })
		return Int(digit).map(Token.int) ?? .illegal
	}
}