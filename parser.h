/**
 * parser.h
 * ---------
 * Keel parser
 */

#pragma once
#include "stdint.h"

/**
 * Basically, writing a LL parser is easy.
 * 
 * X = A B C ;
 * 
 * This can be coded as:
 * 
 *  subroutine P()
 *    if ~(Q())
 *       { 
 *         if ~(R()) return false;
 *          return true;
 *       }
 *    return true;
 *  end P;
 */

/**
 * For list forming rules, left recursive.
 * 
 * L = A | L A ;
 * 
 * Code this as:
 * 
 *  subroutine L()
 *    if ~(A()) then return false;
 *    while (A()) do // loop
 *    return true;
 *  end L;
 */

typedef enum CharacterCodes {
    NULLCHARACTER = 0,
    MAXASCIICHARACTER = 0x7F,

    LINEFEED = 0x0A, // \n
    CARRIAGERETURN = 0x0D, // \r
    SPACE = 0x20,
    UNDERSCORE = 0x5f,
    DOLLARSIGN = 0x24,

    AMPERSAND = 0x26, // &
    ASTERISK = 0x2A, // *
    AT = 0x40, // @
    BACKSLASH = 0x5C,
    BACKTICK = 0x60, // `
    BAR = 0x7C, // |
    CARET = 0x5E, // ^
    CLOSEBRACE = 0x7D, // }
    CLOSEBRACKET = 0x5D, // ]
    CLOSEPAREN = 0x29, // )
    COLON = 0x3A, // :
    COMMA = 0x2C, // ,
    DOT = 0x2E, // .
    DOUBLEQUOTE = 0x22, // "
    EQUALS = 0x3D, // =
    EXCLAMATION = 0x21, // !
    GREATERTHAN = 0x3E, // >
    HASH = 0x23, // #
    LESSTHAN = 0x3C, // <
    MINUS = 0x2D, // -
    OPENBRACE = 0x7B, // {
    OPENBRACKET = 0x5B, // [
    OPENPAREN = 0x28, // (
    PERCENT = 0x25, // %
    PLUS = 0x2B, // +
    QUESTION = 0x3F, // ?
    SEMICOLON = 0x3B, // ;
    SINGLEQUOTE = 0x27, // '
    SLASH = 0x2F, // /
    TILDE = 0x7E, // ~

    BACKSPACE = 0x08, // \b
    FORMFEED = 0x0C, // \f
    BYTEORDERMARK = 0xFEFF,
    TAB = 0x09, // \t
    VERTICALTAB = 0x0B, // \v
} CharacterCodes;

typedef enum Token {
    EndOfFile,
    Newline,
    Whitespace,

    Comment,

    StringLiteral,
    CharLiteral,
    RawStringLiteral,

    DataLiteral,
    LetLiteral,

    IfStatement,
    ElseStatement,

    Identifier,
    Unknown,
} Token;

/**
 * Source Code navigation
 * ----------------------
 * for parsing error messages
 */
typedef struct tokenRange {
    uint8_t tokenType;
    uint32_t start;
    uint32_t end;
} tokenRange;

typedef struct srcFile {
    uint8_t* filepath;
    uint8_t* buf;
    uint32_t len;
} srcFile;

typedef struct srcScanner {
    srcFile* src;
    uint32_t pos;
    uint32_t line;
    uint32_t col;
    tokenRange* token;
} srcScanner;

void new_scanner(srcScanner* scanner, srcFile* source);
Token scan(srcScanner* scanner);
