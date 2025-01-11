import plane_partitions
from pprint import pprint

print(plane_partitions.version())
print()
print(plane_partitions.sspp_tp_tspp([[0, 0], [0, 0]], 2))

print(plane_partitions.sspp_tp_tspp([[1, 0], [0, 0]], 2))

print(plane_partitions.sspp_tp_tspp([[2, 0], [0, 0]], 2))

print(plane_partitions.sspp_tp_tspp([[2, 1], [0, 0]], 2))

print(plane_partitions.sspp_tp_tspp([[2, 1], [1, 0]], 2))

print()

print(plane_partitions.to_tikz_diagram([[2, 1], [1, 0]], 2))

print()

part = [[0,0,0],[0,0,0],[0,0,0]]
pprint(plane_partitions.rowmotion_orbit(part, 3))
print()
for i in range(8):
    print(part, plane_partitions.complement(part, 3))
    part = plane_partitions.rowmotion(part, 3)

print()

pprint([[0, 1],[0,0]])
print("Is Partition:", plane_partitions.is_plane_partition([[0, 1],[0,0]], 2))

print()

part = [[13,13,13,13,13,13,13,11,11,9,9,7,7],[13,13,13,13,13,12,12,10,10,9,7,7,5],[13,13,13,13,13,11,10,10,8,8,6,5,5],[13,13,13,12,12,11,9,9,8,6,6,5,3],[13,13,13,12,10,10,8,7,6,6,4,4,3],[13,12,11,11,10,8,8,7,5,5,4,2,1],[13,12,10,9,8,8,6,6,4,3,2,2,1],[11,10,10,9,7,7,6,4,4,3,1,0,0],[11,10,8,8,6,5,4,4,2,2,1,0,0],[9,9,8,6,6,5,3,3,2,0,0,0,0],[9,7,6,6,4,4,2,1,1,0,0,0,0],[7,7,5,5,4,2,2,0,0,0,0,0,0],[7,5,5,3,3,1,1,0,0,0,0,0,0]]
print("Complement = Rowmotion:", plane_partitions.complement(part, 13) == plane_partitions.rowmotion(part, 13))
part = [[5,5,5,5,5],[5,5,5,4,4],[3,3,3,2,1],[3,1,0,0,0],[2,0,0,0,0]]
for i in range(plane_partitions.rowmotion_orbit_length(part, 5)):
    part = plane_partitions.rowmotion(part, 5)
    print(plane_partitions.to_tikz_diagram(part, 5))
