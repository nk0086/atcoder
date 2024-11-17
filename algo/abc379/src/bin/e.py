def main():
    import sys

    n = int(sys.stdin.readline())
    s = sys.stdin.readline().strip()

    total_sum = 0
    prev = 0

    for i, ch in enumerate(s):
        num = int(ch)
        prev = prev * 10 + num * (i + 1)
        total_sum += prev

    print(total_sum)

if __name__ == "__main__":
    main() 
