#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>

%%{
    machine keel;

    openParen  = '(';
    closeParen = ')';
    openBracket = '[';
    closeBracket = ']';
    openBrace = '{';
    closeBrace = '}';
    plus = '+';
    minus = '-';
    asterisk = '*';
    slash = '/';
    backslash = '\\';
    colon     = ':';
    equal     = '=';
    fatArrow  = '=>';
    leftArrow = '->';
    rightArrow = '<-';

    newline = '\n' @{curline ++;};

    integer    = ('+' | '-')?[0-9]+;
    float      = ('+' | '-')?[0-9]+'.'[0-9]+;

    main := |*

        # Alpha numberic characters or underscore
        alnum_u = alnum | '_';

        # Alpha characters, underscore or apostrophe
        alpha_u_apos = alpha | '_' | '\'';

	    # Whitespace is standard ws, newlines and control codes.
        any_count_line = any | newline;
        any_count_line - 0x21..0x7e;

        # Comments - TODO: collect comments and compile into docs
        '--' [^\n]* newline;

        # Single Quote
        CharLit = [^'\\] | newline | ( '\\' . any_count_line );
        '\'' . CharLit* . '\'' {

        }

        # Double Quote
        StringLit = [^"\\] | newline | ( '\\' . any_count_line );
        '"' . StringLit* . '"' {

        }
        
    *|;

}%%

%% write data nofinal;

void scanner() {
    
}
