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

export type ASTNode = ASTBinaryOperator | ASTUnaryOperator | ASTLiteral | ASTIdentifier;

export type LogicParsingResult = {
  status: 'success' | 'error',
  ast: ASTNode
}
