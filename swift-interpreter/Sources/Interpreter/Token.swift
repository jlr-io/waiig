public enum Token : Equatable {
	case ident(String)
	case int(Int)
	case 
		assign, 
		plus, 
		comma, 
		semicolon, 
		lparen, 
		rparen, 
		lbrace, 
		rbrace, 
		function, 
		`let`,
		illegal,
		eof
}

let keywords: [String : Token] = [
	String("fn"): .function,
	String("let"): .let,
]

func lookupIdent(_ ident: String) -> Token {
	if let keyword = keywords[ident] {
		return keyword
	}
	return Token.ident(String(ident))
}