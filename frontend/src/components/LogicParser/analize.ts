import type { ASTBinaryOperator, ASTNode, ASTUnaryOperator } from '@types';

export function nodeIsBinaryOperator(node: ASTNode): node is ASTBinaryOperator {
  return Object.hasOwn(node, 'left');
}

export function nodeIsUnaryOperator(node: ASTNode): node is ASTUnaryOperator {
  return Object.hasOwn(node, 'operand');
}

export function analizeTree(tree: ASTNode) {
  const stack: ASTNode[] = [tree];

  while (true) {
    const node = stack.pop();
    if (!node) break;

    if (node.type === 'identifier') {
    }

    if (nodeIsBinaryOperator(node)) {
      stack.push(node.left);
      stack.push(node.right);
    }
    else if (nodeIsUnaryOperator(node)) {
      stack.push(node.operand);
    }
  }
}
