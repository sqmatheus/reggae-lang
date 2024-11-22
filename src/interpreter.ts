import { Lexer } from "./lexer";
import { Token, TokenType } from "./token";
import { none, Option, some } from "./utils";
import { Writer } from "./writer";

export default class Interpreter {

    private cursor: number = 0
    private tokens: Token[] = []
    private variables: Map<string, string> = new Map()

    constructor(private output: Writer) { }

    private clear(): void {
        this.cursor = 0
        this.tokens = []
        this.variables = new Map()
        this.output.clear()
    }

    run(input: string): void {
        this.clear()
        const lexer = new Lexer(input)
        this.tokens = lexer.parse()
        this.execute()
    }

    private next(): Option<Token> {
        if (this.cursor >= this.tokens.length) {
            return none()
        }
        return some(this.tokens[this.cursor++])
    }

    private peek(): Option<Token> {
        if (this.cursor >= this.tokens.length) {
            return none()
        }
        return some(this.tokens[this.cursor])
    }

    private consume(type: TokenType): void {
        const next = this.next()
        if (next.isNone) throw new Error('eof')
        if (next.value.type != type) throw new Error('unexpected type')
    }

    private variableSignature(): void {
        const name = this.next().unwrap()
        this.consume(TokenType.EqualsOperator)
        const value = this.next().unwrap()
        this.consume(TokenType.SemiColon)

        this.variables.set(name.value, value.value)
        console.log(this.variables)
    }

    private funcallSignature(func: string): void {
        this.consume(TokenType.LeftParen)
        const variable = this.next().unwrap()
        this.consume(TokenType.RightParen)
        this.consume(TokenType.SemiColon)

        if (func === 'sound') {
            if (variable.type === TokenType.Identifier) {
                const value = this.variables.get(variable.value)
                this.output.write(value ?? '')
                this.output.write('\n')
            }
            if (variable.type === TokenType.Literal) {
                this.output.write(variable.value)
                this.output.write('\n')
            }
        }
    }

    private executeIdentifier(identifier: Token): void {
        if (identifier.value === 'roots') {
            this.variableSignature()
            return
        }

        const next = this.peek()
        if (!next.isSome) {
            return
        }

        const token = next.value
        if (token.type === TokenType.LeftParen) {
            this.funcallSignature(identifier.value)
        }
    }

    private execute(): void {
        let next = this.next()
        while (next.isSome) {
            const token = next.value
            if (token.type === TokenType.Identifier) {
                this.executeIdentifier(token)
            }
            next = this.next()
        }
    }

}