F83 = GF(83)
P = PolynomialRing(F83, 'x')
x = P.gen()

R = P.quotient(x^4 + 1, 't')
t = R.gen()

A = Matrix(R, 2, 3, [
    23 + 64*t + 11*t^2 + 45*t^3,
     5 + 38*t + 72*t^2 + 19*t^3,
    48 + 12*t + 33*t^2 + 67*t^3,
    76 + 21*t +  4*t^2 + 13*t^3,
    15 + 59*t + 28*t^2 + 50*t^3,
     9 + 42*t + 61*t^2 +  3*t^3,
])

print(A)
print()

# Перетворимо матрицю A над кільцем многочленів у матрицю над полем F83,
# її розміри стануть 8x12

M = Matrix(F83, 8, 12)

for i in range(2):
    for j in range(3):
        for k in range(4):
            p = (A[i, j] * t^k).lift()
            for l in range(4):
                M[4 * i + l, 4 * j + k] = p[l]
            

print(f"M = {M}")
print()
print(f"rank(M) = {M.rank()}")

M_reduced = M.rref()

# Дістаємо позиції вільних змінних, відкидаючи одиничні вектори
unity_vecs = M_reduced.pivots()

free = [j for j in range(M.ncols()) if j not in unity_vecs]

coefs = [[-int(M_reduced[i, j]) for j in free] for i in range(len(unity_vecs))]

solutions = list()

from itertools import product
for values in product(range(-10, 11), repeat=len(free)):
    if not any(values): # Відкидаємо одразу нульовий вектор
        continue 

    pivot_values = [
        (sum(a*b for a, b in zip(row, values)) + 41) % 83 - 41
        for row in coefs
    ]

    if all(abs(c) <= 10 for c in pivot_values):
        solution = [0] * M.ncols()
        for j, c in zip(free, values):
            solution[j] = c
        for j, c in zip(unity_vecs, pivot_values):
            solution[j] = c
        solutions.append(solution)
        


if not solutions:
    print("Рішення немає.")
else:
    for solution in solutions:
        z = vector(R, [
            sum(solution[4*j + k] * t^k for k in range(4))
        for j in range(3)
        ])
        assert A * z == vector(R, [0, 0])
        print("Коефіцієнти зі знаками:", solution)
        print("Многочлен:", z)

# Результат
"""
M = [23 38 72 19  5 64 11 45 48 16 50 71]
[64 23 38 72 38  5 64 11 12 48 16 50]
[11 64 23 38 72 38  5 64 33 12 48 16]
[45 11 64 23 19 72 38  5 67 33 12 48]
[76 70 79 62 15 33 55 24  9 80 22 41]
[21 76 70 79 59 15 33 55 42  9 80 22]
[ 4 21 76 70 28 59 15 33 61 42  9 80]
[13  4 21 76 50 28 59 15  3 61 42  9]

rank(M) = 8
Рішення немає.
"""

# Якщо підняти допуск до норми = 12, то розвʼязків вже буде багато.
