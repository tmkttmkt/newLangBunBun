from node import node
class ornode(list):
    toindex: int = 0
    def __init__(self,*input) -> None:
        super().__init__(input)
    def gene(self,txt,ind):
        n=0
        exenum=0
        for rnode in self:
            if not (isinstance(rnode, node) or isinstance(rnode, list)):
                rinode=rnode()
                n=self.index(rnode)
                self[n]=rinode
                num=rinode.gene(txt,ind)
            else:num=rnode.gene(txt,ind)
            if num!=-1:
                exenum=num
                self.toindex=n

            n+=1  
        return exenum if exenum!=0 else -1 
    def __repr__(self) -> str:
        txt="["
        for rnode in self:
            txt+=rnode.__class__.__name__
            txt+=","
        return txt+"]"
    def repr(self) -> str:
        return "["+self[self.toindex].repr()+"]"
    def invar(self):
        no=self[self.toindex].invar()
        if no!=None:
            return andnode(no).invar()
        else:
            return andnode(self[self.toindex])

class andnode(list):
    length: int = 0
    def __init__(self,*input) -> None:
        super().__init__(input)
    def gene(self,txt,ind):
        exenum=0
        for rnode in self:
            if not (isinstance(rnode,node) or isinstance(rnode, list)):
                rinode=rnode()
                self[self.index(rnode)]=rinode
                num=rinode.gene(txt,ind)
            else:num=rnode.gene(txt,ind)
            if num!=-1:
                exenum+=num
                ind+=num
            else:
                return -1
        return exenum
    def repr(self) -> str:
        txt="("
        for rnode in self:    
            txt+=rnode.repr()
            txt+=","
        return txt+")"
    def __repr__(self) -> str:
        txt="("
        for rnode in self:
            txt+=rnode.__class__.__name__
            txt+=","
        return txt+")"
    def __add__(self,other):
        lis=super().__add__(other)
        return andnode(*lis)
    def copy(self):
        return andnode(*super().copy())
    def invar(self):
        for rnode in self:
            no=rnode.invar()
            if no!=None:
                self[self.index(rnode)]=no
        for rnode in self[:]:
            if isinstance(rnode,andnode):
                self= andnode(*self[:self.index(rnode)])+rnode+andnode(*self[self.index(rnode)+1:])
        return self

class retnode(list):
    length: int = 0
    main:andnode
    out:andnode=andnode()
    def __init__(self,*input) -> None:
        self.main=andnode(*input)
        
        
    def gene(self,txt,ind):
        self.length=0
        exenum=0
        while True:
            new=self.main.copy()
            num=new.gene(txt,ind)
            if(num==-1):
                break
            else:
                self.out=self.out+new
                exenum+=num
                ind+=num
                self.length+=1
        return exenum
    def repr(self) -> str:
        if self.length>0:
            txt=""
            for rnode in self.out:
                txt+=rnode.repr()
                txt+="."
            return "{"+txt+"}"
        else:
            return "{}"
    def __repr__(self) -> str:
        txt="{"
        for rnode in self.main:
            txt+=rnode.__class__.__name__
            txt+=","

        return txt+"}"
    def invar(self):
        ne=self.out.invar()
        if(ne==None):
            ne=self.out
        return ne
