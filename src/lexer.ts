import { Token, TokenType } from "./token";
import { none, Option, some } from "./utils";

export class Lexer {

    private cursor: number = 0;

    constructor(private input: string) {
    }

    private next(): Option<string> {
        if (this.cursor >= this.input.length) {
            return none()
        }
        return some(this.input.charAt(this.cursor++))
    }

    private peek(): Option<string> {
        if (this.cursor >= this.input.length) {
            return none()
        }
        return some(this.input.charAt(this.cursor))
    }

    private skipWhitespace() {
        let n = this.peek()
        while (n.isSome && /\s/.test(n.value)) {
            this.next()
            n = this.peek()
        }
    }

    private isAlphabetic(str: string): boolean {
        return /^[a-zA-Z]+$/.test(str);
    }

    private isAlphanumeric(str: string): boolean {
        return /^[a-zA-Z0-9_]+$/.test(str);
    }

    private parseIdentifier(start: string): Token {
        let identifier = start
        let n = this.peek()
        while (n.isSome && this.isAlphanumeric(n.value)) {
            identifier += this.next().unwrap()
            n = this.peek()
        }

        return new Token(TokenType.Identifier, identifier)
    }

    private parseStringLiteral(): Token {
        let literal = ''
        let n = this.peek()
        while (n.isSome && n.value != '"') {
            literal += this.next().unwrap()
            n = this.peek()
        }

        if (n.unwrap() != '"') {
            throw new Error("eof")
        }

        this.next()

        return new Token(TokenType.Literal, literal)
    }

    private getToken(): Option<Token> {
        let n = this.next()
        if (n.isNone) {
            return none()
        }

        const char = n.value

        switch (char) {
            case '(': return some(new Token(TokenType.LeftParen, '('));
            case ')': return some(new Token(TokenType.RightParen, ')'));
            case ';': return some(new Token(TokenType.SemiColon, ';'));
            case '=': return some(new Token(TokenType.EqualsOperator, '='));
            case '"': return some(this.parseStringLiteral());
        }

        if (this.isAlphabetic(char)) {
            return some(this.parseIdentifier(char))
        }

        return none()
    }

    parse(): Token[] {
        const tokens: Token[] = []

        this.skipWhitespace()
        let token = this.getToken()
        while (token.isSome) {
            tokens.push(token.value)
            this.skipWhitespace()
            token = this.getToken()
        }

        return tokens
    }

}