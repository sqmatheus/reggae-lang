export enum TokenType {
    LeftParen,
    RightParen,
    SemiColon,
    Identifier,
    Keyword,
    EqualsOperator,
    Literal
}

export class Token {

    constructor(
        public readonly type: TokenType,
        public readonly value: string
    ) { }

}

