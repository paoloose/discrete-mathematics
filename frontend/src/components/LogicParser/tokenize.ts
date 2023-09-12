import type { Token, TokenKind } from '@types';

export function isUnicodeAlphabetic(c: string): boolean {
  return /\p{L}/u.test(c);
}

export function isUnicodeAlphanumericOrUnderscore(c: string): boolean {
  return c === '_' || /\p{L}|\d/u.test(c);
}

// The tokenizing logic can be just exposed from the rust crate, but
// I found it found to rewrite that logic in TypeScript
export function tokenizeExpr(expr: string): Token[] {
  const tokens: Token[] = [];
  let pos = 0;

  const consume = () => {
    return expr.charAt(pos++);
  };

  const peek = () => {
    return expr.charAt(pos);
  };

  const consumeWhile = (f: (c: string) => boolean) => {
    let str = '';
    while (f(peek())) {
      str += consume();
    }
    return str;
  };

  const consumeExact = (str: string) => {
    if (expr.slice(pos, pos + str.length) === str) {
      pos += str.length;
      return true;
    }
    return false;
  };

  const skipWhitespaces = () => {
    consumeWhile((c) => c === '\t' || c === ' ' || c === '\r');
  };

  while (pos < expr.length) {
    skipWhitespaces();
    const c = peek();
    if (!c) break;

    const start = pos;
    let tokenKind: TokenKind;

    if (c === '!' || c === '~') {
      consume();
      tokenKind = { kind: 'not' };
    }
    else if (c === '(') {
      consume();
      tokenKind = { kind: 'openparen' };
    }
    else if (c === ')') {
      consume();
      tokenKind = { kind: 'closeparen' };
    }
    else if (['=>', '->'].some(consumeExact)) {
      tokenKind = { kind: 'implies' };
    }
    else if (['<=>', '<->'].some(consumeExact)) {
      tokenKind = { kind: 'iff' };
    }
    else if (['&&', '&'].some(consumeExact)) {
      tokenKind = { kind: 'and' };
    }
    else if (['||', '|'].some(consumeExact)) {
      tokenKind = { kind: 'or' };
    }
    else if (isUnicodeAlphabetic(c) || c === '_') {
      tokenKind = {
        kind: 'identifier',
        value: consumeWhile(isUnicodeAlphanumericOrUnderscore)
      };
    }
    else {
      throw new SyntaxError(`unknown token '${c}'`);
    }

    tokens.push({ ...tokenKind, span: [start, pos] });
  }
  return tokens;
}
