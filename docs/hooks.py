import sys
import os
import pygments.lexers

sys.path.append(os.path.abspath(os.path.dirname(__file__) + '/..'))
from kyro_lexer import KyroLexer

original_get_lexer_by_name = pygments.lexers.get_lexer_by_name

def patched_get_lexer_by_name(alias, *args, **kwargs):
    if alias.lower() == 'kyro':
        return KyroLexer(*args, **kwargs)
    return original_get_lexer_by_name(alias, *args, **kwargs)

pygments.lexers.get_lexer_by_name = patched_get_lexer_by_name

def on_config(config, **kwargs):
    return config