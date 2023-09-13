import type { ASTBinaryOperator, ASTIdentifier, ASTNode, ASTUnaryOperator } from '@types';

export function nodeIsBinaryOperator(node: ASTNode): node is ASTBinaryOperator {
  return Object.hasOwn(node, 'left');
}

export function nodeIsUnaryOperator(node: ASTNode): node is ASTUnaryOperator {
  return Object.hasOwn(node, 'operand');
}

export function analizeTree(tree: ASTNode): { identifiers: ASTIdentifier[] } {
  const stack: ASTNode[] = [tree];
  const identifiers: ASTIdentifier[] = [];

  while (true) {
    const node = stack.pop();
    if (!node) break;

    if (node.type === 'identifier') {
      if (!identifiers.some(id => id.name === node.name))  {
        identifiers.push(node);
      }
    }

    if (nodeIsBinaryOperator(node)) {
      stack.push(node.left);
      stack.push(node.right);
    }
    else if (nodeIsUnaryOperator(node)) {
      stack.push(node.operand);
    }
  }

  return {
    identifiers
  };
}
