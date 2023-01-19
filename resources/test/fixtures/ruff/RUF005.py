foo = [4, 5, 6]
bar = [1, 2, 3] + foo
quux = (7, 8, 9) + bar
spam = quux + (10, 11, 12)
eggs = spam + [13, 14, 15]
# TODO: this results in very strange (buggy) results:
#  chain = ['a', 'b', 'c'] + eggs + ('yes', 'no', 'pants') + quux
