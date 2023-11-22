import type { AstroInstance } from 'astro'

export type ProjectMetadata = {
  title: string
  description: string
  img: string
}

export type ProjectPageInstance = AstroInstance & {
  metadata: ProjectMetadata
}

export type ProjectPage = {
  page: ProjectPageInstance
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

export type ASTOperator = ASTBinaryOperator | ASTUnaryOperator;

export type ASTNode =
  ASTOperator |
  ASTLiteral |
  ASTIdentifier;

export type LogicParserSuccessResult = {
  status: 'success',
  ast: ASTNode
}

export type LogicParserErrorResult = {
  status: 'error',
  error: string,
  span: [start: number, end: number]
}

export type LogicParsingResult = LogicParserSuccessResult | LogicParserErrorResult;

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

/* Project 2 */

export type RecordInformation = {
  id: string, // vennbase uuid
  mimetype: string,
  tags: string[],
}

export interface QueriedRecordResult {
  id: string;
  mimetype: string;
  tags: string[];
}
