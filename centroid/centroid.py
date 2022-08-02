from typing import List

d1: List[float] = [2.1, 10.9, 15.8]
d2 = [4.6, 11.3, 23.0]
d3 = [1.8, 11.4, 19.2]
d4 = [1.7, 9.2, 17.1]
d5 = [2.1, 9.5, 19.2]
d6 = [7.8, 15.4, 22.0]
c1 = [1.8, 10.0, 16.4]
c2 = [2.0, 10.6, 19.0]

def distance(p: List[float], c: List[float]) -> float:
    return pow((p[0] - c[0]),2.0) + pow((p[1] - c[1]),2.0) + pow((p[2] - c[2]),2.0)

print('c1 distance')
print(distance(d1, c1))
print(distance(d2, c1))
print(distance(d3, c1))
print(distance(d4, c1))
print(distance(d5, c1))
print(distance(d6, c1))
print('')
print('c2 distance')
print(distance(d1, c2))
print(distance(d2, c2))
print(distance(d3, c2))
print(distance(d4, c2))
print(distance(d5, c2))
print(distance(d6, c2))
print('')

d_c1 = [d1,d4]
d_c2 = [d2,d3,d5,d6]

def update_centroid(ds: List[List[float]]) -> List[float]:
    sum_x = 0.0
    sum_y = 0.0
    sum_z = 0.0
    for d in ds:
        sum_x += d[0]
        sum_y += d[1]
        sum_z += d[2]
    ds_len = len(ds) # dynamic programming
    return [sum_x / ds_len, sum_y / ds_len, sum_z / ds_len]

c1 = update_centroid(d_c1)
c2 = update_centroid(d_c2)

print('new c1')
print(c1)
print('new c2')
print(c2)

print('c1 distance')
print(distance(d1, c1))
print(distance(d2, c1))
print(distance(d3, c1))
print(distance(d4, c1))
print(distance(d5, c1))
print(distance(d6, c1))
print('')
print('c2 distance')
print(distance(d1, c2))
print(distance(d2, c2))
print(distance(d3, c2))
print(distance(d4, c2))
print(distance(d5, c2))
print(distance(d6, c2))
print('')

d_c1 = [d1, d4, d5]
d_c2 = [d2, d3, d6]

c1 = update_centroid(d_c1)
c2 = update_centroid(d_c2)

print('new c1')
print(c1)
print('new c2')
print(c2)

print('c1 distance')
print(distance(d1, c1))
print(distance(d2, c1))
print(distance(d3, c1))
print(distance(d4, c1))
print(distance(d5, c1))
print(distance(d6, c1))
print('')
print('c2 distance')
print(distance(d1, c2))
print(distance(d2, c2))
print(distance(d3, c2))
print(distance(d4, c2))
print(distance(d5, c2))
print(distance(d6, c2))
print('')

d_c1 = [d1, d3, d4, d5]
d_c2 = [d2, d6]
c1 = update_centroid(d_c1)
c2 = update_centroid(d_c2)
print('new c1')
print(c1)
print('new c2')
print(c2)

print('')
print('distance for new value')
d7 = [3.2,13.3,24.9]
print(distance(d7,c1))
print(distance(d7,c2))
