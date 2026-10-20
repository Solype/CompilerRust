#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 16
#define LARGE_STATE_COUNT 12
#define SYMBOL_COUNT 71
#define ALIAS_COUNT 0
#define TOKEN_COUNT 60
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_comment = 1,
  anon_sym_DQUOTE = 2,
  aux_sym_string_token1 = 3,
  anon_sym_DQUOTE2 = 4,
  sym_escape = 5,
  sym_number = 6,
  anon_sym_ucc38 = 7,
  anon_sym_uac70uc9d3 = 8,
  anon_sym_uc815uc218 = 9,
  anon_sym_uc18cuc218 = 10,
  anon_sym_ub17cub9ac = 11,
  anon_sym_ubb38uc790 = 12,
  anon_sym_ubc14uc774ud2b8 = 13,
  anon_sym_uc8fcuc18c = 14,
  anon_sym_uc9e7uc740 = 15,
  anon_sym_ubd80ud638uc5c6ub294 = 16,
  anon_sym_uace0uc815ub41c = 17,
  anon_sym_uc678ubd80 = 18,
  anon_sym_ubc18ud658ud558ub294 = 19,
  anon_sym_ubc18ud658ud574uc694 = 20,
  anon_sym_ubc18ud658 = 21,
  anon_sym_ubc1buc544 = 22,
  anon_sym_uc138uba74uc11c = 23,
  anon_sym_uadf8ub9ccud574uc694 = 24,
  anon_sym_ub118uc5b4uac00uc694 = 25,
  anon_sym_ud06cuae30 = 26,
  anon_sym_ub9ccuc57d = 27,
  anon_sym_uc544ub2c8uba74 = 28,
  anon_sym_uc544ub2cc = 29,
  anon_sym_ub3d9uc548 = 30,
  anon_sym_uadf8ub9acuace0 = 31,
  anon_sym_ub610ub294 = 32,
  sym_identifier = 33,
  sym_syllable = 34,
  anon_sym_LT_LT = 35,
  anon_sym_GT_GT = 36,
  anon_sym_LT_EQ = 37,
  anon_sym_GT_EQ = 38,
  anon_sym_EQ_EQ = 39,
  anon_sym_BANG_EQ = 40,
  anon_sym_PLUS = 41,
  anon_sym_DASH = 42,
  anon_sym_STAR = 43,
  anon_sym_SLASH = 44,
  anon_sym_PERCENT = 45,
  anon_sym_AMP = 46,
  anon_sym_PIPE = 47,
  anon_sym_CARET = 48,
  anon_sym_TILDE = 49,
  anon_sym_LT = 50,
  anon_sym_GT = 51,
  anon_sym_DOT_DOT_DOT = 52,
  anon_sym_u2026 = 53,
  anon_sym_COMMA = 54,
  anon_sym_DOT = 55,
  anon_sym_LPAREN = 56,
  anon_sym_RPAREN = 57,
  anon_sym_LBRACE = 58,
  anon_sym_RBRACE = 59,
  sym_source_file = 60,
  sym__token = 61,
  sym_string = 62,
  sym_boolean = 63,
  sym_type = 64,
  sym_keyword = 65,
  sym_operator = 66,
  sym_punctuation = 67,
  sym_bracket = 68,
  aux_sym_source_file_repeat1 = 69,
  aux_sym_string_repeat1 = 70,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_comment] = "comment",
  [anon_sym_DQUOTE] = "\"",
  [aux_sym_string_token1] = "string_token1",
  [anon_sym_DQUOTE2] = "\"",
  [sym_escape] = "escape",
  [sym_number] = "number",
  [anon_sym_ucc38] = "\ucc38",
  [anon_sym_uac70uc9d3] = "\uac70\uc9d3",
  [anon_sym_uc815uc218] = "\uc815\uc218",
  [anon_sym_uc18cuc218] = "\uc18c\uc218",
  [anon_sym_ub17cub9ac] = "\ub17c\ub9ac",
  [anon_sym_ubb38uc790] = "\ubb38\uc790",
  [anon_sym_ubc14uc774ud2b8] = "\ubc14\uc774\ud2b8",
  [anon_sym_uc8fcuc18c] = "\uc8fc\uc18c",
  [anon_sym_uc9e7uc740] = "\uc9e7\uc740",
  [anon_sym_ubd80ud638uc5c6ub294] = "\ubd80\ud638 \uc5c6\ub294",
  [anon_sym_uace0uc815ub41c] = "\uace0\uc815\ub41c",
  [anon_sym_uc678ubd80] = "\uc678\ubd80",
  [anon_sym_ubc18ud658ud558ub294] = "\ubc18\ud658\ud558\ub294",
  [anon_sym_ubc18ud658ud574uc694] = "\ubc18\ud658\ud574\uc694",
  [anon_sym_ubc18ud658] = "\ubc18\ud658",
  [anon_sym_ubc1buc544] = "\ubc1b\uc544",
  [anon_sym_uc138uba74uc11c] = "\uc138\uba74\uc11c",
  [anon_sym_uadf8ub9ccud574uc694] = "\uadf8\ub9cc\ud574\uc694",
  [anon_sym_ub118uc5b4uac00uc694] = "\ub118\uc5b4\uac00\uc694",
  [anon_sym_ud06cuae30] = "\ud06c\uae30",
  [anon_sym_ub9ccuc57d] = "\ub9cc\uc57d",
  [anon_sym_uc544ub2c8uba74] = "\uc544\ub2c8\uba74",
  [anon_sym_uc544ub2cc] = "\uc544\ub2cc",
  [anon_sym_ub3d9uc548] = "\ub3d9\uc548",
  [anon_sym_uadf8ub9acuace0] = "\uadf8\ub9ac\uace0",
  [anon_sym_ub610ub294] = "\ub610\ub294",
  [sym_identifier] = "identifier",
  [sym_syllable] = "syllable",
  [anon_sym_LT_LT] = "<<",
  [anon_sym_GT_GT] = ">>",
  [anon_sym_LT_EQ] = "<=",
  [anon_sym_GT_EQ] = ">=",
  [anon_sym_EQ_EQ] = "==",
  [anon_sym_BANG_EQ] = "!=",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [anon_sym_STAR] = "*",
  [anon_sym_SLASH] = "/",
  [anon_sym_PERCENT] = "%",
  [anon_sym_AMP] = "&",
  [anon_sym_PIPE] = "|",
  [anon_sym_CARET] = "^",
  [anon_sym_TILDE] = "~",
  [anon_sym_LT] = "<",
  [anon_sym_GT] = ">",
  [anon_sym_DOT_DOT_DOT] = "...",
  [anon_sym_u2026] = "\u2026",
  [anon_sym_COMMA] = ",",
  [anon_sym_DOT] = ".",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [sym_source_file] = "source_file",
  [sym__token] = "_token",
  [sym_string] = "string",
  [sym_boolean] = "boolean",
  [sym_type] = "type",
  [sym_keyword] = "keyword",
  [sym_operator] = "operator",
  [sym_punctuation] = "punctuation",
  [sym_bracket] = "bracket",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_string_repeat1] = "string_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_comment] = sym_comment,
  [anon_sym_DQUOTE] = anon_sym_DQUOTE,
  [aux_sym_string_token1] = aux_sym_string_token1,
  [anon_sym_DQUOTE2] = anon_sym_DQUOTE,
  [sym_escape] = sym_escape,
  [sym_number] = sym_number,
  [anon_sym_ucc38] = anon_sym_ucc38,
  [anon_sym_uac70uc9d3] = anon_sym_uac70uc9d3,
  [anon_sym_uc815uc218] = anon_sym_uc815uc218,
  [anon_sym_uc18cuc218] = anon_sym_uc18cuc218,
  [anon_sym_ub17cub9ac] = anon_sym_ub17cub9ac,
  [anon_sym_ubb38uc790] = anon_sym_ubb38uc790,
  [anon_sym_ubc14uc774ud2b8] = anon_sym_ubc14uc774ud2b8,
  [anon_sym_uc8fcuc18c] = anon_sym_uc8fcuc18c,
  [anon_sym_uc9e7uc740] = anon_sym_uc9e7uc740,
  [anon_sym_ubd80ud638uc5c6ub294] = anon_sym_ubd80ud638uc5c6ub294,
  [anon_sym_uace0uc815ub41c] = anon_sym_uace0uc815ub41c,
  [anon_sym_uc678ubd80] = anon_sym_uc678ubd80,
  [anon_sym_ubc18ud658ud558ub294] = anon_sym_ubc18ud658ud558ub294,
  [anon_sym_ubc18ud658ud574uc694] = anon_sym_ubc18ud658ud574uc694,
  [anon_sym_ubc18ud658] = anon_sym_ubc18ud658,
  [anon_sym_ubc1buc544] = anon_sym_ubc1buc544,
  [anon_sym_uc138uba74uc11c] = anon_sym_uc138uba74uc11c,
  [anon_sym_uadf8ub9ccud574uc694] = anon_sym_uadf8ub9ccud574uc694,
  [anon_sym_ub118uc5b4uac00uc694] = anon_sym_ub118uc5b4uac00uc694,
  [anon_sym_ud06cuae30] = anon_sym_ud06cuae30,
  [anon_sym_ub9ccuc57d] = anon_sym_ub9ccuc57d,
  [anon_sym_uc544ub2c8uba74] = anon_sym_uc544ub2c8uba74,
  [anon_sym_uc544ub2cc] = anon_sym_uc544ub2cc,
  [anon_sym_ub3d9uc548] = anon_sym_ub3d9uc548,
  [anon_sym_uadf8ub9acuace0] = anon_sym_uadf8ub9acuace0,
  [anon_sym_ub610ub294] = anon_sym_ub610ub294,
  [sym_identifier] = sym_identifier,
  [sym_syllable] = sym_syllable,
  [anon_sym_LT_LT] = anon_sym_LT_LT,
  [anon_sym_GT_GT] = anon_sym_GT_GT,
  [anon_sym_LT_EQ] = anon_sym_LT_EQ,
  [anon_sym_GT_EQ] = anon_sym_GT_EQ,
  [anon_sym_EQ_EQ] = anon_sym_EQ_EQ,
  [anon_sym_BANG_EQ] = anon_sym_BANG_EQ,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_PERCENT] = anon_sym_PERCENT,
  [anon_sym_AMP] = anon_sym_AMP,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_CARET] = anon_sym_CARET,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_LT] = anon_sym_LT,
  [anon_sym_GT] = anon_sym_GT,
  [anon_sym_DOT_DOT_DOT] = anon_sym_DOT_DOT_DOT,
  [anon_sym_u2026] = anon_sym_u2026,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [sym_source_file] = sym_source_file,
  [sym__token] = sym__token,
  [sym_string] = sym_string,
  [sym_boolean] = sym_boolean,
  [sym_type] = sym_type,
  [sym_keyword] = sym_keyword,
  [sym_operator] = sym_operator,
  [sym_punctuation] = sym_punctuation,
  [sym_bracket] = sym_bracket,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_string_repeat1] = aux_sym_string_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_DQUOTE] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_string_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_DQUOTE2] = {
    .visible = true,
    .named = false,
  },
  [sym_escape] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_ucc38] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uac70uc9d3] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc815uc218] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc18cuc218] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ub17cub9ac] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ubb38uc790] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ubc14uc774ud2b8] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc8fcuc18c] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc9e7uc740] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ubd80ud638uc5c6ub294] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uace0uc815ub41c] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc678ubd80] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ubc18ud658ud558ub294] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ubc18ud658ud574uc694] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ubc18ud658] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ubc1buc544] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc138uba74uc11c] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uadf8ub9ccud574uc694] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ub118uc5b4uac00uc694] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ud06cuae30] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ub9ccuc57d] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc544ub2c8uba74] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uc544ub2cc] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ub3d9uc548] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_uadf8ub9acuace0] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ub610ub294] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_syllable] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LT_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_EQ_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_BANG_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PERCENT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_AMP] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_CARET] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT_DOT_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_u2026] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__token] = {
    .visible = false,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_boolean] = {
    .visible = true,
    .named = true,
  },
  [sym_type] = {
    .visible = true,
    .named = true,
  },
  [sym_keyword] = {
    .visible = true,
    .named = true,
  },
  [sym_operator] = {
    .visible = true,
    .named = true,
  },
  [sym_punctuation] = {
    .visible = true,
    .named = true,
  },
  [sym_bracket] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_string_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(24);
      ADVANCE_MAP(
        '!', 6,
        '"', 29,
        '%', 92,
        '&', 93,
        '(', 103,
        ')', 104,
        '*', 90,
        '+', 88,
        ',', 101,
        '-', 89,
        '.', 102,
        '/', 91,
        '<', 97,
        '=', 7,
        '>', 98,
        '\\', 21,
        '^', 95,
        '{', 105,
        '|', 94,
        '}', 106,
        '~', 96,
        0x2026, 100,
        0xac70, 79,
        0xace0, 78,
        0xadf8, 64,
        0xb118, 74,
        0xb17c, 65,
        0xb3d9, 72,
        0xb610, 62,
        0xb9cc, 73,
        0xbb38, 77,
        0xbc14, 76,
        0xbc18, 81,
        0xbc1b, 71,
        0xbd80, 80,
        0xc138, 66,
        0xc18c, 69,
        0xc544, 63,
        0xc678, 67,
        0xc815, 70,
        0xc8fc, 68,
        0xc9e7, 75,
        0xcc38, 33,
        0xd06c, 61,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(23);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(31);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(59);
      if ((0xac00 <= lookahead && lookahead <= 0xd7a3)) ADVANCE(60);
      END_STATE();
    case 1:
      if (lookahead == ' ') ADVANCE(15);
      END_STATE();
    case 2:
      if (lookahead == '"') ADVANCE(29);
      if (lookahead == '\\') ADVANCE(21);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(28);
      END_STATE();
    case 3:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead == '/') ADVANCE(25);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 4:
      if (lookahead == '*') ADVANCE(3);
      if (lookahead != 0) ADVANCE(4);
      END_STATE();
    case 5:
      if (lookahead == '.') ADVANCE(99);
      END_STATE();
    case 6:
      if (lookahead == '=') ADVANCE(87);
      END_STATE();
    case 7:
      if (lookahead == '=') ADVANCE(86);
      END_STATE();
    case 8:
      if (lookahead == 0xac00) ADVANCE(17);
      END_STATE();
    case 9:
      if (lookahead == 0xace0) ADVANCE(57);
      END_STATE();
    case 10:
      if (lookahead == 0xb294) ADVANCE(45);
      END_STATE();
    case 11:
      if (lookahead == 0xb294) ADVANCE(42);
      END_STATE();
    case 12:
      if (lookahead == 0xb41c) ADVANCE(43);
      END_STATE();
    case 13:
      if (lookahead == 0xba74) ADVANCE(54);
      END_STATE();
    case 14:
      if (lookahead == 0xc11c) ADVANCE(49);
      END_STATE();
    case 15:
      if (lookahead == 0xc5c6) ADVANCE(11);
      END_STATE();
    case 16:
      if (lookahead == 0xc694) ADVANCE(50);
      END_STATE();
    case 17:
      if (lookahead == 0xc694) ADVANCE(51);
      END_STATE();
    case 18:
      if (lookahead == 0xc694) ADVANCE(46);
      END_STATE();
    case 19:
      if (lookahead == 0xd2b8) ADVANCE(39);
      END_STATE();
    case 20:
      if (lookahead == 0xd574) ADVANCE(16);
      END_STATE();
    case 21:
      if (lookahead == '"' ||
          lookahead == '0' ||
          lookahead == '\\' ||
          lookahead == 'n' ||
          lookahead == 't') ADVANCE(30);
      END_STATE();
    case 22:
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(32);
      END_STATE();
    case 23:
      if (eof) ADVANCE(24);
      ADVANCE_MAP(
        '!', 6,
        '"', 27,
        '%', 92,
        '&', 93,
        '(', 103,
        ')', 104,
        '*', 90,
        '+', 88,
        ',', 101,
        '-', 89,
        '.', 102,
        '/', 91,
        '<', 97,
        '=', 7,
        '>', 98,
        '^', 95,
        '{', 105,
        '|', 94,
        '}', 106,
        '~', 96,
        0x2026, 100,
        0xac70, 79,
        0xace0, 78,
        0xadf8, 64,
        0xb118, 74,
        0xb17c, 65,
        0xb3d9, 72,
        0xb610, 62,
        0xb9cc, 73,
        0xbb38, 77,
        0xbc14, 76,
        0xbc18, 81,
        0xbc1b, 71,
        0xbd80, 80,
        0xc138, 66,
        0xc18c, 69,
        0xc544, 63,
        0xc678, 67,
        0xc815, 70,
        0xc8fc, 68,
        0xc9e7, 75,
        0xcc38, 33,
        0xd06c, 61,
      );
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(23);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(31);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(59);
      if ((0xac00 <= lookahead && lookahead <= 0xd7a3)) ADVANCE(60);
      END_STATE();
    case 24:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 25:
      ACCEPT_TOKEN(sym_comment);
      END_STATE();
    case 26:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(26);
      END_STATE();
    case 27:
      ACCEPT_TOKEN(anon_sym_DQUOTE);
      END_STATE();
    case 28:
      ACCEPT_TOKEN(aux_sym_string_token1);
      if (lookahead != 0 &&
          lookahead != '\n' &&
          lookahead != '"' &&
          lookahead != '\\') ADVANCE(28);
      END_STATE();
    case 29:
      ACCEPT_TOKEN(anon_sym_DQUOTE2);
      END_STATE();
    case 30:
      ACCEPT_TOKEN(sym_escape);
      END_STATE();
    case 31:
      ACCEPT_TOKEN(sym_number);
      if (lookahead == '.') ADVANCE(22);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(31);
      END_STATE();
    case 32:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(32);
      END_STATE();
    case 33:
      ACCEPT_TOKEN(anon_sym_ucc38);
      END_STATE();
    case 34:
      ACCEPT_TOKEN(anon_sym_uac70uc9d3);
      END_STATE();
    case 35:
      ACCEPT_TOKEN(anon_sym_uc815uc218);
      END_STATE();
    case 36:
      ACCEPT_TOKEN(anon_sym_uc18cuc218);
      END_STATE();
    case 37:
      ACCEPT_TOKEN(anon_sym_ub17cub9ac);
      END_STATE();
    case 38:
      ACCEPT_TOKEN(anon_sym_ubb38uc790);
      END_STATE();
    case 39:
      ACCEPT_TOKEN(anon_sym_ubc14uc774ud2b8);
      END_STATE();
    case 40:
      ACCEPT_TOKEN(anon_sym_uc8fcuc18c);
      END_STATE();
    case 41:
      ACCEPT_TOKEN(anon_sym_uc9e7uc740);
      END_STATE();
    case 42:
      ACCEPT_TOKEN(anon_sym_ubd80ud638uc5c6ub294);
      END_STATE();
    case 43:
      ACCEPT_TOKEN(anon_sym_uace0uc815ub41c);
      END_STATE();
    case 44:
      ACCEPT_TOKEN(anon_sym_uc678ubd80);
      END_STATE();
    case 45:
      ACCEPT_TOKEN(anon_sym_ubc18ud658ud558ub294);
      END_STATE();
    case 46:
      ACCEPT_TOKEN(anon_sym_ubc18ud658ud574uc694);
      END_STATE();
    case 47:
      ACCEPT_TOKEN(anon_sym_ubc18ud658);
      if (lookahead == 0xd558) ADVANCE(10);
      if (lookahead == 0xd574) ADVANCE(18);
      END_STATE();
    case 48:
      ACCEPT_TOKEN(anon_sym_ubc1buc544);
      END_STATE();
    case 49:
      ACCEPT_TOKEN(anon_sym_uc138uba74uc11c);
      END_STATE();
    case 50:
      ACCEPT_TOKEN(anon_sym_uadf8ub9ccud574uc694);
      END_STATE();
    case 51:
      ACCEPT_TOKEN(anon_sym_ub118uc5b4uac00uc694);
      END_STATE();
    case 52:
      ACCEPT_TOKEN(anon_sym_ud06cuae30);
      END_STATE();
    case 53:
      ACCEPT_TOKEN(anon_sym_ub9ccuc57d);
      END_STATE();
    case 54:
      ACCEPT_TOKEN(anon_sym_uc544ub2c8uba74);
      END_STATE();
    case 55:
      ACCEPT_TOKEN(anon_sym_uc544ub2cc);
      END_STATE();
    case 56:
      ACCEPT_TOKEN(anon_sym_ub3d9uc548);
      END_STATE();
    case 57:
      ACCEPT_TOKEN(anon_sym_uadf8ub9acuace0);
      END_STATE();
    case 58:
      ACCEPT_TOKEN(anon_sym_ub610ub294);
      END_STATE();
    case 59:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(59);
      END_STATE();
    case 60:
      ACCEPT_TOKEN(sym_syllable);
      END_STATE();
    case 61:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xae30) ADVANCE(52);
      END_STATE();
    case 62:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xb294) ADVANCE(58);
      END_STATE();
    case 63:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xb2c8) ADVANCE(13);
      if (lookahead == 0xb2cc) ADVANCE(55);
      END_STATE();
    case 64:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xb9ac) ADVANCE(9);
      if (lookahead == 0xb9cc) ADVANCE(20);
      END_STATE();
    case 65:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xb9ac) ADVANCE(37);
      END_STATE();
    case 66:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xba74) ADVANCE(14);
      END_STATE();
    case 67:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xbd80) ADVANCE(44);
      END_STATE();
    case 68:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc18c) ADVANCE(40);
      END_STATE();
    case 69:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc218) ADVANCE(36);
      END_STATE();
    case 70:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc218) ADVANCE(35);
      END_STATE();
    case 71:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc544) ADVANCE(48);
      END_STATE();
    case 72:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc548) ADVANCE(56);
      END_STATE();
    case 73:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc57d) ADVANCE(53);
      END_STATE();
    case 74:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc5b4) ADVANCE(8);
      END_STATE();
    case 75:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc740) ADVANCE(41);
      END_STATE();
    case 76:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc774) ADVANCE(19);
      END_STATE();
    case 77:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc790) ADVANCE(38);
      END_STATE();
    case 78:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc815) ADVANCE(12);
      END_STATE();
    case 79:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xc9d3) ADVANCE(34);
      END_STATE();
    case 80:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xd638) ADVANCE(1);
      END_STATE();
    case 81:
      ACCEPT_TOKEN(sym_syllable);
      if (lookahead == 0xd658) ADVANCE(47);
      END_STATE();
    case 82:
      ACCEPT_TOKEN(anon_sym_LT_LT);
      END_STATE();
    case 83:
      ACCEPT_TOKEN(anon_sym_GT_GT);
      END_STATE();
    case 84:
      ACCEPT_TOKEN(anon_sym_LT_EQ);
      END_STATE();
    case 85:
      ACCEPT_TOKEN(anon_sym_GT_EQ);
      END_STATE();
    case 86:
      ACCEPT_TOKEN(anon_sym_EQ_EQ);
      END_STATE();
    case 87:
      ACCEPT_TOKEN(anon_sym_BANG_EQ);
      END_STATE();
    case 88:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 89:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 90:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 91:
      ACCEPT_TOKEN(anon_sym_SLASH);
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '/') ADVANCE(26);
      END_STATE();
    case 92:
      ACCEPT_TOKEN(anon_sym_PERCENT);
      END_STATE();
    case 93:
      ACCEPT_TOKEN(anon_sym_AMP);
      END_STATE();
    case 94:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 95:
      ACCEPT_TOKEN(anon_sym_CARET);
      END_STATE();
    case 96:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 97:
      ACCEPT_TOKEN(anon_sym_LT);
      if (lookahead == '<') ADVANCE(82);
      if (lookahead == '=') ADVANCE(84);
      END_STATE();
    case 98:
      ACCEPT_TOKEN(anon_sym_GT);
      if (lookahead == '=') ADVANCE(85);
      if (lookahead == '>') ADVANCE(83);
      END_STATE();
    case 99:
      ACCEPT_TOKEN(anon_sym_DOT_DOT_DOT);
      END_STATE();
    case 100:
      ACCEPT_TOKEN(anon_sym_u2026);
      END_STATE();
    case 101:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 102:
      ACCEPT_TOKEN(anon_sym_DOT);
      if (lookahead == '.') ADVANCE(5);
      END_STATE();
    case 103:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 104:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 105:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 106:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 23},
  [2] = {.lex_state = 23},
  [3] = {.lex_state = 23},
  [4] = {.lex_state = 23},
  [5] = {.lex_state = 23},
  [6] = {.lex_state = 23},
  [7] = {.lex_state = 23},
  [8] = {.lex_state = 23},
  [9] = {.lex_state = 23},
  [10] = {.lex_state = 23},
  [11] = {.lex_state = 23},
  [12] = {.lex_state = 2},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2},
  [15] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_comment] = ACTIONS(1),
    [anon_sym_DQUOTE] = ACTIONS(1),
    [anon_sym_DQUOTE2] = ACTIONS(1),
    [sym_escape] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
    [anon_sym_ucc38] = ACTIONS(1),
    [anon_sym_uac70uc9d3] = ACTIONS(1),
    [anon_sym_uc815uc218] = ACTIONS(1),
    [anon_sym_uc18cuc218] = ACTIONS(1),
    [anon_sym_ub17cub9ac] = ACTIONS(1),
    [anon_sym_ubb38uc790] = ACTIONS(1),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(1),
    [anon_sym_uc8fcuc18c] = ACTIONS(1),
    [anon_sym_uc9e7uc740] = ACTIONS(1),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(1),
    [anon_sym_uace0uc815ub41c] = ACTIONS(1),
    [anon_sym_uc678ubd80] = ACTIONS(1),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(1),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(1),
    [anon_sym_ubc18ud658] = ACTIONS(1),
    [anon_sym_ubc1buc544] = ACTIONS(1),
    [anon_sym_uc138uba74uc11c] = ACTIONS(1),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(1),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(1),
    [anon_sym_ud06cuae30] = ACTIONS(1),
    [anon_sym_ub9ccuc57d] = ACTIONS(1),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(1),
    [anon_sym_uc544ub2cc] = ACTIONS(1),
    [anon_sym_ub3d9uc548] = ACTIONS(1),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(1),
    [anon_sym_ub610ub294] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_syllable] = ACTIONS(1),
    [anon_sym_LT_LT] = ACTIONS(1),
    [anon_sym_GT_GT] = ACTIONS(1),
    [anon_sym_LT_EQ] = ACTIONS(1),
    [anon_sym_GT_EQ] = ACTIONS(1),
    [anon_sym_EQ_EQ] = ACTIONS(1),
    [anon_sym_BANG_EQ] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_PERCENT] = ACTIONS(1),
    [anon_sym_AMP] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_CARET] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_LT] = ACTIONS(1),
    [anon_sym_GT] = ACTIONS(1),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(1),
    [anon_sym_u2026] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(15),
    [sym__token] = STATE(2),
    [sym_string] = STATE(2),
    [sym_boolean] = STATE(2),
    [sym_type] = STATE(2),
    [sym_keyword] = STATE(2),
    [sym_operator] = STATE(2),
    [sym_punctuation] = STATE(2),
    [sym_bracket] = STATE(2),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym_comment] = ACTIONS(5),
    [anon_sym_DQUOTE] = ACTIONS(7),
    [sym_number] = ACTIONS(5),
    [anon_sym_ucc38] = ACTIONS(9),
    [anon_sym_uac70uc9d3] = ACTIONS(9),
    [anon_sym_uc815uc218] = ACTIONS(11),
    [anon_sym_uc18cuc218] = ACTIONS(11),
    [anon_sym_ub17cub9ac] = ACTIONS(11),
    [anon_sym_ubb38uc790] = ACTIONS(11),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(11),
    [anon_sym_uc8fcuc18c] = ACTIONS(11),
    [anon_sym_uc9e7uc740] = ACTIONS(11),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(11),
    [anon_sym_uace0uc815ub41c] = ACTIONS(11),
    [anon_sym_uc678ubd80] = ACTIONS(13),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(13),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(13),
    [anon_sym_ubc18ud658] = ACTIONS(15),
    [anon_sym_ubc1buc544] = ACTIONS(13),
    [anon_sym_uc138uba74uc11c] = ACTIONS(13),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(13),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(13),
    [anon_sym_ud06cuae30] = ACTIONS(13),
    [anon_sym_ub9ccuc57d] = ACTIONS(13),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(13),
    [anon_sym_uc544ub2cc] = ACTIONS(13),
    [anon_sym_ub3d9uc548] = ACTIONS(13),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(13),
    [anon_sym_ub610ub294] = ACTIONS(13),
    [sym_identifier] = ACTIONS(5),
    [sym_syllable] = ACTIONS(17),
    [anon_sym_LT_LT] = ACTIONS(19),
    [anon_sym_GT_GT] = ACTIONS(19),
    [anon_sym_LT_EQ] = ACTIONS(19),
    [anon_sym_GT_EQ] = ACTIONS(19),
    [anon_sym_EQ_EQ] = ACTIONS(19),
    [anon_sym_BANG_EQ] = ACTIONS(19),
    [anon_sym_PLUS] = ACTIONS(19),
    [anon_sym_DASH] = ACTIONS(19),
    [anon_sym_STAR] = ACTIONS(19),
    [anon_sym_SLASH] = ACTIONS(21),
    [anon_sym_PERCENT] = ACTIONS(19),
    [anon_sym_AMP] = ACTIONS(19),
    [anon_sym_PIPE] = ACTIONS(19),
    [anon_sym_CARET] = ACTIONS(19),
    [anon_sym_TILDE] = ACTIONS(19),
    [anon_sym_LT] = ACTIONS(21),
    [anon_sym_GT] = ACTIONS(21),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(23),
    [anon_sym_u2026] = ACTIONS(23),
    [anon_sym_COMMA] = ACTIONS(23),
    [anon_sym_DOT] = ACTIONS(25),
    [anon_sym_LPAREN] = ACTIONS(27),
    [anon_sym_RPAREN] = ACTIONS(27),
    [anon_sym_LBRACE] = ACTIONS(27),
    [anon_sym_RBRACE] = ACTIONS(27),
  },
  [2] = {
    [sym__token] = STATE(3),
    [sym_string] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_type] = STATE(3),
    [sym_keyword] = STATE(3),
    [sym_operator] = STATE(3),
    [sym_punctuation] = STATE(3),
    [sym_bracket] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(29),
    [sym_comment] = ACTIONS(31),
    [anon_sym_DQUOTE] = ACTIONS(7),
    [sym_number] = ACTIONS(31),
    [anon_sym_ucc38] = ACTIONS(9),
    [anon_sym_uac70uc9d3] = ACTIONS(9),
    [anon_sym_uc815uc218] = ACTIONS(11),
    [anon_sym_uc18cuc218] = ACTIONS(11),
    [anon_sym_ub17cub9ac] = ACTIONS(11),
    [anon_sym_ubb38uc790] = ACTIONS(11),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(11),
    [anon_sym_uc8fcuc18c] = ACTIONS(11),
    [anon_sym_uc9e7uc740] = ACTIONS(11),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(11),
    [anon_sym_uace0uc815ub41c] = ACTIONS(11),
    [anon_sym_uc678ubd80] = ACTIONS(13),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(13),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(13),
    [anon_sym_ubc18ud658] = ACTIONS(15),
    [anon_sym_ubc1buc544] = ACTIONS(13),
    [anon_sym_uc138uba74uc11c] = ACTIONS(13),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(13),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(13),
    [anon_sym_ud06cuae30] = ACTIONS(13),
    [anon_sym_ub9ccuc57d] = ACTIONS(13),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(13),
    [anon_sym_uc544ub2cc] = ACTIONS(13),
    [anon_sym_ub3d9uc548] = ACTIONS(13),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(13),
    [anon_sym_ub610ub294] = ACTIONS(13),
    [sym_identifier] = ACTIONS(31),
    [sym_syllable] = ACTIONS(33),
    [anon_sym_LT_LT] = ACTIONS(19),
    [anon_sym_GT_GT] = ACTIONS(19),
    [anon_sym_LT_EQ] = ACTIONS(19),
    [anon_sym_GT_EQ] = ACTIONS(19),
    [anon_sym_EQ_EQ] = ACTIONS(19),
    [anon_sym_BANG_EQ] = ACTIONS(19),
    [anon_sym_PLUS] = ACTIONS(19),
    [anon_sym_DASH] = ACTIONS(19),
    [anon_sym_STAR] = ACTIONS(19),
    [anon_sym_SLASH] = ACTIONS(21),
    [anon_sym_PERCENT] = ACTIONS(19),
    [anon_sym_AMP] = ACTIONS(19),
    [anon_sym_PIPE] = ACTIONS(19),
    [anon_sym_CARET] = ACTIONS(19),
    [anon_sym_TILDE] = ACTIONS(19),
    [anon_sym_LT] = ACTIONS(21),
    [anon_sym_GT] = ACTIONS(21),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(23),
    [anon_sym_u2026] = ACTIONS(23),
    [anon_sym_COMMA] = ACTIONS(23),
    [anon_sym_DOT] = ACTIONS(25),
    [anon_sym_LPAREN] = ACTIONS(27),
    [anon_sym_RPAREN] = ACTIONS(27),
    [anon_sym_LBRACE] = ACTIONS(27),
    [anon_sym_RBRACE] = ACTIONS(27),
  },
  [3] = {
    [sym__token] = STATE(3),
    [sym_string] = STATE(3),
    [sym_boolean] = STATE(3),
    [sym_type] = STATE(3),
    [sym_keyword] = STATE(3),
    [sym_operator] = STATE(3),
    [sym_punctuation] = STATE(3),
    [sym_bracket] = STATE(3),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(35),
    [sym_comment] = ACTIONS(37),
    [anon_sym_DQUOTE] = ACTIONS(40),
    [sym_number] = ACTIONS(37),
    [anon_sym_ucc38] = ACTIONS(43),
    [anon_sym_uac70uc9d3] = ACTIONS(43),
    [anon_sym_uc815uc218] = ACTIONS(46),
    [anon_sym_uc18cuc218] = ACTIONS(46),
    [anon_sym_ub17cub9ac] = ACTIONS(46),
    [anon_sym_ubb38uc790] = ACTIONS(46),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(46),
    [anon_sym_uc8fcuc18c] = ACTIONS(46),
    [anon_sym_uc9e7uc740] = ACTIONS(46),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(46),
    [anon_sym_uace0uc815ub41c] = ACTIONS(46),
    [anon_sym_uc678ubd80] = ACTIONS(49),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(49),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(49),
    [anon_sym_ubc18ud658] = ACTIONS(52),
    [anon_sym_ubc1buc544] = ACTIONS(49),
    [anon_sym_uc138uba74uc11c] = ACTIONS(49),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(49),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(49),
    [anon_sym_ud06cuae30] = ACTIONS(49),
    [anon_sym_ub9ccuc57d] = ACTIONS(49),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(49),
    [anon_sym_uc544ub2cc] = ACTIONS(49),
    [anon_sym_ub3d9uc548] = ACTIONS(49),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(49),
    [anon_sym_ub610ub294] = ACTIONS(49),
    [sym_identifier] = ACTIONS(37),
    [sym_syllable] = ACTIONS(55),
    [anon_sym_LT_LT] = ACTIONS(58),
    [anon_sym_GT_GT] = ACTIONS(58),
    [anon_sym_LT_EQ] = ACTIONS(58),
    [anon_sym_GT_EQ] = ACTIONS(58),
    [anon_sym_EQ_EQ] = ACTIONS(58),
    [anon_sym_BANG_EQ] = ACTIONS(58),
    [anon_sym_PLUS] = ACTIONS(58),
    [anon_sym_DASH] = ACTIONS(58),
    [anon_sym_STAR] = ACTIONS(58),
    [anon_sym_SLASH] = ACTIONS(61),
    [anon_sym_PERCENT] = ACTIONS(58),
    [anon_sym_AMP] = ACTIONS(58),
    [anon_sym_PIPE] = ACTIONS(58),
    [anon_sym_CARET] = ACTIONS(58),
    [anon_sym_TILDE] = ACTIONS(58),
    [anon_sym_LT] = ACTIONS(61),
    [anon_sym_GT] = ACTIONS(61),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(64),
    [anon_sym_u2026] = ACTIONS(64),
    [anon_sym_COMMA] = ACTIONS(64),
    [anon_sym_DOT] = ACTIONS(67),
    [anon_sym_LPAREN] = ACTIONS(70),
    [anon_sym_RPAREN] = ACTIONS(70),
    [anon_sym_LBRACE] = ACTIONS(70),
    [anon_sym_RBRACE] = ACTIONS(70),
  },
  [4] = {
    [ts_builtin_sym_end] = ACTIONS(73),
    [sym_comment] = ACTIONS(73),
    [anon_sym_DQUOTE] = ACTIONS(73),
    [sym_number] = ACTIONS(73),
    [anon_sym_ucc38] = ACTIONS(73),
    [anon_sym_uac70uc9d3] = ACTIONS(73),
    [anon_sym_uc815uc218] = ACTIONS(73),
    [anon_sym_uc18cuc218] = ACTIONS(73),
    [anon_sym_ub17cub9ac] = ACTIONS(73),
    [anon_sym_ubb38uc790] = ACTIONS(73),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(73),
    [anon_sym_uc8fcuc18c] = ACTIONS(73),
    [anon_sym_uc9e7uc740] = ACTIONS(73),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(73),
    [anon_sym_uace0uc815ub41c] = ACTIONS(73),
    [anon_sym_uc678ubd80] = ACTIONS(73),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(73),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(73),
    [anon_sym_ubc18ud658] = ACTIONS(75),
    [anon_sym_ubc1buc544] = ACTIONS(73),
    [anon_sym_uc138uba74uc11c] = ACTIONS(73),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(73),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(73),
    [anon_sym_ud06cuae30] = ACTIONS(73),
    [anon_sym_ub9ccuc57d] = ACTIONS(73),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(73),
    [anon_sym_uc544ub2cc] = ACTIONS(73),
    [anon_sym_ub3d9uc548] = ACTIONS(73),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(73),
    [anon_sym_ub610ub294] = ACTIONS(73),
    [sym_identifier] = ACTIONS(73),
    [sym_syllable] = ACTIONS(75),
    [anon_sym_LT_LT] = ACTIONS(73),
    [anon_sym_GT_GT] = ACTIONS(73),
    [anon_sym_LT_EQ] = ACTIONS(73),
    [anon_sym_GT_EQ] = ACTIONS(73),
    [anon_sym_EQ_EQ] = ACTIONS(73),
    [anon_sym_BANG_EQ] = ACTIONS(73),
    [anon_sym_PLUS] = ACTIONS(73),
    [anon_sym_DASH] = ACTIONS(73),
    [anon_sym_STAR] = ACTIONS(73),
    [anon_sym_SLASH] = ACTIONS(75),
    [anon_sym_PERCENT] = ACTIONS(73),
    [anon_sym_AMP] = ACTIONS(73),
    [anon_sym_PIPE] = ACTIONS(73),
    [anon_sym_CARET] = ACTIONS(73),
    [anon_sym_TILDE] = ACTIONS(73),
    [anon_sym_LT] = ACTIONS(75),
    [anon_sym_GT] = ACTIONS(75),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(73),
    [anon_sym_u2026] = ACTIONS(73),
    [anon_sym_COMMA] = ACTIONS(73),
    [anon_sym_DOT] = ACTIONS(75),
    [anon_sym_LPAREN] = ACTIONS(73),
    [anon_sym_RPAREN] = ACTIONS(73),
    [anon_sym_LBRACE] = ACTIONS(73),
    [anon_sym_RBRACE] = ACTIONS(73),
  },
  [5] = {
    [ts_builtin_sym_end] = ACTIONS(77),
    [sym_comment] = ACTIONS(77),
    [anon_sym_DQUOTE] = ACTIONS(77),
    [sym_number] = ACTIONS(77),
    [anon_sym_ucc38] = ACTIONS(77),
    [anon_sym_uac70uc9d3] = ACTIONS(77),
    [anon_sym_uc815uc218] = ACTIONS(77),
    [anon_sym_uc18cuc218] = ACTIONS(77),
    [anon_sym_ub17cub9ac] = ACTIONS(77),
    [anon_sym_ubb38uc790] = ACTIONS(77),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(77),
    [anon_sym_uc8fcuc18c] = ACTIONS(77),
    [anon_sym_uc9e7uc740] = ACTIONS(77),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(77),
    [anon_sym_uace0uc815ub41c] = ACTIONS(77),
    [anon_sym_uc678ubd80] = ACTIONS(77),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(77),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(77),
    [anon_sym_ubc18ud658] = ACTIONS(79),
    [anon_sym_ubc1buc544] = ACTIONS(77),
    [anon_sym_uc138uba74uc11c] = ACTIONS(77),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(77),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(77),
    [anon_sym_ud06cuae30] = ACTIONS(77),
    [anon_sym_ub9ccuc57d] = ACTIONS(77),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(77),
    [anon_sym_uc544ub2cc] = ACTIONS(77),
    [anon_sym_ub3d9uc548] = ACTIONS(77),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(77),
    [anon_sym_ub610ub294] = ACTIONS(77),
    [sym_identifier] = ACTIONS(77),
    [sym_syllable] = ACTIONS(79),
    [anon_sym_LT_LT] = ACTIONS(77),
    [anon_sym_GT_GT] = ACTIONS(77),
    [anon_sym_LT_EQ] = ACTIONS(77),
    [anon_sym_GT_EQ] = ACTIONS(77),
    [anon_sym_EQ_EQ] = ACTIONS(77),
    [anon_sym_BANG_EQ] = ACTIONS(77),
    [anon_sym_PLUS] = ACTIONS(77),
    [anon_sym_DASH] = ACTIONS(77),
    [anon_sym_STAR] = ACTIONS(77),
    [anon_sym_SLASH] = ACTIONS(79),
    [anon_sym_PERCENT] = ACTIONS(77),
    [anon_sym_AMP] = ACTIONS(77),
    [anon_sym_PIPE] = ACTIONS(77),
    [anon_sym_CARET] = ACTIONS(77),
    [anon_sym_TILDE] = ACTIONS(77),
    [anon_sym_LT] = ACTIONS(79),
    [anon_sym_GT] = ACTIONS(79),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(77),
    [anon_sym_u2026] = ACTIONS(77),
    [anon_sym_COMMA] = ACTIONS(77),
    [anon_sym_DOT] = ACTIONS(79),
    [anon_sym_LPAREN] = ACTIONS(77),
    [anon_sym_RPAREN] = ACTIONS(77),
    [anon_sym_LBRACE] = ACTIONS(77),
    [anon_sym_RBRACE] = ACTIONS(77),
  },
  [6] = {
    [ts_builtin_sym_end] = ACTIONS(81),
    [sym_comment] = ACTIONS(81),
    [anon_sym_DQUOTE] = ACTIONS(81),
    [sym_number] = ACTIONS(81),
    [anon_sym_ucc38] = ACTIONS(81),
    [anon_sym_uac70uc9d3] = ACTIONS(81),
    [anon_sym_uc815uc218] = ACTIONS(81),
    [anon_sym_uc18cuc218] = ACTIONS(81),
    [anon_sym_ub17cub9ac] = ACTIONS(81),
    [anon_sym_ubb38uc790] = ACTIONS(81),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(81),
    [anon_sym_uc8fcuc18c] = ACTIONS(81),
    [anon_sym_uc9e7uc740] = ACTIONS(81),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(81),
    [anon_sym_uace0uc815ub41c] = ACTIONS(81),
    [anon_sym_uc678ubd80] = ACTIONS(81),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(81),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(81),
    [anon_sym_ubc18ud658] = ACTIONS(83),
    [anon_sym_ubc1buc544] = ACTIONS(81),
    [anon_sym_uc138uba74uc11c] = ACTIONS(81),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(81),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(81),
    [anon_sym_ud06cuae30] = ACTIONS(81),
    [anon_sym_ub9ccuc57d] = ACTIONS(81),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(81),
    [anon_sym_uc544ub2cc] = ACTIONS(81),
    [anon_sym_ub3d9uc548] = ACTIONS(81),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(81),
    [anon_sym_ub610ub294] = ACTIONS(81),
    [sym_identifier] = ACTIONS(81),
    [sym_syllable] = ACTIONS(83),
    [anon_sym_LT_LT] = ACTIONS(81),
    [anon_sym_GT_GT] = ACTIONS(81),
    [anon_sym_LT_EQ] = ACTIONS(81),
    [anon_sym_GT_EQ] = ACTIONS(81),
    [anon_sym_EQ_EQ] = ACTIONS(81),
    [anon_sym_BANG_EQ] = ACTIONS(81),
    [anon_sym_PLUS] = ACTIONS(81),
    [anon_sym_DASH] = ACTIONS(81),
    [anon_sym_STAR] = ACTIONS(81),
    [anon_sym_SLASH] = ACTIONS(83),
    [anon_sym_PERCENT] = ACTIONS(81),
    [anon_sym_AMP] = ACTIONS(81),
    [anon_sym_PIPE] = ACTIONS(81),
    [anon_sym_CARET] = ACTIONS(81),
    [anon_sym_TILDE] = ACTIONS(81),
    [anon_sym_LT] = ACTIONS(83),
    [anon_sym_GT] = ACTIONS(83),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(81),
    [anon_sym_u2026] = ACTIONS(81),
    [anon_sym_COMMA] = ACTIONS(81),
    [anon_sym_DOT] = ACTIONS(83),
    [anon_sym_LPAREN] = ACTIONS(81),
    [anon_sym_RPAREN] = ACTIONS(81),
    [anon_sym_LBRACE] = ACTIONS(81),
    [anon_sym_RBRACE] = ACTIONS(81),
  },
  [7] = {
    [ts_builtin_sym_end] = ACTIONS(85),
    [sym_comment] = ACTIONS(85),
    [anon_sym_DQUOTE] = ACTIONS(85),
    [sym_number] = ACTIONS(85),
    [anon_sym_ucc38] = ACTIONS(85),
    [anon_sym_uac70uc9d3] = ACTIONS(85),
    [anon_sym_uc815uc218] = ACTIONS(85),
    [anon_sym_uc18cuc218] = ACTIONS(85),
    [anon_sym_ub17cub9ac] = ACTIONS(85),
    [anon_sym_ubb38uc790] = ACTIONS(85),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(85),
    [anon_sym_uc8fcuc18c] = ACTIONS(85),
    [anon_sym_uc9e7uc740] = ACTIONS(85),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(85),
    [anon_sym_uace0uc815ub41c] = ACTIONS(85),
    [anon_sym_uc678ubd80] = ACTIONS(85),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(85),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(85),
    [anon_sym_ubc18ud658] = ACTIONS(87),
    [anon_sym_ubc1buc544] = ACTIONS(85),
    [anon_sym_uc138uba74uc11c] = ACTIONS(85),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(85),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(85),
    [anon_sym_ud06cuae30] = ACTIONS(85),
    [anon_sym_ub9ccuc57d] = ACTIONS(85),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(85),
    [anon_sym_uc544ub2cc] = ACTIONS(85),
    [anon_sym_ub3d9uc548] = ACTIONS(85),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(85),
    [anon_sym_ub610ub294] = ACTIONS(85),
    [sym_identifier] = ACTIONS(85),
    [sym_syllable] = ACTIONS(87),
    [anon_sym_LT_LT] = ACTIONS(85),
    [anon_sym_GT_GT] = ACTIONS(85),
    [anon_sym_LT_EQ] = ACTIONS(85),
    [anon_sym_GT_EQ] = ACTIONS(85),
    [anon_sym_EQ_EQ] = ACTIONS(85),
    [anon_sym_BANG_EQ] = ACTIONS(85),
    [anon_sym_PLUS] = ACTIONS(85),
    [anon_sym_DASH] = ACTIONS(85),
    [anon_sym_STAR] = ACTIONS(85),
    [anon_sym_SLASH] = ACTIONS(87),
    [anon_sym_PERCENT] = ACTIONS(85),
    [anon_sym_AMP] = ACTIONS(85),
    [anon_sym_PIPE] = ACTIONS(85),
    [anon_sym_CARET] = ACTIONS(85),
    [anon_sym_TILDE] = ACTIONS(85),
    [anon_sym_LT] = ACTIONS(87),
    [anon_sym_GT] = ACTIONS(87),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(85),
    [anon_sym_u2026] = ACTIONS(85),
    [anon_sym_COMMA] = ACTIONS(85),
    [anon_sym_DOT] = ACTIONS(87),
    [anon_sym_LPAREN] = ACTIONS(85),
    [anon_sym_RPAREN] = ACTIONS(85),
    [anon_sym_LBRACE] = ACTIONS(85),
    [anon_sym_RBRACE] = ACTIONS(85),
  },
  [8] = {
    [ts_builtin_sym_end] = ACTIONS(89),
    [sym_comment] = ACTIONS(89),
    [anon_sym_DQUOTE] = ACTIONS(89),
    [sym_number] = ACTIONS(89),
    [anon_sym_ucc38] = ACTIONS(89),
    [anon_sym_uac70uc9d3] = ACTIONS(89),
    [anon_sym_uc815uc218] = ACTIONS(89),
    [anon_sym_uc18cuc218] = ACTIONS(89),
    [anon_sym_ub17cub9ac] = ACTIONS(89),
    [anon_sym_ubb38uc790] = ACTIONS(89),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(89),
    [anon_sym_uc8fcuc18c] = ACTIONS(89),
    [anon_sym_uc9e7uc740] = ACTIONS(89),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(89),
    [anon_sym_uace0uc815ub41c] = ACTIONS(89),
    [anon_sym_uc678ubd80] = ACTIONS(89),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(89),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(89),
    [anon_sym_ubc18ud658] = ACTIONS(91),
    [anon_sym_ubc1buc544] = ACTIONS(89),
    [anon_sym_uc138uba74uc11c] = ACTIONS(89),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(89),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(89),
    [anon_sym_ud06cuae30] = ACTIONS(89),
    [anon_sym_ub9ccuc57d] = ACTIONS(89),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(89),
    [anon_sym_uc544ub2cc] = ACTIONS(89),
    [anon_sym_ub3d9uc548] = ACTIONS(89),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(89),
    [anon_sym_ub610ub294] = ACTIONS(89),
    [sym_identifier] = ACTIONS(89),
    [sym_syllable] = ACTIONS(91),
    [anon_sym_LT_LT] = ACTIONS(89),
    [anon_sym_GT_GT] = ACTIONS(89),
    [anon_sym_LT_EQ] = ACTIONS(89),
    [anon_sym_GT_EQ] = ACTIONS(89),
    [anon_sym_EQ_EQ] = ACTIONS(89),
    [anon_sym_BANG_EQ] = ACTIONS(89),
    [anon_sym_PLUS] = ACTIONS(89),
    [anon_sym_DASH] = ACTIONS(89),
    [anon_sym_STAR] = ACTIONS(89),
    [anon_sym_SLASH] = ACTIONS(91),
    [anon_sym_PERCENT] = ACTIONS(89),
    [anon_sym_AMP] = ACTIONS(89),
    [anon_sym_PIPE] = ACTIONS(89),
    [anon_sym_CARET] = ACTIONS(89),
    [anon_sym_TILDE] = ACTIONS(89),
    [anon_sym_LT] = ACTIONS(91),
    [anon_sym_GT] = ACTIONS(91),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(89),
    [anon_sym_u2026] = ACTIONS(89),
    [anon_sym_COMMA] = ACTIONS(89),
    [anon_sym_DOT] = ACTIONS(91),
    [anon_sym_LPAREN] = ACTIONS(89),
    [anon_sym_RPAREN] = ACTIONS(89),
    [anon_sym_LBRACE] = ACTIONS(89),
    [anon_sym_RBRACE] = ACTIONS(89),
  },
  [9] = {
    [ts_builtin_sym_end] = ACTIONS(93),
    [sym_comment] = ACTIONS(93),
    [anon_sym_DQUOTE] = ACTIONS(93),
    [sym_number] = ACTIONS(93),
    [anon_sym_ucc38] = ACTIONS(93),
    [anon_sym_uac70uc9d3] = ACTIONS(93),
    [anon_sym_uc815uc218] = ACTIONS(93),
    [anon_sym_uc18cuc218] = ACTIONS(93),
    [anon_sym_ub17cub9ac] = ACTIONS(93),
    [anon_sym_ubb38uc790] = ACTIONS(93),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(93),
    [anon_sym_uc8fcuc18c] = ACTIONS(93),
    [anon_sym_uc9e7uc740] = ACTIONS(93),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(93),
    [anon_sym_uace0uc815ub41c] = ACTIONS(93),
    [anon_sym_uc678ubd80] = ACTIONS(93),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(93),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(93),
    [anon_sym_ubc18ud658] = ACTIONS(95),
    [anon_sym_ubc1buc544] = ACTIONS(93),
    [anon_sym_uc138uba74uc11c] = ACTIONS(93),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(93),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(93),
    [anon_sym_ud06cuae30] = ACTIONS(93),
    [anon_sym_ub9ccuc57d] = ACTIONS(93),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(93),
    [anon_sym_uc544ub2cc] = ACTIONS(93),
    [anon_sym_ub3d9uc548] = ACTIONS(93),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(93),
    [anon_sym_ub610ub294] = ACTIONS(93),
    [sym_identifier] = ACTIONS(93),
    [sym_syllable] = ACTIONS(95),
    [anon_sym_LT_LT] = ACTIONS(93),
    [anon_sym_GT_GT] = ACTIONS(93),
    [anon_sym_LT_EQ] = ACTIONS(93),
    [anon_sym_GT_EQ] = ACTIONS(93),
    [anon_sym_EQ_EQ] = ACTIONS(93),
    [anon_sym_BANG_EQ] = ACTIONS(93),
    [anon_sym_PLUS] = ACTIONS(93),
    [anon_sym_DASH] = ACTIONS(93),
    [anon_sym_STAR] = ACTIONS(93),
    [anon_sym_SLASH] = ACTIONS(95),
    [anon_sym_PERCENT] = ACTIONS(93),
    [anon_sym_AMP] = ACTIONS(93),
    [anon_sym_PIPE] = ACTIONS(93),
    [anon_sym_CARET] = ACTIONS(93),
    [anon_sym_TILDE] = ACTIONS(93),
    [anon_sym_LT] = ACTIONS(95),
    [anon_sym_GT] = ACTIONS(95),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(93),
    [anon_sym_u2026] = ACTIONS(93),
    [anon_sym_COMMA] = ACTIONS(93),
    [anon_sym_DOT] = ACTIONS(95),
    [anon_sym_LPAREN] = ACTIONS(93),
    [anon_sym_RPAREN] = ACTIONS(93),
    [anon_sym_LBRACE] = ACTIONS(93),
    [anon_sym_RBRACE] = ACTIONS(93),
  },
  [10] = {
    [ts_builtin_sym_end] = ACTIONS(97),
    [sym_comment] = ACTIONS(97),
    [anon_sym_DQUOTE] = ACTIONS(97),
    [sym_number] = ACTIONS(97),
    [anon_sym_ucc38] = ACTIONS(97),
    [anon_sym_uac70uc9d3] = ACTIONS(97),
    [anon_sym_uc815uc218] = ACTIONS(97),
    [anon_sym_uc18cuc218] = ACTIONS(97),
    [anon_sym_ub17cub9ac] = ACTIONS(97),
    [anon_sym_ubb38uc790] = ACTIONS(97),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(97),
    [anon_sym_uc8fcuc18c] = ACTIONS(97),
    [anon_sym_uc9e7uc740] = ACTIONS(97),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(97),
    [anon_sym_uace0uc815ub41c] = ACTIONS(97),
    [anon_sym_uc678ubd80] = ACTIONS(97),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(97),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(97),
    [anon_sym_ubc18ud658] = ACTIONS(99),
    [anon_sym_ubc1buc544] = ACTIONS(97),
    [anon_sym_uc138uba74uc11c] = ACTIONS(97),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(97),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(97),
    [anon_sym_ud06cuae30] = ACTIONS(97),
    [anon_sym_ub9ccuc57d] = ACTIONS(97),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(97),
    [anon_sym_uc544ub2cc] = ACTIONS(97),
    [anon_sym_ub3d9uc548] = ACTIONS(97),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(97),
    [anon_sym_ub610ub294] = ACTIONS(97),
    [sym_identifier] = ACTIONS(97),
    [sym_syllable] = ACTIONS(99),
    [anon_sym_LT_LT] = ACTIONS(97),
    [anon_sym_GT_GT] = ACTIONS(97),
    [anon_sym_LT_EQ] = ACTIONS(97),
    [anon_sym_GT_EQ] = ACTIONS(97),
    [anon_sym_EQ_EQ] = ACTIONS(97),
    [anon_sym_BANG_EQ] = ACTIONS(97),
    [anon_sym_PLUS] = ACTIONS(97),
    [anon_sym_DASH] = ACTIONS(97),
    [anon_sym_STAR] = ACTIONS(97),
    [anon_sym_SLASH] = ACTIONS(99),
    [anon_sym_PERCENT] = ACTIONS(97),
    [anon_sym_AMP] = ACTIONS(97),
    [anon_sym_PIPE] = ACTIONS(97),
    [anon_sym_CARET] = ACTIONS(97),
    [anon_sym_TILDE] = ACTIONS(97),
    [anon_sym_LT] = ACTIONS(99),
    [anon_sym_GT] = ACTIONS(99),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(97),
    [anon_sym_u2026] = ACTIONS(97),
    [anon_sym_COMMA] = ACTIONS(97),
    [anon_sym_DOT] = ACTIONS(99),
    [anon_sym_LPAREN] = ACTIONS(97),
    [anon_sym_RPAREN] = ACTIONS(97),
    [anon_sym_LBRACE] = ACTIONS(97),
    [anon_sym_RBRACE] = ACTIONS(97),
  },
  [11] = {
    [ts_builtin_sym_end] = ACTIONS(101),
    [sym_comment] = ACTIONS(101),
    [anon_sym_DQUOTE] = ACTIONS(101),
    [sym_number] = ACTIONS(101),
    [anon_sym_ucc38] = ACTIONS(101),
    [anon_sym_uac70uc9d3] = ACTIONS(101),
    [anon_sym_uc815uc218] = ACTIONS(101),
    [anon_sym_uc18cuc218] = ACTIONS(101),
    [anon_sym_ub17cub9ac] = ACTIONS(101),
    [anon_sym_ubb38uc790] = ACTIONS(101),
    [anon_sym_ubc14uc774ud2b8] = ACTIONS(101),
    [anon_sym_uc8fcuc18c] = ACTIONS(101),
    [anon_sym_uc9e7uc740] = ACTIONS(101),
    [anon_sym_ubd80ud638uc5c6ub294] = ACTIONS(101),
    [anon_sym_uace0uc815ub41c] = ACTIONS(101),
    [anon_sym_uc678ubd80] = ACTIONS(101),
    [anon_sym_ubc18ud658ud558ub294] = ACTIONS(101),
    [anon_sym_ubc18ud658ud574uc694] = ACTIONS(101),
    [anon_sym_ubc18ud658] = ACTIONS(103),
    [anon_sym_ubc1buc544] = ACTIONS(101),
    [anon_sym_uc138uba74uc11c] = ACTIONS(101),
    [anon_sym_uadf8ub9ccud574uc694] = ACTIONS(101),
    [anon_sym_ub118uc5b4uac00uc694] = ACTIONS(101),
    [anon_sym_ud06cuae30] = ACTIONS(101),
    [anon_sym_ub9ccuc57d] = ACTIONS(101),
    [anon_sym_uc544ub2c8uba74] = ACTIONS(101),
    [anon_sym_uc544ub2cc] = ACTIONS(101),
    [anon_sym_ub3d9uc548] = ACTIONS(101),
    [anon_sym_uadf8ub9acuace0] = ACTIONS(101),
    [anon_sym_ub610ub294] = ACTIONS(101),
    [sym_identifier] = ACTIONS(101),
    [sym_syllable] = ACTIONS(103),
    [anon_sym_LT_LT] = ACTIONS(101),
    [anon_sym_GT_GT] = ACTIONS(101),
    [anon_sym_LT_EQ] = ACTIONS(101),
    [anon_sym_GT_EQ] = ACTIONS(101),
    [anon_sym_EQ_EQ] = ACTIONS(101),
    [anon_sym_BANG_EQ] = ACTIONS(101),
    [anon_sym_PLUS] = ACTIONS(101),
    [anon_sym_DASH] = ACTIONS(101),
    [anon_sym_STAR] = ACTIONS(101),
    [anon_sym_SLASH] = ACTIONS(103),
    [anon_sym_PERCENT] = ACTIONS(101),
    [anon_sym_AMP] = ACTIONS(101),
    [anon_sym_PIPE] = ACTIONS(101),
    [anon_sym_CARET] = ACTIONS(101),
    [anon_sym_TILDE] = ACTIONS(101),
    [anon_sym_LT] = ACTIONS(103),
    [anon_sym_GT] = ACTIONS(103),
    [anon_sym_DOT_DOT_DOT] = ACTIONS(101),
    [anon_sym_u2026] = ACTIONS(101),
    [anon_sym_COMMA] = ACTIONS(101),
    [anon_sym_DOT] = ACTIONS(103),
    [anon_sym_LPAREN] = ACTIONS(101),
    [anon_sym_RPAREN] = ACTIONS(101),
    [anon_sym_LBRACE] = ACTIONS(101),
    [anon_sym_RBRACE] = ACTIONS(101),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 3,
    ACTIONS(107), 1,
      anon_sym_DQUOTE2,
    STATE(13), 1,
      aux_sym_string_repeat1,
    ACTIONS(105), 2,
      aux_sym_string_token1,
      sym_escape,
  [11] = 3,
    ACTIONS(111), 1,
      anon_sym_DQUOTE2,
    STATE(14), 1,
      aux_sym_string_repeat1,
    ACTIONS(109), 2,
      aux_sym_string_token1,
      sym_escape,
  [22] = 3,
    ACTIONS(116), 1,
      anon_sym_DQUOTE2,
    STATE(14), 1,
      aux_sym_string_repeat1,
    ACTIONS(113), 2,
      aux_sym_string_token1,
      sym_escape,
  [33] = 1,
    ACTIONS(118), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(12)] = 0,
  [SMALL_STATE(13)] = 11,
  [SMALL_STATE(14)] = 22,
  [SMALL_STATE(15)] = 33,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(2),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [21] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [25] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [27] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [29] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [33] = {.entry = {.count = 1, .reusable = false}}, SHIFT(3),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [37] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [40] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(12),
  [43] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(4),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(5),
  [49] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(6),
  [52] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(6),
  [55] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [58] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(7),
  [61] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(7),
  [64] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(8),
  [67] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(8),
  [70] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(9),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_boolean, 1, 0, 0),
  [75] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_boolean, 1, 0, 0),
  [77] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type, 1, 0, 0),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_type, 1, 0, 0),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyword, 1, 0, 0),
  [83] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_keyword, 1, 0, 0),
  [85] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_operator, 1, 0, 0),
  [87] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_operator, 1, 0, 0),
  [89] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_punctuation, 1, 0, 0),
  [91] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_punctuation, 1, 0, 0),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_bracket, 1, 0, 0),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_bracket, 1, 0, 0),
  [97] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 2, 0, 0),
  [99] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 2, 0, 0),
  [101] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_string, 3, 0, 0),
  [103] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_string, 3, 0, 0),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [111] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [113] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_string_repeat1, 2, 0, 0), SHIFT_REPEAT(14),
  [116] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_string_repeat1, 2, 0, 0),
  [118] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_kr(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
