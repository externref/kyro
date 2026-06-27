from pygments.lexer import RegexLexer, words
from pygments.token import Comment, Keyword, Name, Number, Operator, String, Text

class KyroLexer(RegexLexer):
    name = 'kyro'
    aliases = ['kyro']
    filenames = ['*.kyro']

    tokens = {
        'root': [
            (r'\s+', Text),
            (r'//.*$', Comment.Single),
            (r'/\*[\s\S]*?\*/', Comment.Multiline),
            (words((
                'var', 'fn', 'class', 'use', 'if', 'else', 'for', 'in', 'while',
                'echo', 'return', 'super', 'this', 'true', 'false', 'nil',
                'try', 'catch', 'throw', 'break', 'continue'
            ), suffix=r'\b'), Keyword),
            (words((
                'Exception', 'ValueError', 'AttributeError', 'TypeError', 'IndexError',
                'List', 'Dict', 'Number', 'String', 'Bool', 'Nil', 'Callable', 'Class'
            ), suffix=r'\b'), Name.Class),
            (r'"[^"]*"', String),
            (r'\'[^\']*\'', String),
            (r'\b\d+(\.\d+)?\b', Number),
            (r'[a-zA-Z_][a-zA-Z0-9_]*', Name),
            (r'[+\-*/%=<>!&|^~]+', Operator),
            (r'[{}()\[\]:;,.]', Text),
        ]
    }