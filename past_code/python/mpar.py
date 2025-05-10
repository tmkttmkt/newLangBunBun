txt="program(assignment(identifier(a,),=,expression(term(factor(number(1,0,),),*,term(factor(number(2,3,),),),),),),;,assignment(identifier(b,),=,expression(term(factor(identifier(a,),),),+,expression(term(factor(number(2,3,),),),),),),;,assignment(identifier(c,),=,expression(term(factor(identifier(a,),),),+,expression(term(factor(identifier(b,),),),),),),;,)"
i=0
ind=0
while len(txt)>ind:
    if(txt[ind]=="("):
        i+=1
        t="\n"+"  "*i
        txt=txt[:ind]+t+txt[ind:]
        ind+=len(t)
    if(txt[ind]==","):
        t="\n"+"  "*i
        txt=txt[:ind]+t+txt[ind:]
        ind+=len(t)
    if(txt[ind]==")"):
        i-=1
        t="\n"+"  "*i
        txt=txt[:ind]+t+txt[ind:]
        ind+=len(t)
    ind+=1

print(txt)
        