import type { ASTNode } from '@types';
import { analizeTree, nodeIsBinaryOperator, nodeIsUnaryOperator } from './analize';

type ASTBinaryOperator = {
  type: 'operator.implies' | 'operator.iff' | 'operator.or' | 'operator.and',
  left: ASTNode,
  right: ASTNode
}

type ASTUnaryOperator = {
  type: 'operator.not',
  operand: ASTNode
}

type ASTOperator = ASTBinaryOperator | ASTUnaryOperator;

function operationToString(node: ASTNode): string {
  switch (node.type) {
    case 'operator.implies':
      return `${operationToString(node.left)} -> ${operationToString(node.right)}`;
    case 'operator.iff':
      return `${operationToString(node.left)} <-> ${operationToString(node.right)}`;
    case 'operator.or':
      return `${operationToString(node.left)} v ${operationToString(node.right)}`;
    case 'operator.and':
      return `${operationToString(node.left)} ^ ${operationToString(node.right)}`;
    case 'operator.not':
      return `!${operationToString(node.operand)}`;
    case 'identifier':
      return node.name;
    case 'literal':
      return node.value.toString();
  }
}


export function generateTable(tree: ASTNode): string[][] {
  const stack: ASTNode[] = [tree];
  const operations: ASTOperator[] = [];

  while (true) {
    const node = stack.pop();
    if (!node) break;

    if (nodeIsBinaryOperator(node)) {
      operations.push(node);
      console.log(JSON.stringify(node))
      stack.push(node.left);
      stack.push(node.right);
    }
    else if (nodeIsUnaryOperator(node)) {
      operations.push(node);
      stack.push(node.operand);
    }
  }
  operations.reverse();
    const { identifiers } = analizeTree(tree);
  const generatedTable: string[][] = [[...identifiers.map(id => id.name), ...operations.map(operationToString)]];

  console.log({generatedTable})
  const combinations = Math.pow(2, identifiers.length);
  // generate all possible variables combinatory
  // p q r
  // 0 0 0
  // 0 0 1
  // 0 1 0
  // ...etc

  for (let i = 0; i < combinations; i++) {
    const row: string[] = [];
    let binary = i.toString(2).padStart(identifiers.length, '0');
    const vals = binary.split('').map(Number).map(Boolean);

    for (let j = 0; j < identifiers.length; j++) {
      row.push(binary[j]);
    }

    generatedTable.push(row);
    const valuesMap = new Map<string, boolean>();
    for (let j = 0; j < identifiers.length; j++) {
      valuesMap.set(identifiers[j].name, vals[j]);
    }

    // so we now can evaluate each identifier name to a value for each operation
    // we need to evaluate each operation recursively and push it to the row
    operations.forEach(operation => {
      const evaluate = (node: ASTNode): boolean => {
        switch (node.type) {
          case 'operator.implies':
            return !evaluate(node.left) || evaluate(node.right);
          case 'operator.iff':
            return evaluate(node.left) === evaluate(node.right);
          case 'operator.or':
            return evaluate(node.left) || evaluate(node.right);
          case 'operator.and':
            return evaluate(node.left) && evaluate(node.right);
          case 'operator.not':
            return !evaluate(node.operand);
          case 'identifier':
            return valuesMap.get(node.name) ?? false;
          case 'literal':
            return node.value;
        }
      }
      row.push(evaluate(operation) ? '1' : '0');
    });
  }
  return generatedTable;
}
