// Token-level grammar, for highlighting only: no statement structure.
// Hangul is lexed one syllable at a time so a keyword glued to its particle still wins the
// longest match (정수를 → 정수 + 를); one-syllable keywords (값, 개, 빈) are left out, they would
// light up inside names
module.exports = grammar({
  name: 'kr',

  extras: $ => [/\s/],

  rules: {
    source_file: $ => repeat($._token),

    _token: $ => choice(
      $.comment, $.string, $.number, $.boolean, $.type, $.keyword,
      $.identifier, $.syllable, $.operator, $.punctuation, $.bracket,
    ),

    comment: _ => token(choice(/\/\/[^\n]*/, /\/\*[^*]*\*+([^/*][^*]*\*+)*\//)),

    string: $ => seq('"', repeat(choice($.escape, token.immediate(/[^"\\\n]+/))), token.immediate('"')),
    escape: _ => token.immediate(/\\[nt0\\"]/),

    number: _ => /[0-9]+(\.[0-9]+)?/,
    boolean: _ => choice('참', '거짓'),

    type: _ => choice('정수', '소수', '논리', '문자', '바이트', '주소', '짧은', '부호 없는', '고정된'),
    keyword: _ => choice(
      '외부', '반환하는', '반환해요', '반환', '받아', '세면서', '그만해요', '넘어가요', '크기',
      '만약', '아니면', '아닌', '동안', '그리고', '또는',
    ),

    identifier: _ => /[a-zA-Z_][a-zA-Z0-9_]*/,
    syllable: _ => /[가-힣]/,

    operator: _ => choice('<<', '>>', '<=', '>=', '==', '!=', '+', '-', '*', '/', '%', '&', '|', '^', '~', '<', '>'),
    punctuation: _ => choice('...', '…', ',', '.'),
    bracket: _ => choice('(', ')', '{', '}'),
  },
});
