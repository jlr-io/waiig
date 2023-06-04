public struct Lexer {
	let input: String
	var position: Int
	var ch: Character?

	public init(_ input: String) {
		self.input = input
		self.position = 0
		self.ch = !input.isEmpty ? input.first : nil  
	}

	public mutating func nextToken() -> Token {	
		self.skipWhitespace()
		let token: Token
		switch self.ch {
			case nil : token = .eof
			case "=" : token = self.hasNext(ch: "=", isToken: .eq, elseIsToken: .assign)
			case "+" : token = .plus
			case "(" : token = .lparen
			case ")" : token = .rparen
			case "{" : token = .lcurly
			case "}" : token = .rcurly
			case "," : token = .comma
			case ";" : token = .semicolon
			case "-" : token = .minus
			case "!" : token = self.hasNext(ch: "=", isToken: .notEq, elseIsToken: .bang)
			case "*" : token = .asterisk
			case "/" : token = .slash
			case "<" : token = .lt
			case ">" : token = .gt
			case let .some(ch) where ch.isLetter : return readIdent()
			case let .some(ch) where ch.isNumber : return readDigit()
			default: token = .illegal
		}
		
		self.advance()
		return token
	}

	mutating func advance() {
		let nextCh = peekNext()
		if nextCh != nil {
			self.position += 1
		}
		self.ch = nextCh
	}

	func peekNext() -> Character? {
		let readPosition = self.position + 1
  	if readPosition >= self.input.count {
			return nil
		}
		let index = self.input.index(self.input.startIndex, offsetBy: readPosition)
		return self.input[index];
	}

	mutating func hasNext(ch: Character, isToken: Token, elseIsToken: Token) -> Token {
		if self.peekNext() == ch {
			self.advance()
			return isToken
		} else {
			return elseIsToken;
		}
	}

	mutating func readWhile(_ pred: (Character) -> Bool) -> String {
		var str = String.init()
		while let ch = self.ch, pred(ch) {
			str += String(ch)
			self.advance()
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