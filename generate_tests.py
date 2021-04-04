from random import randint, choice, seed
from string import ascii_letters

seed(100)

f1 = open("tmp/hugeout.txt", "w")
f2 = open("tmp/hugein.txt", "w")

for i in range(100_000):
    S = choice(ascii_letters) * randint(0, 500)
    f1.write(S+"\n")
    f2.write(S+' '*randint(0, 100)+"\n")

f2.write('\n'*randint(0, 30))
f1.close()
f2.close()
