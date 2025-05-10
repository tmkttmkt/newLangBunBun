from node import *

class smallbrackets(endnode):
    char="("
class smallbracketsc(endnode):
    char=")"
class plus(endnode):
    char="+"
class minus(endnode):
    char="-"
class multiply(endnode):
    char="*"
class cut(endnode):
    char="/"
class end(endnode):
    char=";"
class equal(endnode):
    char="="
class digit(endnode):
    char= "0123456789"
    def value(self):
        return int(self.char[self.next_index])
class letter(endnode):
    char="abcdefghijklmnopqrstuvwxyz"
    def value(self):
        return self.char[self.next_index]