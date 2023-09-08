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

type ASTBinaryOperator = {
  type: 'operator.implies' | 'operator.iff' | 'operator.or' | 'operator.and',
  left: ASTNode
}

type ASTLiteral = {
  type: 'literal',
  value: boolean
}

type ASTUnaryOperator = {
  type: 'operator.not',
  operand: ASTNode
}

type ASTNode = ASTBinaryOperator | ASTLiteral | ASTUnaryOperator;

export type LogicParsingResult = {
  status: 'success' | 'error',
  ast: ASTNode
}
