from node import *
from oltmanode import *
from endclass import *
from dataspace import *
o=ornode
a=andnode
e=retnode

class identifier(node):
    def __init__(self):
        self.nodes = a(letter, e(o(letter, digit)))
    def value(self):
        txt=""
        for vd in self.nodes:
            txt+=vd.repr()
        return identifierlsit[txt]
    def variable(self):
        txt=""
        for vd in self.nodes:
            txt+=vd.repr()
        return txt

class number(node):
    def __init__(self):
        self.nodes = a(digit, e(digit,))
    def value(self):
        value=0
        for vd in self.nodes:
            value*=10
            value+=vd.value()
        return value

class factor(node):
    def __init__(self):
        self.nodes = o(number, identifier, a(smallbrackets, expression, smallbracketsc))
    def value(self):
        if(type(self.nodes[0])==number):
            return self.nodes[0].value()
        elif(type(self.nodes[0])==identifier):
            return self.nodes[0].value()
        elif(type(self.nodes[1])==expression):
            return self.nodes[1].value()

class term(node):
    def __init__(self):
        self.nodes = o(factor, a(factor, multiply, term), a(factor, cut, term))
    def value(self):
        if(len(self.nodes)==3):
            if(type(self.nodes[1])==multiply):
                return self.nodes[0].value()*self.nodes[2].value()
            if(type(self.nodes[1])==cut):
                return self.nodes[0].value()/self.nodes[2].value()
        elif(len(self.nodes)==1):
            return self.nodes[0].value()

class expression(node):
    def __init__(self):
        self.nodes = o(term, a(term, plus, expression), a(term, minus, expression))
    def value(self):
        if(len(self.nodes)==3):
            if(type(self.nodes[1])==plus):
                return self.nodes[0].value()+self.nodes[2].value()
            if(type(self.nodes[1])==minus):
                return self.nodes[0].value()-self.nodes[2].value()
        elif(len(self.nodes)==1):
            return self.nodes[0].value()

class assignment(node):
    def __init__(self):
        self.nodes = o(expression,a(identifier, equal, expression))
    def value(self):
        if(len(self.nodes)==3):
            identifierlsit[self.nodes[0].variable()]=self.nodes[2].value()
        elif(len(self.nodes)==1):
            return self.nodes[0].value()
class program(node):
    def __init__(self):
        self.nodes = a(assignment, end, e(assignment, end))
    def value(self):
        for rnode in self.nodes:
            if(type(rnode)==assignment):
                if(rnode.value()!=None):
                    print(rnode.value())