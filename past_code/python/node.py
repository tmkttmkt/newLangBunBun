class node:
    nodes=None
    
    def repr(self) -> str:
        return self.__class__.__name__+self.nodes.repr()
    def __repr__(self) -> str:
        return self.__class__.__name__+"="+self.nodes.__repr__()
    def gene(self,txt,ind):
        return self.nodes.gene(txt,ind)
    def invar(self):
        no=self.nodes.invar()
        if no!=None:
            self.nodes=no
    def value(self):
        return None



class endnode(node):
    char: str
    next_index: int = 0
    def gene(self, text:str,ind):
        if len(text)<=ind or ind<0:
            return -1
        elif text[ind] in self.char:
            self.next_index = self.char.index(text[ind])
            return 1
        else:
            return -1
    def repr(self) -> str:
        return self.char[self.next_index]
    def __repr__(self) -> str:
        return self.__class__.__name__
    def invar(self):
            pass

# Example input
"a=a*2;"