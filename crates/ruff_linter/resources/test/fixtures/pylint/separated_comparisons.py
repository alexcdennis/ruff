# pylint: disable=missing-docstring, invalid-name, trailing-newlines, pointless-statement, fixme

a = 0
b = 1
c = 2
d = 3

e = 'x'
f = 'y'
g = 'z'

h = b'x'
i = b'y'
j = b'z'

k = True
l = True
m = False

n = None
o = None
p = None

q = ...
r = ...
s = ...

# triggers the lint
# numbers
a < b and b < c
b < c and b > a
a < 10 and a > 1
a > 1 and a < 10
a > 1 and a <= 10
a > 1 and a < 10 and b == 2
a > 1 and c == b and a < 10
a > 100 and c == b and a < 10
a < b and b < c
a > b and b > c
a < b and a == 1 and b < c
a < b and b < c and c == 786
a < b and b < 0 and c == 786
a < b and c == 786 and b < 0
c == 786 and b < 0 and a < b
a < b < c and c < d
b < c < d and a < b
a < b < c and a < b and b < c
a < 10 and a > 1
# strings
e < f and f < g
e < 'k' and 'k' < g
e > 'kj' and e < 'kz'
# bytes
h < i and i < j
h < b'k' and b'k' < i
h > b'kj' and h < b'kz'
# booleans
k < l and l < m
k < True and True < l
k > False and k < True
# none type
n < o and o < p
# todo: should None constants be included in the lint?
#   it is NOT included in the original pylint lint, but that
#   may not be intentional
n < None and None < o
# ellipsis type
q < r and r < s
q < ... and ... < r
# todo: should non-monotonic chains be included in the lint?
#   it is included in the original pylint lint, but that may
#   not be intentional
a < b > c and b < d

# does not trigger the lint
a < 10 and b > 1
a > 1 and b < 10
a > 1 and a == 10
a == 1 and b == 2
a > 1 and a > 10
a < 100 and a < 10
a < b and a < c
a < b and a == 1 and a < c
a < b and a < c and c == 786
a < b < c and b < d
a < b < c and a < d
b < c < d and a < c
b < c < d and a < d
a < b and b > c
# strings
e < 'k' and e < 'l'
e < 'k' and f < 'k'
a < 1 and '1' < e
# bytes
h < b'k' and h < b'l'
h < b'k' and i < b'k'
a < 1 and b'1' < h
e < 'k' and b'k' < h
# booleans
k < True and k < False
k < True and l < True
e < 'True' and True < k
h < b'True' and True < k
# none
n < None and o < None
e < 'None' and None < n
h < b'None' and None < n
# ellipsis
q < ... and r < ...
e < '...' and ... < q
h < b'...' and ... < q


