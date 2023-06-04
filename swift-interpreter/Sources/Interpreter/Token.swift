public enum Token : Equatable {
	// special
	case illegal, eof

	// identifiers
	case ident(String)
	case int(Int)

	// keywords
	case 
		function, 
		`let`,
		`true`,
		`false`,
		`if`,
		`else`,
		`return`

	// operators
	case 
		assign, 
		plus, 
		comma,
		semicolon,
		minus,
		bang,
		asterisk,
		slash,
		// logical
		lt,
		gt,
		eq,
		notEq
	
	// delimiters
	case
		lparen, 
		rparen, 
		lcurly, 
		rcurly
}

let keywords: [String : Token] = [
	String("fn"): .function,
	String("let"): .let,
	String("true"): .true,
	String("false"): .false,
	String("if"): .if,
	String("else"): .else,
	String("return"): .return,
]

func lookupIdent(_ ident: String) -> Token {
	if let keyword = keywords[ident] {
		return keyword
	}
	return Token.ident(String(ident))
}