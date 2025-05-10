from nodeclass import *

with open("tl") as f:
    txt=f.read().replace("\n","")
print(txt)
p=program()
#print(p.repr())
num=p.gene(txt,0)
print(num,"*****************************************")
p.invar()
print(p.repr())
p.value()


"""
program(
    assignment(
        identifier(c,{},),
        =,
        expression[
            term[
                (
                    factor[
                        identifier(a,{},)
                    ],
                    *,
                    term[
                        (
                            factor[
                                number(1,{0.},)
                            ],
                            *,
                            term[
                                factor[
                                    number(1,{},)
                                ]
                            ],
                        )
                    ],
                )
            ]
        ],
    ),
    ;,
    {},
)
"""