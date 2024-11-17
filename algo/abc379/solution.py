import sys
import bisect

def main():
    input = sys.stdin.readline
    Q = int(sys.stdin.readline())
    queries = []
    for _ in range(Q):
        parts = sys.stdin.readline().split()
        queries.append(parts)

    plants = []
    global_addition = 0
    result = []

    for q in queries:
        if q[0] == '1':
            # クエリ1: 植物を植える
            # 植物の相対高さを登録
            bisect.insort(plants, -global_addition)
        elif q[0] == '2':
            # クエリ2: T日待つ
            T = int(q[1])
            global_addition += T
        elif q[0] == '3':
            # クエリ3: 高さがH以上の植物を収穫
            H = int(q[1])
            # 必要な相対高さは H - global_addition
            target = H - global_addition
            # bisect_leftでtargetを超える最初の位置を見つける
            idx = bisect.bisect_left(plants, target)
            harvested = len(plants) - idx
            result.append(str(harvested))
            # 植物を収穫（削除）
            plants = plants[:idx]

    print('\n'.join(result))

if __name__ == "__main__":
    main() 
