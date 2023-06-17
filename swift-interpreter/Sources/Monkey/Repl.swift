import Foundation

let PROMPT = ">> "

func start() {
    while true { 
			print(PROMPT, terminator: "")

			if let line = readLine() {
				var lexer = Lexer(line)
				var token = lexer.nextToken()
				
				while token != .eof {
						print(token)
						token = lexer.nextToken()
				}
			}
    }
}