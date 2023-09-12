import type { AstroInstance } from 'astro'

export type WeekMetadata = {
  title: string
  description: string
  img: string
}

export type WeekPage = {
  page: AstroInstance & {
    metadata: WeekMetadata
  }
}

export type ASTBinaryOperator = {
  type: 'operator.implies' | 'operator.iff' | 'operator.or' | 'operator.and',
  left: ASTNode,
  right: ASTNode
}

export type ASTLiteral = {
  type: 'literal',
  value: boolean
}

export type ASTIdentifier = {
  type: 'identifier',
  name: string
}

export type ASTUnaryOperator = {
  type: 'operator.not',
  operand: ASTNode
}

export type ASTNode =
  ASTBinaryOperator |
  ASTUnaryOperator |
  ASTLiteral |
  ASTIdentifier;

export type LogicParsingResult = {
  status: 'success' | 'error',
  ast: ASTNode
}

type TokenIdentifier = {
  kind: 'identifier',
  value: string
}
type TokenLiteral = {
  kind: 'literal',
  value: boolean
}
type TokenNot = {
  kind: 'not';
}
type TokenAnd = {
  kind: 'and';
}
type TokenOr = {
  kind: 'or';
}
type TokenImplies = {
  kind: 'implies';
}
type TokenIfAndOnlyIf = {
  kind: 'iff';
}
type TokenOpenParen = {
  kind: 'openparen';
}
type TokenCloseParen = {
  kind: 'closeparen';
}

export type TokenKind =
  TokenIdentifier |
  TokenLiteral |
  TokenNot |
  TokenAnd |
  TokenOr |
  TokenImplies |
  TokenIdentifier |
  TokenIfAndOnlyIf |
  TokenOpenParen |
  TokenCloseParen;

export type Token = TokenKind & { span: [start: number, end: number] };
